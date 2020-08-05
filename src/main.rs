use crate::framebuffer::FrameBuffer;
use std::fs::File;
use std::io::Write;
use png::ColorType::RGB;
use std::io;
use rayon::prelude::*;
use std::path::PathBuf;
use rand::AsByteSliceMut;

mod framebuffer;
mod color_utils;
mod ass_emitter;

fn convert_png(input: String, output: String, at: u64, size: u64) {
    let decoder = png::Decoder::new(File::open(input).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut fb = FrameBuffer::new(info.width as usize, info.height as usize);

    let mut buf = vec![0; info.buffer_size()];
    let mut p = 4;
    if info.color_type == RGB {
        p = 3;
    }

    reader.next_frame(&mut buf).unwrap();

    for x in 0..info.width as usize {
        for y in 0..info.height as usize {
            let index = ((y * (info.width as usize)) * p) + (x * p);
            fb.set_pixel(x, y, color_utils::join_colors(buf[index], buf[index+1], buf[index+2]));
        }
    }

    let mut f = File::create(output).expect("Failed to create file!");

    for i in 0..(info.height as usize) / 4 {
        f.write(fb.create_ass_line(i, at, size).as_bytes()).expect("Failed to write");
        f.write("\n".as_bytes()).expect("Failed to write");
    }

    f.flush().expect("Failed to flush");
}

fn worker(z: &str) {
    let l = z.split("/").collect::<Vec<&str>>();
    let l = l[1].split(".png").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
    println!("{}", l);
    convert_png(z.to_string(),
                format!("{}.asstxt", z.to_string()),
                l * 4,
                4
    );
}

fn main() {
    /*
    let mut z = std::fs::read_dir("frames").expect("Failed to read directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().expect("Froge");


    z.par_iter().for_each(
        |el| worker(el.to_str().expect("A"))
    );*/

        let mut f = File::create("output.ass").expect("Failed to create file!");
        f.write(include_bytes!("top")).expect("Failed to write top");
        f.write(format!("\nPlayResX: {}\nPlayResY: {}\n\n", 120, 67).as_bytes()).expect("Failed to exist");
        f.write(include_bytes!("font")).expect("Failed to write font");
        f.write("\n".as_bytes());

        for i in 1..7502 {
            let mut content = std::fs::read(format!("frames/{:06}.png.asstxt", i)).unwrap();
            f.write(content.as_byte_slice_mut());

            println!("{} / {}", i, 7502);
        }
}
