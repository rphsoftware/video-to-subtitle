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
    }

    #[cfg(not(debug_assertions))]
    println!("frog")
}
