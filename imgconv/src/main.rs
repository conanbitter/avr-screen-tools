use anyhow::Result;
use image::ImageReader;

mod image_raw;

fn main() -> Result<()> {
    let img = ImageReader::open("../test_data/test.png")?.decode()?.to_rgb8();

    let mut raw = image_raw::Image::new(img.width(), img.height());

    for (x, y, c) in img.enumerate_pixels() {
        let r = c[0] >> 3;
        let g = c[1] >> 2;
        let b = c[2] >> 3;
        let index = x + y * raw.width;
        let color_converted: u16 = (r as u16) << 11 | (g as u16) << 5 | (b as u16);
        raw.data[index as usize] = color_converted;
    }

    raw.save("../test_data/test_result.png")?;
    return Ok(());
}
