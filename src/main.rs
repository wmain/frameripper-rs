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

    opts.reqopt("i", "input_path", "The input video file", "");
    opts.reqopt("o", "output_path", "The output image path", "");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let output = match matches.opt_str("o") {
        Some(o) => o,
        None => {
            eprintln!("Required output option not provided");
            process::exit(1);
        }
    };

    let input = match matches.opt_str("i") {
        Some(i) => i,
        None => {
            eprintln!("Required input option not provided");
            process::exit(1);
        }
    };

    let mut ripper = FrameRipper::new(&input, &output);
    ripper.rip().await?;
    Ok(())
}
