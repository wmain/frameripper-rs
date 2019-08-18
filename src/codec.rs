use crate::pixel::Pixel;
use bytes::BytesMut;
use image::ImageBuffer;
use std::io;
use tokio::codec::Decoder;

pub type FrameBuffer = ImageBuffer<Pixel, Vec<u8>>;

pub struct VideoFrameCodec {
  width: u32,
  height: u32,
  capacity: usize,
}

impl VideoFrameCodec {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      capacity: (width * height * 3) as usize,
    }
  }
}

impl Decoder for VideoFrameCodec {
  type Error = io::Error;
  type Item = FrameBuffer;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
    if src.len() == self.capacity {
      let buf_vec = src.to_vec();
      let frame_buffer = FrameBuffer::from_raw(self.width, self.height, buf_vec)
        .expect("Could not read frame into FrameBuffer");
      src.advance(self.capacity);
      Ok(Some(frame_buffer))
    } else {
      Ok(None)
    }
  }
}
