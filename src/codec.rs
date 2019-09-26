use crate::pixel::Pixel;
use bytes::BytesMut;
use image::ImageBuffer;
use std::io;
use tokio::codec::Decoder;

pub type FrameBuffer = ImageBuffer<Pixel, Vec<u8>>;

pub struct VideoFrameCodec {
  capacity: usize,
}

impl VideoFrameCodec {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      capacity: (width * height * 3) as usize,
    }
  }
}

impl Decoder for VideoFrameCodec {
  type Error = io::Error;
  type Item = BytesMut;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
    if src.capacity() < self.capacity {
      src.reserve(self.capacity)
    }
    if src.len() == self.capacity {
      Ok(Some(src.take()))
    } else {
      Ok(None)
    }
  }
}
