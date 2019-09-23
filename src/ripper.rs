use futures::stream::StreamExt;
use image::ImageBuffer;
use tokio::codec::FramedRead;
use tokio_process::Command;

use crate::codec::{FrameBuffer, VideoFrameCodec};
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
    let duration = f64::floor(duration - 1.0);
    let video_dimensions = &get_video_dimensions(self.input_path)?;
    let aspect_preserved_width = video_dimensions.height * 3;
    let barcode_dimensions = Dimensions::new(aspect_preserved_width, video_dimensions.height);
    let pixels = self
      .spawn_ffmpeg_ripper(duration, &video_dimensions, &barcode_dimensions)
      .await?;
    self.save_barcode(pixels, &barcode_dimensions)?;
    Ok(())
  }

  pub async fn spawn_ffmpeg_ripper(
    &self,
    duration: f64,
    video_dimensions: &Dimensions,
    barcode_dimensions: &Dimensions,
  ) -> Result<Vec<Pixel>, Box<dyn std::error::Error>> {
    let mut ss: f64 = 0.0;
    let fps_dividend = duration / barcode_dimensions.width as f64;
    let total_pixels = barcode_dimensions.width * barcode_dimensions.height;
    let mut pixels = Vec::with_capacity((total_pixels) as usize);

    while ss < duration {
      let output = Command::new("ffmpeg")
        .args(&[
          "-ss",
          &ss.to_string(),
          "-i",
          self.input_path,
          "-frames:v",
          "1",
          "-f",
          "image2pipe",
          "-pix_fmt",
          "rgb24",
          "-vcodec",
          "rawvideo",
          "-an",
          "-",
        ])
        .output();

      let output = output.await?;
      assert!(output.status.success());
      let buf_vec = output.stdout;
      let frame_buffer =
        FrameBuffer::from_raw(video_dimensions.width, video_dimensions.height, buf_vec).unwrap();
      let mut average_pixels = match self.is_simple {
        true => get_simple_col_average_pixels(frame_buffer, video_dimensions),
        false => get_blended_col_average_pixels(frame_buffer, video_dimensions),
      };
      pixels.append(&mut average_pixels);

      ss += fps_dividend;
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
