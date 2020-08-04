pub fn join_colors(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) +
    ((g as u32) << 8) +
    (b as u32)
}

pub fn split_colors(source: u32) -> (u8, u8, u8) {
    (
        ((source >> 16) & 0xFF) as u8,
        ((source >> 8) & 0xFF)  as u8,
        (source & 0xFF)         as u8
    )
}


pub fn calculate_magic(c: u32) -> f64 {
    let (r, g, b) = split_colors(c);
    let mut rr : f64 = r as f64;
    let mut gg : f64 = g as f64;
    let mut bb : f64 = b as f64;

    rr *= rr;
    gg *= gg;
    bb *= bb;

    rr *= 0.299;
    gg *= 0.587;
    bb *= 0.114;

    (rr + gg + bb).sqrt()
}