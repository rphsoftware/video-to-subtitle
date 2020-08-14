use crate::color_utils;

pub struct FrameBuffer {
    pixels: Vec<u32>,
    width: usize
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
            pixels: vec![0; width * height],
            width
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.pixels[(y * self.width) + x] = value;
    }

    pub fn create_vector_string(&mut self, x: usize, y: usize) -> String {
        let mut base = String::with_capacity(50);
        base.push_str("{\\c&");
        let (r, g, b) = color_utils::split_colors(self.pixels[(y * self.width) + x]);
        base.push_str(&*format!("{:01$x}", b, 2));
        base.push_str(&*format!("{:01$x}", g, 2));
        base.push_str(&*format!("{:01$x}", r, 2));
        base.push_str("&}{\\p1}m 0 0 l 1 0 1 1 0 1 {\\p0}");

        return base;
    }

    pub fn create_ass_line(&mut self, l: usize, start: u64, len: u64) -> String {
        let mut z = String::with_capacity(self.width * 60);
        z.push_str("Dialogue: 0,");
        z.push_str(&*create_timestamp_string(start));
        z.push_str(",");
        z.push_str(&*create_timestamp_string(start + len));
        z.push_str(",Default,,0,0,0,,{\\pos(0");
        z.push_str(&*format!(",{}", l));
        z.push_str(")}");

        for i in 0..self.width {
            z.push_str(&*self.create_vector_string(i, l));
        }

        return z;
    }
}