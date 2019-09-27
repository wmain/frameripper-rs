use crate::ripper::Dimensions;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

pub struct RipperBar<'a> {
  input_path: &'a str,
  dimensions: &'a Dimensions,
  pub bar: ProgressBar,
}

impl<'a> RipperBar<'a> {
  pub fn new(input_path: &'a str, dimensions: &'a Dimensions) -> Self {
    let progress_bar = ProgressBar::new(dimensions.width as u64);

    progress_bar.set_style(
      ProgressStyle::default_bar()
        .template("{spinner:.red} {bar:75.cyan/blue} Frame: {pos:}/{len}"),
    );

    Self {
      input_path: input_path,
      dimensions: dimensions,
      bar: progress_bar,
    }
  }

  // TODO: Implement Display for Dimensions
  pub fn print_prelude(&self) {
    let ripping = style("Building barcode for").bold();
    let meta = format!(
      "{} ({:?}x{:?})",
      self.input_path, self.dimensions.width, self.dimensions.height
    );
    let styled_meta = style(meta).bold().red();
    let full_line = format!("{} {}", ripping, styled_meta);

    eprintln!("{}", full_line);
  }
}
