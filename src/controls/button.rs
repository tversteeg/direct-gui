use blit::BlitBuffer;

use super::*;

pub struct Color(u32);

/// The skin of the button is rendered by an algorithm.
pub struct Flat {
    pub fill_color: Color
}

/// The skin of the button is a sprite.
pub struct Image {
    pub sprite_ref: SpriteRef
}

/// In what state the button currently is in as determined by the `update` function.
enum ButtonState {
    Normal,
    Hover,
    Pressed
}

/// A button widget that can be rendered in multiple ways:
/// `Flat`: using a simpel pixel rectangle algorithm.
/// `Image`: using a spritesheet divided into 3 parts for rendering its state.
pub struct Button<S> {
    /// How the button is rendered (`Flat` or `Image`).
    show: S,

    pos: (i32, i32),
    state: ButtonState
}

impl Button<Image> {
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
    pub fn new(sprite_ref: SpriteRef) -> Self {
        let img = Image {
            sprite_ref
        };

        Button { 
            show: img,
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

    /// Get if the button is currently pressed.
    pub fn is_pressed(&self) -> bool {
        match self.state {
            ButtonState::Pressed => true,
            _ => false
        }
    }
}

impl Control for Button<Image> {
    fn update(&mut self, args: &ControlState, res: &Resources) {
        let sprite = res.get_sprite(self.show.sprite_ref).unwrap();

        if !args.mouse_collision(self.pos, sprite.size()) {
            self.state = ButtonState::Normal;
        } else {
            self.state = match args.mouse_down {
                true => ButtonState::Pressed,
                false => ButtonState::Hover
            };
        }
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources) {
        let sprite = res.get_sprite(self.show.sprite_ref).unwrap();

        let mut draw_size = sprite.size();
        draw_size.1 /= 3;

        let height_offset = match self.state {
            ButtonState::Normal => 0,
            ButtonState::Hover => draw_size.1,
            ButtonState::Pressed => draw_size.1 * 2
        } as i32;

        sprite.blit_rect(buffer, buffer_width, self.pos, (0, height_offset, draw_size.0, draw_size.1));
    }
}
