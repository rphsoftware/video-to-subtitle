// This file's purpose is to emit .ass files
// Start, end, length are in milliseconds
// Colors are stored in the following byte format:
// <unused> <R> <G> <B>
// Reading colors:
// r = (color >> 16) & 0xFF
// g = (color >> 8) & 0xFF
// b = color & 0xFF
//
// Other way around just bitshift in the other direction

pub struct SubtitleLine {
    background: u32,
    foreground: u32,
    text: str,
    start: u64,
    end: u64
}

pub struct Subtitles {
    lines: Vec<SubtitleLine>,
    length: u64
}