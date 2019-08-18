use crate::codec::FrameBuffer;
use crate::TOTAL_PIXELS;
use image::Rgb;

pub type Pixel = Rgb<u8>;

pub fn get_average_pixel(buf: FrameBuffer) -> Pixel {
  let averages = buf.pixels().fold([0.0f64; 3], |mut acc, pix| {
    let r = f64::from(pix[0]);
    let g = f64::from(pix[1]);
    let b = f64::from(pix[2]);
    acc[0] += r.powi(2);
    acc[1] += g.powi(2);
    acc[2] += b.powi(2);
    acc
  });
  let r = (averages[0] / TOTAL_PIXELS).sqrt().round() as u8;
  let g = (averages[1] / TOTAL_PIXELS).sqrt().round() as u8;
  let b = (averages[2] / TOTAL_PIXELS).sqrt().round() as u8;

  Rgb([r, g, b])
}
