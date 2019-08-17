#![feature(async_await)]

use futures::stream::StreamExt;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use tokio::codec::{FramedRead, LinesCodec};
use tokio_process::CommandExt;

pub mod codec;

const WIDTH: f64 = 1500.0;
const HEIGHT: f64 = 500.0;

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
  fps_dividend: Option<f64>,
  dimensions: Option<Dimensions>,
}

impl<'a> FrameRipper<'a> {
  pub fn new(input_path: &'a str) -> Self {
    Self {
      input_path,
      fps_dividend: None,
      dimensions: None,
    }
  }

  pub async fn rip(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let duration = self.get_video_duration().unwrap();
    self.fps_dividend = Some(duration / (WIDTH + 10.0));
    self.dimensions = Some(self.get_video_dimensions().unwrap());
    self.spawn_ffmpeg_ripper().await?;
    Ok(())
  }

  fn get_video_duration(&self) -> Result<f64, &'static str> {
    let mut cmd = Command::new("ffprobe")
      .args(&[
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
        self.input_path,
      ])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();
    let mut duration: Option<f64> = None;
    // This feels like the wrong way to handle this.
    // I'm only expecting one line of output ever, though
    // I know Rust can't know that. Any better approach?
    for line in stdout_lines {
      if let Ok(val) = line {
        duration = Some(val.parse::<f64>().expect("Couldn't parse duration"));
      }
    }

    cmd.wait().unwrap();

    match duration {
      Some(val) => Ok(val),
      None => Err("No duration found"),
    }
  }

  fn get_video_dimensions(&self) -> Result<Dimensions, &'static str> {
    let mut cmd = Command::new("ffprobe")
      .args(&[
        "-v",
        "error",
        "-show_entries",
        "stream=width,height",
        "-of",
        "csv=p=0:s=x",
        self.input_path,
      ])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();
    let mut dimensions: Option<Dimensions> = None;
    for line in stdout_lines {
      match dimensions {
        None => {
          if let Ok(val) = line {
            let vec_dimensions = val
              .split("x")
              .map(|dim| dim.parse::<u32>().unwrap())
              .collect::<Vec<_>>();
            dimensions = Some(Dimensions::new(vec_dimensions[0], vec_dimensions[1]));
          }
        }
        _ => {}
      }
    }

    cmd.wait().unwrap();

    match dimensions {
      Some(dims) => Ok(dims),
      _ => Err("No dimensions found"),
    }
  }

  async fn spawn_ffmpeg_ripper(&self) -> Result<(), Box<dyn std::error::Error>> {
    let dividend_str: &str = &self.fps_dividend.unwrap().to_string();
    let fps_arg = &format!("fps=1/{}", dividend_str)[..];

    let mut child = Command::new("ffmpeg")
      .args(&[
        "-i",
        self.input_path,
        "-vf",
        fps_arg,
        "-f",
        "rawvideo",
        "-",
        "-hide_banner",
        "-pix_fmt",
        "+rgb24",
      ])
      .stdout(Stdio::piped())
      .spawn_async()
      .expect("failed to spawn ffmpeg for ripping");
    let stdout = child
      .stdout()
      .take()
      .expect("child did not have a handle to stdout");
    let dims = self.dimensions.unwrap();
    let mut reader = FramedRead::new(stdout, codec::VideoFrameCodec::new(dims.width, dims.height));

    tokio::spawn(async {
      let status = child.await.expect("child process encountered an error");

      println!("child status was: {}", status);
    });

    let mut counter = 0;
    while let Some(frame) = reader.next().await {
      frame.unwrap().save(format!("frames/{}.jpg", counter))?;
      counter += 1;
    }

    Ok(())
  }
}
