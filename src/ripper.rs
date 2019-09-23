use futures::stream::StreamExt;
use image::ImageBuffer;
use std::process::{Command, Stdio};
use tokio::codec::FramedRead;
use tokio_process::CommandExt;

use crate::codec::VideoFrameCodec;
use crate::ffmpeg::{get_video_dimensions, get_video_duration};
use crate::pixel::{get_blended_col_average_pixels, get_simple_col_average_pixels, Pixel};

pub const WIDTH: f64 = 1600.0;
pub const HEIGHT: f64 = 500.0;

#[derive(Copy, Clone)]
pub struct Dimensions {
  pub width: u32,
  pub height: u32,
}

impl Dimensions {
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }
}

pub struct FrameRipper<'a> {
  input_path: &'a str,
  output_path: &'a str,
  is_simple: bool,
}

impl<'a> FrameRipper<'a> {
  pub fn new(input_path: &'a str, output_path: &'a str, is_simple: bool) -> Self {
    Self {
      input_path,
      output_path,
      is_simple,
    }
  }

  pub async fn rip(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let duration = get_video_duration(self.input_path)?;
    let dimensions = &get_video_dimensions(self.input_path)?;
    let aspect_preserved_width = dimensions.height * 3;
    let img_dimensions = Dimensions::new(aspect_preserved_width, dimensions.height);
    let fps_dividend = duration / img_dimensions.width as f64;
    let average_pixels = self
      .spawn_ffmpeg_ripper(fps_dividend, &img_dimensions)
      .await?;
    self.save_barcode(average_pixels, &img_dimensions)?;
    Ok(())
  }

  pub async fn spawn_ffmpeg_ripper(
    &self,
    fps_dividend: f64,
    dimensions: &Dimensions,
  ) -> Result<Vec<Pixel>, Box<dyn std::error::Error>> {
    let dividend_str = fps_dividend.to_string();
    let fps_arg = &format!("fps=1/{}", dividend_str)[..];
    let total_pixels = dimensions.width * dimensions.height;

    let mut child = Command::new("ffmpeg")
      .args(&[
        "-i",
        self.input_path,
        "-vf",
        fps_arg,
        "-f",
        "image2pipe",
        "-pix_fmt",
        "rgb24",
        "-vcodec",
        "rawvideo",
        "-",
      ])
      .stdout(Stdio::piped())
      .spawn_async()
      .expect("failed to spawn ffmpeg for ripping");
    let stdout = child
      .stdout()
      .take()
      .expect("child did not have a handle to stdout");
    let mut reader = FramedRead::new(
      stdout,
      VideoFrameCodec::new(dimensions.width, dimensions.height),
    );

    tokio::spawn(async {
      let status = child.await.expect("child process encountered an error");

      println!("child status was: {}", status);
    });

    let mut pixels = Vec::with_capacity((total_pixels) as usize);

    while let Some(frame) = reader.next().await {
      if pixels.len() < (total_pixels) as usize {
        let mut average_pixels = match self.is_simple {
          true => get_simple_col_average_pixels(frame.unwrap(), dimensions),
          false => get_blended_col_average_pixels(frame.unwrap(), dimensions),
        };
        pixels.append(&mut average_pixels);
      }
    }

    Ok(pixels)
  }

  fn save_barcode(
    &self,
    average_pixels: Vec<Pixel>,
    dimensions: &Dimensions,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageBuffer::from_fn(dimensions.width, dimensions.height as u32, |row, col| {
      average_pixels[(row * dimensions.height as u32 + col) as usize]
    });
    img.save(self.output_path)?;
    Ok(())
  }
}
