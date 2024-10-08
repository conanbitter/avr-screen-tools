fn convert8to5(d: u8) -> u8 {
    let result = (d as f64 * 0b11111 as f64 / 0b11111111 as f64).round() as u8;
    if result > 0b11111 {
        0b11111
    } else {
        result
    }
}

fn convert8to6(d: u8) -> u8 {
    let result = (d as f64 * 0b111111 as f64 / 0b11111111 as f64).round() as u8;
    if result > 0b111111 {
        0b111111
    } else {
        result
    }
}

fn convert5to8(d: u8) -> u8 {
    let result = (d as f64 * 0b11111111 as f64 / 0b11111 as f64).round() as u16;
    if result > 0b11111111 {
        0b11111111
    } else {
        result as u8
    }
}

fn convert6to8(d: u8) -> u8 {
    let result = (d as f64 * 0b11111111 as f64 / 0b111111 as f64).round() as u16;
    if result > 0b11111111 {
        0b11111111
    } else {
        result as u8
    }
}

pub fn find_color(r: u8, g: u8, b: u8) -> u16 {
    let r = convert8to5(r);
    let g = convert8to6(g);
    let b = convert8to5(b);
    (r as u16) << 11 | (g as u16) << 5 | (b as u16)
}

pub fn find_color_float(r: f64, g: f64, b: f64) -> u16 {
    let r = ((r.clamp(0.0, 1.0) * 0b11111 as f64).round() as u16).min(0b11111);
    let g = ((g.clamp(0.0, 1.0) * 0b111111 as f64).round() as u16).min(0b111111);
    let b = ((b.clamp(0.0, 1.0) * 0b11111 as f64).round() as u16).min(0b11111);
    r << 11 | g << 5 | b
}

pub fn revert_color(color: u16) -> (u8, u8, u8) {
    (
        convert5to8(((color >> 11) & 0b11111) as u8),
        convert6to8(((color >> 5) & 0b111111) as u8),
        convert5to8((color & 0b11111) as u8),
    )
}
