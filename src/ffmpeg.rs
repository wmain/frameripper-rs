use snafu::{OptionExt, ResultExt};
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
    let duration = stdout_reader
        .lines()
        .next()
        .context(FFMPEGLineReadError)?
        .unwrap()
        .parse::<f64>()
        .context(ParseDurationError)?;

    cmd.wait().context(FFMPEGExitError)?;

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
    let dimensions = stdout_reader
        .lines()
        .next()
        .context(FFMPEGLineReadError)?
        .unwrap()
        .split('x')
        .map(|dim| dim.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let dimensions = Dimensions::new(dimensions[0], dimensions[1]);

    cmd.wait().context(FFMPEGExitError)?;

    Ok(dimensions)
}
