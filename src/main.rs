use crate::ass_emitter::{Glyph, Line};
use std::time::Instant;
use crate::framebuffer::FrameBuffer;
use rand;
use rand::Rng;
use std::fs::File;

mod framebuffer;
mod color_utils;
mod color_simplifier;
mod ass_emitter;

fn main() {
    let mut pal = color_simplifier::generate_palette_256();
    let mut rng = rand::thread_rng();

    // Therapist: The empty unsafe block isn't real, it can't hurt you
    // Empty unsafe block:
    unsafe {}

    #[cfg(debug_assertions)]
    {
        let mut fb = FrameBuffer::new(384, 216);
        let decoder = png::Decoder::new(File::open("test2.png").unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];

        reader.next_frame(&mut buf).unwrap();

        for x in 0..384 {
            for y in 0..216 {
                let index = ((y * 384) * 4) + (x * 4);
                fb.set_pixel(x, y, color_utils::join_colors(buf[index], buf[index+1], buf[index+2]));
            }
        }
        fb.simplify(&mut pal);

        for i in 0..54 {
            println!("{}", fb.create_ass_line(i, 0, 500));
        }

    }
}
