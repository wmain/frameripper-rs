use getopts::Options;
use snafu::ResultExt;

use crate::error::*;

pub struct Config {
  pub input_file_path: String,
  pub output_file_path: String,
  pub is_simple: bool,
}

impl Config {
  pub fn new(args: Vec<String>) -> Result<Self> {
    let mut opts = Options::new();

    opts.reqopt("i", "input_path", "The input video file", "");
    opts.reqopt("o", "output_path", "The output image path", "");
    opts.optflag(
      "s",
      "simple",
      "Average the entire frame color palette, rather than average by frame row.",
    );

    let matches = opts.parse(&args[1..]).context(ParseFailureError)?;

    let output = matches.opt_str("o").unwrap();
    let input = matches.opt_str("i").unwrap();
    let is_simple = matches.opt_present("s");

    Ok(Self {
      is_simple,
      input_file_path: input,
      output_file_path: output,
    })
  }
}
