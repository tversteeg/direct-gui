use blit::{Color, BlitBuffer};

#[derive(Debug, Copy, Clone)]
pub struct FontSettings {
    pub start: char,
    pub char_size: (usize, usize),
    pub leading_offset: i32,
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

    pub fn draw_char(&self, buffer: &mut Vec<u32>, buffer_width: usize, ch: char, pos: (i32, i32)) {
        let offset = ch as u8 - self.settings.start as u8;
        let rect = (offset as i32 * self.settings.char_size.0 as i32, 0,
                    self.settings.char_size.0 as i32, self.settings.char_size.1 as i32);
        self.buffer.blit_rect(buffer, buffer_width, pos, rect);
    }

    pub fn draw_string(&self, buffer: &mut Vec<u32>, buffer_width: usize, string: &String, pos: (i32, i32)) {
        let chars = string.chars();

        let mut new_pos = pos;
        for ch in chars.into_iter() {
            match ch {
                ' ' => {
                    new_pos.0 += self.settings.char_size.0 as i32;
                },
                '\n' => {
                    // Align left, start on a newline on linebreak
                    new_pos.0 = pos.0;
                    new_pos.1 += self.settings.char_size.1 as i32 + self.settings.leading_offset;
                },
                ch => {
                    self.draw_char(buffer, buffer_width, ch, new_pos);
                    new_pos.0 += self.settings.char_size.0 as i32;
                }
            }
        }
    }
}
