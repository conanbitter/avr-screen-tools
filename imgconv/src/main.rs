use anyhow::Result;
use image::ImageReader;

mod color;
mod dithering;
mod image_raw;

fn main() -> Result<()> {
    let img = ImageReader::open("../test_data/test.png")?.decode()?.to_rgb8();

    let mut raw = image_raw::Image::new(img.width(), img.height());
    let converter = dithering::get_converter("pat8");

    for (x, y, c) in img.enumerate_pixels() {
        let index = x + y * raw.width;
        let color_converted = converter(x, y, c[0], c[1], c[2]);
        raw.data[index as usize] = color_converted;
    }

    raw.save("../test_data/test_result.png")?;
    return Ok(());
}
