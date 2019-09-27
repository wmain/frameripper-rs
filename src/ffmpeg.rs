use snafu::ResultExt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::error::*;
use crate::ripper::Dimensions;

pub fn get_video_duration(input_video_path: &str) -> Result<f64> {
  let mut cmd = Command::new("ffprobe")
    .args(&[
      "-v",
      "error",
      "-show_entries",
      "format=duration",
      "-of",
      "default=noprint_wrappers=1:nokey=1",
      input_video_path,
    ])
    .stdout(Stdio::piped())
    .spawn()
    .context(CommandSpawnError)?;

  let stdout = cmd.stdout.as_mut().unwrap();
  let stdout_reader = BufReader::new(stdout);
  let mut stdout_lines = stdout_reader.lines();
  let mut duration_line: Option<String> = None;
  while duration_line.is_none() {
    let next_line = stdout_lines.next();
    if let Some(result) = next_line {
      duration_line = Some(result.context(FFMPEGLineReadError)?);
    }
  }
  let duration = duration_line
    .unwrap()
    .parse::<f64>()
    .context(ParseDurationError)?;

  cmd.wait().unwrap();

  Ok(duration)
}

pub fn get_video_dimensions(input_video_path: &str) -> Result<Dimensions> {
  let mut cmd = Command::new("ffprobe")
    .args(&[
      "-v",
      "error",
      "-show_entries",
      "stream=width,height",
      "-of",
      "csv=p=0:s=x",
      input_video_path,
    ])
    .stdout(Stdio::piped())
    .spawn()
    .context(CommandSpawnError)?;

  let stdout = cmd.stdout.as_mut().unwrap();
  let stdout_reader = BufReader::new(stdout);
  let mut stdout_lines = stdout_reader.lines();
  let mut dimensions_line: Option<String> = None;
  while dimensions_line.is_none() {
    let next_line = stdout_lines.next();
    if let Some(result) = next_line {
      dimensions_line = Some(result.context(FFMPEGLineReadError)?);
    }
  }
  let vec_dimensions = dimensions_line
    .unwrap()
    .split('x')
    // TODO: Why doesn't .context work on parse here?
    .map(|dim| dim.parse::<u32>().unwrap())
    .collect::<Vec<_>>();

  let dimensions = Dimensions::new(vec_dimensions[0], vec_dimensions[1]);

  cmd.wait().unwrap();

  Ok(dimensions)
}
