use super::*;

/// A text label widget using a font resource to render the text.
pub struct Label {
    font_ref: FontRef,

    pos: (i32, i32),

    text: String
}

impl Label {
    /// Create a new label for rendering of static text.
    pub fn new(font_ref: FontRef) -> Self {
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

    /// Update the text of the label.
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

impl Control for Label {
    fn update(&mut self, _args: &ControlState, _res: &Resources) { }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources) {
        let font = res.get_font(self.font_ref).unwrap();

        font.draw_string(buffer, buffer_width, &self.text, self.pos);
    }

    fn control_type(&self) -> ControlType {
        ControlType::Label
    }

    fn as_any(&self) -> &Any {
        self
    }
}
