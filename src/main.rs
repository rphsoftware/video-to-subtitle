mod color_utils;
mod color_simplifier;
mod ass_emitter;

fn main() {
    let mut pal = color_simplifier::generate_palette_256();

    println!("{}", pal.simplify(0xEEEEEE));
    println!("{}", pal.simplify(0xEEEEEE));
    println!("{}", pal.simplify(0xEEEEEE));
    println!("{}", pal.simplify(0xEEEEEE));
    println!("{}", pal.simplify(0xEEEEEE));
}
