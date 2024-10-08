use std::path::Path;

use anyhow::Result;
use image::RgbImage;

use crate::color::revert_color;

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
            (c[0], c[1], c[2]) = revert_color(color);
        }
        result.save(filename)?;
        return Ok(());
    }
}
