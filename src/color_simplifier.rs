// Colors are stored in the following byte format:
// <unused> <R> <G> <B>
// Reading colors:
// r = (color >> 16) & 0xFF
// g = (color >> 8) & 0xFF
// b = color & 0xFF
//
// Other way around just bitshift in the other direction

pub struct ColorPalette {
    pal: Vec<u32>
}

impl ColorPalette {
    fn add(&mut self, r: u8, g: u8, b: u8) {

    }
}