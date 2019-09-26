#![feature(async_await)]
extern crate getopts;

use std::env;
use std::process;

mod codec;
mod config;
mod ffmpeg;
mod pixel;
mod progress;
mod ripper;

use crate::config::Config;
use crate::ripper::FrameRipper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut ripper = FrameRipper::new(
        &config.input_file_path,
        &config.output_file_path,
        config.is_simple,
    );
    ripper.rip().await?;
    Ok(())
}
