#![feature(async_await)]
extern crate getopts;

use frame_ripper::FrameRipper;
use getopts::Options;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.reqopt("i", "input", "The input video file", "");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let input = match matches.opt_str("i") {
        Some(i) => i,
        None => {
            eprintln!("Required input option not provided");
            process::exit(1);
        }
    };

    let mut ripper = FrameRipper::new(&input);
    ripper.rip().await?;
    Ok(())
}
