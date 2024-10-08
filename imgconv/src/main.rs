use std::fs::File;

use anyhow::Result;
use image::ImageReader;

mod color;
mod dithering;
mod image_raw;

fn main() -> Result<()> {
    let converter = dithering::get_converter("pat8");
    let mut file = File::create("../test_data/card.img")?;

    for i in 0..=23 {
        println!("Image {}", i);
        let filename = if i == 0 {
            "../test_data/test.png".to_string()
        } else {
            format!("../test_data/img240/{:02}.png", i)
        };
        let img = ImageReader::open(filename)?.decode()?.to_rgb8();
        let mut raw = image_raw::Image::new(img.width(), img.height());
        for (x, y, c) in img.enumerate_pixels() {
            let index = x + y * raw.width;
            let color_converted = converter(x, y, c[0], c[1], c[2]);
            raw.data[index as usize] = color_converted;
        }
        raw.write(&mut file)?;
        //raw.save_as_image(format!("../test_data/img240/{:02}_p.png", i))?;
    }

    //raw.save_as_image("../test_data/test_result.png")?;
    //raw.save_raw("../test_data/test.img")?;
    return Ok(());
}
