use itertools::Itertools;

use crate::codec::FrameBuffer;
use crate::ripper::Dimensions;
use image::Rgb;

pub type Pixel = Rgb<u8>;

fn get_avg_pixel_color(pixels: Vec<&Pixel>) -> Pixel {
  let num_pixels = pixels.len() as f64;
  let averages = pixels.into_iter().fold([0.0f64; 3], |mut acc, pix| {
    let r = f64::from(pix[0]);
    let g = f64::from(pix[1]);
    let b = f64::from(pix[2]);
    acc[0] += r.powi(2);
    acc[1] += g.powi(2);
    acc[2] += b.powi(2);
    acc
  });
  let r = (averages[0] / num_pixels).sqrt().round() as u8;
  let g = (averages[1] / num_pixels).sqrt().round() as u8;
  let b = (averages[2] / num_pixels).sqrt().round() as u8;
  Rgb([r, g, b])
}

pub fn get_simple_col_average_pixels(buf: FrameBuffer, dimensions: &Dimensions) -> Vec<Pixel> {
  let average_pixel = get_avg_pixel_color(buf.pixels().collect());

  vec![average_pixel; dimensions.height as usize]
}

pub fn get_blended_col_average_pixels(buf: FrameBuffer, dimensions: &Dimensions) -> Vec<Pixel> {
  buf
    .pixels()
    .collect::<Vec<&Pixel>>()
    .chunks(dimensions.width as usize)
    .map(|chunk| get_avg_pixel_color(chunk.to_vec()))
    .collect()
}
