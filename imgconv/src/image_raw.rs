use std::{fs::File, io::Write, path::Path};

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

    pub fn save_as_image<P: AsRef<Path>>(self, filename: P) -> Result<()> {
        let mut result = RgbImage::new(self.width, self.height);
        for (x, y, c) in result.enumerate_pixels_mut() {
            let index = x + y * self.width;
            let color = self.data[index as usize];
            (c[0], c[1], c[2]) = revert_color(color);
        }
        result.save(filename)?;
        return Ok(());
    }

    pub fn save_raw<P: AsRef<Path>>(self, filename: P) -> Result<u32> {
        let data_size = self.width * self.height;
        let block_count = (data_size as f64 / 512.0).ceil() as u32;
        let end_padding = block_count * 512 - data_size;

        let mut file = File::create(filename)?;
        for pixel in self.data {
            file.write_all(&pixel.to_be_bytes())?;
        }
        for _ in 0..end_padding {
            file.write_all(&[0])?;
        }
        return Ok(block_count);
    }

    pub fn write(self, file: &mut File) -> Result<u32> {
        let data_size = self.width * self.height;
        let block_count = (data_size as f64 / 512.0).ceil() as u32;
        let end_padding = block_count * 512 - data_size;

        for pixel in self.data {
            file.write_all(&pixel.to_be_bytes())?;
        }
        for _ in 0..end_padding {
            file.write_all(&[0])?;
        }
        return Ok(block_count);
    }
}
