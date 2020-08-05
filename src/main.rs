use crate::framebuffer::FrameBuffer;
use std::fs::File;
use std::io::Write;
use png::ColorType::RGB;
use std::{io, env};
use rayon::prelude::*;
use std::path::PathBuf;
use png::OutputInfo;

mod framebuffer;
mod color_utils;
mod ass_emitter;

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

    for i in 0..(info.height as usize) / 4 {
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

fn print_help() {
    println!("For usage visit github wiki.");
}

fn main() {/*
    let mut z = std::fs::read_dir("frames").expect("Failed to read directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().expect("Froge");


    z.par_iter().for_each(
        |el| worker(el.to_str().expect("A"))
    );*/
/*
        let mut f = File::create("output.ass").expect("Failed to create file!");
        f.write(include_bytes!("top")).expect("Failed to write top");
        f.write(format!("\nPlayResX: {}\nPlayResY: {}\n\n", 120, 67).as_bytes()).expect("Failed to exist");
        f.write(include_bytes!("font")).expect("Failed to write font");
        f.write("\n".as_bytes());

        for i in 1..2248 {
            let mut content = std::fs::read(format!("frames/{:09}.png.asstxt", i)).unwrap();
            f.write(content.as_byte_slice_mut());

            println!("{} / {}", i, 2248);
        }
*/
    /*
        CLI TODO:
            -- GENERATE ENTIRE SUBS WITH DIALOGUE LINE
                - ask for source image resolution
            -- VIDEO MODE
                - require ffmpeg
                - generate entire subs only
     */
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return print_help();
    }
    match args[1].as_str() {
        "i" => {
            if args.len() < 7 {
                return println!("Usage of image mode: <input AS PNG> <start centisecond> <end centisecond> <mode: f or p> <output file>");
            }
            // arg[2]: input, arg[3]: start, arg[4]: end, arg[5]: mode, arg[6]: target

            let start = args[3].parse::<u64>().unwrap();
            let end = args[4].parse::<u64>().unwrap();

            let iminfo = convert_png((&*args[2]).parse().unwrap(), "_________tmp.asstxt".parse().unwrap(), start, end - start);

            if args[5] == "f" {
                let data = std::fs::read("_________tmp.asstxt").unwrap();
                let mut f = File::create(&*args[6]).expect("Frog");
                f.write(generate_sub_file_header((iminfo.width / 4) as u64, (iminfo.height / 4) as u64).as_ref()).expect("Failed to write");
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

        },
        "j" => {

        },
        _ => {
            return print_help();
        }
    }
}
