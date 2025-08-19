use nes::ppu::renderer::RGB;
use rustc_hash::FxBuildHasher;
use std::hash::BuildHasher;

pub struct Frame {
    pub data: Vec<u8>,
    width: usize,
    height: usize,
    prev_pixel_hash: u64,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            data: vec![0x00; width * height * 3],
            width,
            height,
            prev_pixel_hash: 0,
        }
    }

    pub fn update(&mut self, pixel_buf: &Vec<RGB>) {
        assert_eq!(
            pixel_buf.len(),
            self.width * self.height,
            "Frame was not initialized with the same dimensions as the NES Pixel Buffer!"
        );

        let new_hash: u64 = FxBuildHasher.hash_one(pixel_buf);
        if new_hash != self.prev_pixel_hash {
            self.prev_pixel_hash = new_hash;
            for (index, color) in pixel_buf.iter().enumerate() {
                let idx: usize = index * 3;
                self.data[idx] = color.0;
                self.data[idx + 1] = color.1;
                self.data[idx + 2] = color.2;
            }
        }
    }
}
