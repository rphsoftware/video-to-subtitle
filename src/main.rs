use crate::ass_emitter::Glyph;

mod color_utils;
mod color_simplifier;
mod ass_emitter;

fn main() {
    let mut pal = color_simplifier::generate_palette_256();

    // Therapist: The empty unsafe block isn't real, it can't hurt you
    // Empty unsafe block:
    unsafe {}

    #[cfg(debug_assertions)]
    {
        println!("{}", pal.simplify(0xEEEEEE));
        println!("{}", pal.simplify(0xEEEEEF));
        println!("{}", pal.simplify(0xEEEEEE));
        println!("{}", pal.simplify(0xEEEEEE));
        println!("{}", pal.simplify(0xEEEEEE));

        let mut a = Glyph::new();
        a.set_pixel(1, 3, true);
        a.set_pixel(0, 1, true);
        a.set_fg(0x0FFF00);
        println!("{}", a.to_ass_string());
    }

    #[cfg(not(debug_assertions))]
    println!("frog")
}
