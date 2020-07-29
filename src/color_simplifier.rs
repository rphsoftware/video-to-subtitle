// Colors are stored in the following byte format:
// <unused> <R> <G> <B>
// Reading colors:
// r = (color >> 16) & 0xFF
// g = (color >> 8) & 0xFF
// b = color & 0xFF
//
// Other way around just bitshift in the other direction

// SOURCES FOR COLOR SIMPLIFICATION ALGHORITMS:
// https://github.com/fifoc/encoder

use crate::color_utils::join_colors;
use std::collections::HashMap;

pub struct ColorPalette {
    pub pal: Vec<u32>,
    simplification_cache: HashMap<u32, u32>
}

impl ColorPalette {
    fn add(&mut self, r: u8, g: u8, b: u8) {
        self.pal.push(join_colors(r, g, b));
    }
}

// Basic 256 color palette, the same one I used in my FIF encoder.
// Might not be the most optimal but works as a test and I have experience with it
// -Rph
pub fn generate_palette_256() -> ColorPalette {
    let mut hm : HashMap<u32, u32> = HashMap::new();
    let mut c = ColorPalette{ pal: vec![], simplification_cache: hm };
    // https://github.com/fifoc/encoder/blob/master/paletteGenerator.go#L4
    let red : [u8; 6] = [0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF];
    let green : [u8; 8] = [0x00, 0x24, 0x49, 0x6D, 0x92, 0xB6, 0xDB, 0xFF];
    let blue : [u8; 5] = [0x00, 0x40, 0x80, 0xC0, 0xFF];
    let gray : [u8; 16] = [0x0F, 0x1E, 0x2D, 0x3C, 0x4B, 0x5A, 0x69, 0x78, 0x87, 0x96, 0xA5, 0xB4, 0xC3, 0xD2, 0xE1, 0xF0];

    for r in 0..red.len() {
        for g in 0..green.len() {
            for b in 0..blue.len() {
                c.add(red[r], green[g], blue[b]);
            }
        }
    }

    for g in 0..gray.len() {
        c.add(gray[g], gray[g], gray[g]);
    }

    return c;
}