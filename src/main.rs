#![feature(async_await)]

use std::process;

mod codec;
mod config;
mod error;
mod ffmpeg;
mod pixel;
mod progress;
mod ripper;

use crate::config::Config;
use crate::ripper::FrameRipper;

#[tokio::main]
async fn main() {
    let config = Config::new();

    let mut ripper = FrameRipper::new(
        &config.input_file_path,
        &config.output_file_path,
        config.is_simple,
    );

    ripper.rip().await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
