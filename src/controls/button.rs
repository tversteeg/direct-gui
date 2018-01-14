use blit::BlitBuffer;

use super::*;

enum ButtonState {
    Normal,
    Hover,
    Pressed
}

pub struct Button {
    sprite_ref: usize,

    pos: (i32, i32),
    state: ButtonState
}

impl Button {
    /// Create a new button with a sprite without text.
    ///
    /// The button image needs to be 3 buttons divided vertically:
    ///  1. normal state
    ///  2. mouse hover state
    ///  3. mouse pressed state
    ///
    /// ```ignore
    /// +-------+
    /// |Normal |
    /// +-------+
    /// | Hover |
    /// +-------+
    /// |Pressed|
    /// +-------+
    /// ```
    pub fn new(sprite_ref: usize) -> Self {
        Button { 
            sprite_ref,
            pos: (0, 0),
            state: ButtonState::Normal
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
}

impl Control for Button {
    fn update(&mut self, args: &ControlState, res: &Resources) {
        let sprite = res.get_sprite(self.sprite_ref).unwrap();

        if !args.mouse_collision(self.pos, sprite.size()) {
            self.state = ButtonState::Normal;
        } else {
            self.state = match args.mouse_down {
                true => ButtonState::Pressed,
                false => ButtonState::Hover
            };
        }
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_size: (i32, i32), res: &Resources) {
        let sprite = res.get_sprite(self.sprite_ref).unwrap();

        let mut draw_size = sprite.size();
        draw_size.1 /= 3;

        let height_offset = match self.state {
            ButtonState::Normal => 0,
            ButtonState::Hover => draw_size.1,
            ButtonState::Pressed => draw_size.1 * 2
        } as i32;

        sprite.blit_rect(buffer, buffer_size, self.pos, (0, height_offset, draw_size.0, draw_size.1));
    }
}
