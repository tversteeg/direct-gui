use blit::{BlitBuffer, Color};

use super::*;

/// The skin of the button is rendered by an algorithm.
#[derive(Debug)]
pub struct Flat {
    pub color: Color,
    pub size: (i32, i32)
}

/// The skin of the button is a sprite.
#[derive(Debug)]
pub struct Image {
    pub sprite_ref: SpriteRef
}

/// In what state the button currently is in as determined by the `update` function.
#[derive(Debug)]
enum ButtonState {
    Normal,
    Hover,
    Pressed
}

/// A button widget that can be rendered in multiple ways:
/// `Flat`: using a simpel pixel rectangle algorithm.
/// `Image`: using a spritesheet divided into 3 parts for rendering its state.
#[derive(Debug)]
pub struct Button<S> {
    /// How the button is rendered (`Flat` or `Image`).
    show: S,

    pos: (i32, i32),
    state: ButtonState
}

impl<S> Button<S> {
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

impl Button<Flat> {
    /// Create a new colored button without text.
    pub fn new(size: (i32, i32), color: Color) -> Self {
        let show = Flat {
            size, color
        };

        Button { 
            show,
            pos: (0, 0),
            state: ButtonState::Normal
        }
    }
}

impl Control for Button<Flat> {
    fn update(&mut self, args: &ControlState, res: &Resources) {
        if !args.mouse_collision(self.pos, self.show.size) {
            self.state = ButtonState::Normal;
        } else {
            self.state = match args.mouse_down {
                true => ButtonState::Pressed,
                false => ButtonState::Hover
            };
        }
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources) {
        let mut color = self.show.color.u32();

        match self.state {
            ButtonState::Hover => color |= 0x444444,
            ButtonState::Pressed => color &= 0xAAAAAA,
            _ => ()
        }

        for y in self.pos.1..self.pos.1 + self.show.size.1 {
            for x in self.pos.0..self.pos.0 + self.show.size.0 {
                if x == self.pos.0 || x == self.pos.0 + self.show.size.0 - 1 {
                    buffer[x as usize + y as usize * buffer_width] = 0;
                } else if y == self.pos.1 || y == self.pos.1 + self.show.size.1 - 1 {
                    buffer[x as usize + y as usize * buffer_width] = 0;
                } else {
                    buffer[x as usize + y as usize * buffer_width] = color;
                }
            }
        }
    }
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
    pub fn new_with_sprite(sprite_ref: SpriteRef) -> Self {
        let img = Image {
            sprite_ref
        };

        Button { 
            show: img,
            pos: (0, 0),
            state: ButtonState::Normal
        }
    }
}

impl Control for Button<Image> {
    fn update(&mut self, args: &ControlState, res: &Resources) {
        let sprite = res.get_sprite(self.show.sprite_ref).unwrap();

        let mut real_size = sprite.size();
        real_size.1 /= 3;
        if !args.mouse_collision(self.pos, real_size) {
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
