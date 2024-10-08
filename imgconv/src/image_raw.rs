use std::path::Path;

use anyhow::Result;
use image::RgbImage;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u16>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            data: vec![0; (width * height) as usize],
        }
    }

    pub fn save<P: AsRef<Path>>(self, filename: P) -> Result<()> {
        let mut result = RgbImage::new(self.width, self.height);
        for (x, y, c) in result.enumerate_pixels_mut() {
            let index = x + y * self.width;
            let color = self.data[index as usize];
            c[0] = ((color >> 8) & 0b11111000) as u8;
            c[1] = ((color >> 3) & 0b11111100) as u8;
            c[2] = ((color << 3) & 0b11111000) as u8;
        }
        result.save(filename)?;
        return Ok(());
    }
}
