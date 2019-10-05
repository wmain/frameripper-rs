use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
  // shared
  #[snafu(display("Failed to spawn ffmpeg. {}", source))]
  CommandSpawnError { source: io::Error },
  //mod config
  #[snafu(display("No input path option found. Input path is required"))]
  InputPathOptionMissingError,
  #[snafu(display("No output path option found. Input path is required"))]
  OutputPathOptionMissingError,
  // mod ffmpeg
  #[snafu(display("FFMPEG failed to exit properly. {}", source))]
  FFMPEGExitError { source: io::Error },
  #[snafu(display("Failed to read dimensions  from ffmpeg stdout."))]
  FFMPEGLineReadError,
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
