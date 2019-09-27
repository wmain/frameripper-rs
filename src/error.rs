use getopts::Fail;
use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
  // shared
  #[snafu(display("Failed to spawn ffmpeg. {}", source))]
  CommandSpawnError { source: io::Error },
  //mod config
  #[snafu(display("Arguments parsing failed. {:?}", source))]
  ParseFailureError { source: Fail },
  // mod ffmpeg
  #[snafu(display("Failed to read line from ffmpeg stdout. {}", source))]
  FFMPEGLineReadError { source: io::Error },
  #[snafu(display("Could not parse video dimensions. {}", source))]
  ParseDimensionsError { source: std::num::ParseIntError },
  #[snafu(display("Could not parse video duration. {}", source))]
  ParseDurationError { source: std::num::ParseFloatError },
  // mod ripper
  #[snafu(display("ffmpeg could not provide a handle to stdout."))]
  StdoutHandleError,
  #[snafu(display("ffmepg encountered an error. {}", source))]
  FfmpegError { source: io::Error },
  #[snafu(display("Could not create a FrameBuffer from ffmepg byte stream."))]
  FrameBufferError,
  #[snafu(display("Barcode failed to save. {}", source))]
  BarcodeSaveError { source: io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
