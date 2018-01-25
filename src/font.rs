use blit::{Color, BlitBuffer};

use resources::Resources;

#[derive(Debug, Copy, Clone)]
pub struct FontSettings {
    pub start: char,
    pub chars: u8,
    pub char_size: (usize, usize),
    pub mask_color: Color
}

#[derive(Debug)]
pub struct Font {
    buffer: BlitBuffer,
    settings: FontSettings
}

impl Font {
    pub fn new(buffer: BlitBuffer, settings: FontSettings) -> Self {
        Font { buffer, settings }
    }
}
