use blit::BlitBuffer;

use super::*;

/// A text label widget using a font resource to render the text.
pub struct Label {
    font_ref: usize,

    pos: (i32, i32),

    text: String
}

impl Label {
    /// Create a new label for rendering of static text.
    pub fn new(font_ref: usize) -> Self {
        Label { 
            font_ref,
            pos: (0, 0),
            text: String::new()
        }
    }

    /// Map a position.
    pub fn pos(mut self, x: i32, y: i32) -> Self {
        self.pos = (x, y);

        self
    }

    /// Change the position.
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = (x, y);
    }

    /// Map the label text.
    pub fn text(mut self, text: &str) -> Self {
        self.text = String::from(text);

        self
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

impl Control for Label {
    fn update(&mut self, args: &ControlState, res: &Resources) { }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_size: (i32, i32), res: &Resources) {
    }
}
