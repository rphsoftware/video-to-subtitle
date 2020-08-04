// This file's purpose is to emit .ass files
// Start, end, length are in milliseconds
// Colors are stored in the following byte format:
// <unused> <R> <G> <B>
// Reading colors:
// r = (color >> 16) & 0xFF
// g = (color >> 8) & 0xFF
// b = color & 0xFF
//
// Other way around just bitshift in the other direction
use crate::color_utils;

pub struct Glyph {
    fg_color: u32,
    bg_color: u32,
    glyph: u8
}

pub struct Line {
    x: u32,
    y: u32,
    glyphs: Vec<Glyph>
}

static GLYPH_MASKS:   [u8; 8] = [0x01, 0x08, 0x02, 0x10, 0x04, 0x20, 0x40, 0x80];

impl Glyph {
    pub fn new() -> Glyph {
        Glyph{ fg_color: 0, bg_color: 0, glyph: 0}
    }

    pub fn get_pixel(&mut self, x: u8, y: u8) -> bool {
        (self.glyph & GLYPH_MASKS[((y * 2) + x) as usize]) >= 1
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, state: bool) {
        if self.get_pixel(x, y) {
            if !state {
                self.glyph -= GLYPH_MASKS[((y * 2) + x) as usize];
            }
        } else {
            if state {
                self.glyph += GLYPH_MASKS[((y * 2) + x) as usize];
            }
        }
    }

    pub fn to_ass_string(&mut self) -> String {
        let mut base = String::with_capacity(40);
        base.push_str("{\\1c&H");

        let (r, g, b) = color_utils::split_colors(self.fg_color);
        base.push_str(&*format!("{:01$x}", b, 2));
        base.push_str(&*format!("{:01$x}", g, 2));
        base.push_str(&*format!("{:01$x}", r, 2));
        base.push_str("&}");

        base.push_str("{\\4c&H");

        let (r, g, b) = color_utils::split_colors(self.bg_color);
        base.push_str(&*format!("{:01$x}", b, 2));
        base.push_str(&*format!("{:01$x}", g, 2));
        base.push_str(&*format!("{:01$x}", r, 2));
        base.push_str("&}");

        base.push_str(&*std::char::from_u32(0x2800 + (self.glyph as u32)).expect("Something went horribly wrong").to_string());

        return base;
    }

    pub fn set_bg(&mut self, col: u32) {
        self.bg_color = col
    }

    pub fn set_fg(&mut self, col: u32) {
        self.fg_color = col
    }
}

impl Line {
    pub fn new(width: usize) -> Line {
        let mut l = Line{
            x: 0,
            y: 0,
            glyphs: Vec::with_capacity(width / 2)
        };

        // Pre-allocate all glyphs
        for _ in 0..(width / 2) {
            l.glyphs.push(Glyph::new());
        }

        l
    }

    pub fn get_glyph(&mut self, n : usize) -> Option<&mut Glyph> {
        return self.glyphs.get_mut(n);
    }

    pub fn set_glyph(&mut self, n : usize, g : Glyph) {
        self.glyphs[n] = g;
    }

    pub fn to_ass_string(&mut self) -> String {
        let mut base = String::with_capacity(self.glyphs.len() * 40);
        for i in 0..(self.glyphs.len()) {
            base.push_str(&*self.glyphs[i].to_ass_string())
        }

        return base;
    }
}
