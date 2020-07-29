mod color_utils;
mod color_simplifier;
mod ass_emitter;

fn main() {
    println!("{:?}", color_utils::join_colors(255, 128, 64));
    println!("{:?}", color_utils::split_colors(0xFF8833));

    let pal = color_simplifier::generate_palette_256();
    println!("{:?}", pal.pal);
}
