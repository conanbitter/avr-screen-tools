use crate::color::{find_color, find_color_float};

const BAYER4: [f64; 16] = [
    0.0,
    8.0 / 16.0,
    2.0 / 16.0,
    10.0 / 16.0,
    12.0 / 16.0,
    4.0 / 16.0,
    14.0 / 16.0,
    6.0 / 16.0,
    3.0 / 16.0,
    11.0 / 16.0,
    1.0 / 16.0,
    9.0 / 16.0,
    15.0 / 16.0,
    7.0 / 16.0,
    13.0 / 16.0,
    5.0 / 16.0,
];

const BAYER8: [f64; 64] = [
    0.0,
    32.0 / 64.0,
    8.0 / 64.0,
    40.0 / 64.0,
    2.0 / 64.0,
    34.0 / 64.0,
    10.0 / 64.0,
    42.0 / 64.0,
    48.0 / 64.0,
    16.0 / 64.0,
    56.0 / 64.0,
    24.0 / 64.0,
    50.0 / 64.0,
    18.0 / 64.0,
    58.0 / 64.0,
    26.0 / 64.0,
    12.0 / 64.0,
    44.0 / 64.0,
    4.0 / 64.0,
    36.0 / 64.0,
    14.0 / 64.0,
    46.0 / 64.0,
    6.0 / 64.0,
    38.0 / 64.0,
    60.0 / 64.0,
    28.0 / 64.0,
    52.0 / 64.0,
    20.0 / 64.0,
    62.0 / 64.0,
    30.0 / 64.0,
    54.0 / 64.0,
    22.0 / 64.0,
    3.0 / 64.0,
    35.0 / 64.0,
    11.0 / 64.0,
    43.0 / 64.0,
    1.0 / 64.0,
    33.0 / 64.0,
    9.0 / 64.0,
    41.0 / 64.0,
    51.0 / 64.0,
    19.0 / 64.0,
    59.0 / 64.0,
    27.0 / 64.0,
    49.0 / 64.0,
    17.0 / 64.0,
    57.0 / 64.0,
    25.0 / 64.0,
    15.0 / 64.0,
    47.0 / 64.0,
    7.0 / 64.0,
    39.0 / 64.0,
    13.0 / 64.0,
    45.0 / 64.0,
    5.0 / 64.0,
    37.0 / 64.0,
    63.0 / 64.0,
    31.0 / 64.0,
    55.0 / 64.0,
    23.0 / 64.0,
    61.0 / 64.0,
    29.0 / 64.0,
    53.0 / 64.0,
    21.0 / 64.0,
];

fn posterize(_x: u32, _y: u32, r: u8, g: u8, b: u8) -> u16 {
    find_color(r, g, b)
}

const RK: f64 = 0.5;
const R5: f64 = (1.0 / 0b11111 as f64) * RK;
const R6: f64 = (1.0 / 0b111111 as f64) * RK;

fn ordered(pattern: &[f64], pattern_width: usize, pattern_height: usize, x: u32, y: u32, r: u8, g: u8, b: u8) -> u16 {
    let pattern_x = x as usize % pattern_width;
    let pattern_y = y as usize % pattern_height;
    let pattern_index = pattern_x + pattern_y * pattern_width;
    let offset = pattern[pattern_index] - 0.5;
    let r = r as f64 / 255.0 + R5 * offset;
    let g = g as f64 / 255.0 + R6 * offset;
    let b = b as f64 / 255.0 + R5 * offset;
    find_color_float(r, g, b)
}

pub fn get_converter(name: &str) -> impl Fn(u32, u32, u8, u8, u8) -> u16 {
    match name {
        "none" => posterize,
        "pat4" => |x: u32, y: u32, r: u8, g: u8, b: u8| ordered(&BAYER4, 4, 4, x, y, r, g, b),
        "pat8" => |x: u32, y: u32, r: u8, g: u8, b: u8| ordered(&BAYER8, 8, 8, x, y, r, g, b),
        _ => posterize,
    }
}
