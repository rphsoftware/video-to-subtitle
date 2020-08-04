use crate::color_simplifier::ColorPalette;
use crate::ass_emitter::Glyph;
use crate::color_utils;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub struct FrameBuffer {
    pixels: Vec<u32>,
    width: usize,
    height: usize
}

pub fn create_timestamp_string(u: u64) -> String {
    let mut u = u;

    let mut z = String::with_capacity(16);
    // Centiseconds
    let centiseconds = u % 100;
    u = u / 100;

    let seconds = u % 60;
    u = u / 60;

    let minutes = u % 60;
    u = u / 60;

    let hours = u;

    z.push_str(&*format!("{:01}:{:02}:{:02}.{:02}", hours, minutes, seconds, centiseconds));

    return z;
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer{
            pixels: vec![0; (width * height)],
            width,
            height
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.pixels[(y * self.width) + x] = value;
    }

    pub fn simplify(&mut self, pal: &mut ColorPalette) {
        for elem in self.pixels.iter_mut() {
            *elem = pal.simplify(*elem);
        }
    }

    pub fn create_glyph(&mut self, x: usize, y: usize) -> Glyph {
        let mut g = Glyph::new();
        let mut colors : HashSet<u32> = HashSet::with_capacity(8);
        let mut pixval : Vec<u32> = Vec::with_capacity(8);

        let base = (y * self.width * 4) + (x * 2);

        pixval.push(self.pixels[base]);
        pixval.push(self.pixels[base + 1]);
        pixval.push(self.pixels[base + self.width]);
        pixval.push(self.pixels[base + self.width + 1]);
        pixval.push(self.pixels[base + (self.width * 2)]);
        pixval.push(self.pixels[base + (self.width * 2) + 1]);
        pixval.push(self.pixels[base + (self.width * 3)]);
        pixval.push(self.pixels[base + (self.width * 3) + 1]);

        for i in 0..8 {
            colors.insert(pixval[i]);
        }

        // https://github.com/fifoc/encoder/blob/master/fifSegment.go#L101
        let mut colors = Vec::from_iter(colors);

        if colors.len() == 1 {
            g.set_bg(colors[0]);
            g.set_fg(colors[0]);
        } else if colors.len() == 2 {
            g.set_bg(colors[0]);
            g.set_fg(colors[1]);

            for i in 0..8 {
                if pixval[i] == colors[1] {
                    g.set_pixel((i & 0x1) as u8, (i >> 1) as u8, true);
                }
            }
        } else {
            let mut occurences : HashMap<u32, usize> = HashMap::with_capacity(8);
            for i in 0..8 {
                if let Some(occount) = occurences.get(&pixval[i]) {
                    occurences.insert(pixval[i], occount + 1);
                } else {
                    occurences.insert(pixval[i], 1);
                }
            }

            let mut smallest_magic = std::f64::MAX;
            let mut largest_magic = std::f64::MIN;
            let mut smallest_col: u32  = 0;
            let mut largest_col: u32  = 0;

            for (oc, _) in occurences.iter() {
                let lum = color_utils::calculate_magic(*oc);

                if lum > largest_magic {
                    largest_magic = lum;
                    largest_col = *oc;
                }

                if lum < smallest_magic {
                    smallest_magic = lum;
                    smallest_col = *oc;
                }
            }

            g.set_bg(smallest_col);
            g.set_fg(largest_col);

            for i in 0..8 {
                if pixval[i] == smallest_col {
                    // noop
                } else if pixval[i] == largest_col {
                    g.set_pixel((i & 0x1) as u8, (i >> 1) as u8, true);
                } else {
                    let magic = color_utils::calculate_magic(pixval[i]);

                    let sdelta = (smallest_magic - magic).abs();
                    let ldelta = (largest_magic - magic).abs();

                    if ldelta < sdelta {
                        g.set_pixel((i & 0x1) as u8, (i >> 1) as u8, true);
                    }
                }
            }
        }

        g
    }

    pub fn create_ass_line(&mut self, l: usize, start: u64, len: u64) -> String {
        let mut z = String::with_capacity(self.width * 60);
        z.push_str("Dialogue: 0,");
        z.push_str(&*create_timestamp_string(start));
        z.push_str(",");
        z.push_str(&*create_timestamp_string(start + len));
        z.push_str(",Default,,0,0,0,,{\\an5}{\\pos(");
        z.push_str(&*format!("{},{}", self.width / 8, l));
        z.push_str(".5)}");

        for i in 0..(self.width / 2) {
            z.push_str(&*self.create_glyph(i, l).to_ass_string());
        }

        return z;
    }

    pub fn raw(&mut self) { // This function likes it raw
        println!("{:?}", self.pixels);
    }
}