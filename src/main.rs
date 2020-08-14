use crate::framebuffer::FrameBuffer;
use std::fs::File;
use std::io::Write;
use png::ColorType::RGB;
use std::{env};
use rayon::prelude::*;
use png::OutputInfo;

mod framebuffer;
mod color_utils;

fn convert_png(input: String, output: String, at: u64, size: u64) -> OutputInfo {
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

    for i in 0..(info.height as usize) {
        f.write(fb.create_ass_line(i, at, size).as_bytes()).expect("Failed to write");
        f.write("\n".as_bytes()).expect("Failed to write");
    }

    f.flush().expect("Failed to flush");

    return info;
}

fn generate_sub_file_header(x: u64, y: u64) -> String {
    let mut a = String::with_capacity(40960);
    a.push_str(include_str!("top"));
    a.push_str(&*format!("\nPlayResX: {}\nPlayResY: {}\n\n", x, y));
    a.push_str(include_str!("font"));
    a.push_str("\n");

    return a;
}

fn worker(z: &str, f: u64) {
    let l = z.split("/").collect::<Vec<&str>>();
    let l = l[1].split(".png").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
    println!("{}", l);
    convert_png(z.to_string(),
                format!("{}.asstxt", z.to_string()),
                l * f,
                f
    );
}

fn print_help() {
    println!("For usage visit github wiki.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return print_help();
    }
    match args[1].as_str() {
        "i" => {
            if args.len() < 7 {
                return println!("Usage of image mode: <input AS PNG> <start centisecond> <end centisecond> <mode: f or p> <output file>");
            }

            let start = args[3].parse::<u64>().unwrap();
            let end = args[4].parse::<u64>().unwrap();

            let iminfo = convert_png((&*args[2]).parse().unwrap(), "_________tmp.asstxt".parse().unwrap(), start, end - start);

            if args[5] == "f" {
                let data = std::fs::read("_________tmp.asstxt").unwrap();
                let mut f = File::create(&*args[6]).expect("Frog");
                f.write(generate_sub_file_header((iminfo.width) as u64, (iminfo.height) as u64).as_ref()).expect("Failed to write");
                f.write(&*data).expect("Failed to write");
                f.flush().expect("Failed to flush");

                std::fs::remove_file("_________tmp.asstxt").expect("Failed to delete");
                return;
            }
            if args[5] == "p" {
                let data = std::fs::read("_________tmp.asstxt").unwrap();
                let mut f = File::create(&*args[6]).expect("Frog");
                f.write(&*data).expect("Failed to write");
                f.flush().expect("Failed to flush");

                std::fs::remove_file("_________tmp.asstxt").expect("Failed to delete");
                return;
            }

            println!("Invalid file mode!");
            std::fs::remove_file("_________tmp.asstxt").expect("Failed to delete");
            return;
        },
        "v" => {
            if args.len() < 5 {
                return println!("Usage of video mode: <input directory> <How many multiples of 12.5 is the framerate [1, 2, 4, 8] supported> <amount of frames> <output file>");
            }

            let framerate = args[3].parse::<u64>().unwrap();
            let framect = args[4].parse::<u64>().unwrap();
            if framerate != 1 && framerate != 2 && framerate != 4 && framerate != 8 {
                return println!("Bad framerate!");
            }

            let mut goodfr: u64 = 0;

            if framerate == 1 { goodfr = 8; }
            if framerate == 2 { goodfr = 4; }
            if framerate == 4 { goodfr = 2; }
            if framerate == 8 { goodfr = 1; }

            let mut zz : Vec<String> = Vec::with_capacity(framect as usize);
            for i in 1..framect + 1 {
                zz.push(format!("{}/{:09}.png", args[2], i))
            }

            zz.par_iter().for_each(
                |el| worker(el.as_str(), goodfr)
            );

            let decoder = png::Decoder::new(File::open(format!("{}/{:09}.png", args[2], 1)).unwrap());
            let (info, _) = decoder.read_info().unwrap();

            let mut f = File::create(&*args[5]).expect("Failed to create file!");
            f.write(include_bytes!("top")).expect("Failed to write top");
            f.write(format!("\nPlayResX: {}\nPlayResY: {}\n\n", info.width, info.height).as_bytes()).expect("Failed to exist");
            f.write(include_bytes!("font")).expect("Failed to write font");
            f.write("\n".as_bytes()).expect("Failed to write newline");

            for i in 1..framect + 1 {
                let content = std::fs::read(format!("{}/{:09}.png.asstxt", args[2], i)).unwrap();
                std::fs::remove_file(format!("{}/{:09}.png.asstxt", args[2], i)).unwrap();
                f.write(content.as_slice()).expect("Failed to write");

                println!("{} / {}", i, framect);
            }
        }
        _ => {
            return print_help();
        }
    }
}
