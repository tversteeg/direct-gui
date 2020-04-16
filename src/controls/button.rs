use blit::Color;

use super::*;

/// The skin of the button is rendered by an algorithm.
#[derive(Debug)]
pub struct Flat {
    pub color: Color,
    pub size: (i32, i32),
}

/// The skin of the button is a sprite.
#[derive(Debug)]
pub struct Image {
    pub sprite_ref: SpriteRef,
}

/// In what state the button currently is in as determined by the `update` function.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ButtonState {
    /// The mouse is not hovering over the button and it's not pressed.
    Normal,
    /// The mouse is hovering over the button but it's not pressed.
    Hover,
    /// The mouse is hovering over the button and it's pressed.
    Pressed,
}

/// A button widget that can be rendered in multiple ways:
/// `Flat`: using a simpel pixel rectangle algorithm.
/// `Image`: using a spritesheet divided into 3 parts for rendering its state.
pub struct Button<S> {
    /// How the button is rendered (`Flat` or `Image`).
    show: S,

    pos: (i32, i32),
    state: ButtonState,

    state_changed: fn(&mut Button<S>, ButtonState),
}

impl<S> Button<S> {
    /// Retrieve the position.
    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    /// Map a position.
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = (x, y);

        self
    }

    /// Change the position.
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos = (x, y);
    }

    /// Get if the button is currently pressed.
    pub fn pressed(&self) -> bool {
        match self.state {
            ButtonState::Pressed => true,
            _ => false,
        }
    }

    /// Set the event to a closure which will be called if the button state changes.
    pub fn with_callback(mut self, func: fn(&mut Button<S>, ButtonState)) -> Self {
        self.state_changed = func;

        self
    }

    /// An empty function so the callback doesn't has to be bound.
    fn empty_state_changed_callback(_: &mut Button<S>, _: ButtonState) {}
}

impl Button<Flat> {
    /// Create a new colored button without text.
    pub fn new(size: (i32, i32), color: Color) -> Self {
        let show = Flat { size, color };

        Button {
            show,
            pos: (0, 0),
            state: ButtonState::Normal,
            state_changed: Button::empty_state_changed_callback,
        }
    }
}

impl Control for Button<Flat> {
    fn update(&mut self, args: &ControlState, _res: &Resources) {
        let prev_state = self.state;

        if !args.mouse_collision(self.pos, self.show.size) {
            self.state = ButtonState::Normal;
        } else {
            self.state = if args.mouse_down {
                ButtonState::Pressed
            } else {
                ButtonState::Hover
            };
        }

        if prev_state != self.state {
            let state = self.state;
            (self.state_changed)(self, state);
        }
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, _res: &Resources) {
        let mut color = self.show.color.u32();

        match self.state {
            ButtonState::Hover => color |= 0x44_44_44,
            ButtonState::Pressed => color &= 0xAA_AA_AA,
            _ => (),
        }

        for y in self.pos.1..self.pos.1 + self.show.size.1 {
            for x in self.pos.0..self.pos.0 + self.show.size.0 {
                let index = x as usize + y as usize * buffer_width;
                if x == self.pos.0
                    || x == self.pos.0 + self.show.size.0 - 1
                    || y == self.pos.1
                    || y == self.pos.1 + self.show.size.1 - 1
                {
                    buffer[index] = 0;
                } else {
                    buffer[x as usize + y as usize * buffer_width] = color;
                }
            }
        }
    }

    fn control_type(&self) -> ControlType {
        ControlType::Button
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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
    /// ```compile_fail
    /// +-------+
    /// |Normal |
    /// +-------+
    /// | Hover |
    /// +-------+
    /// |Pressed|
    /// +-------+
    /// ```
    pub fn new_with_sprite(sprite_ref: SpriteRef) -> Self {
        let img = Image { sprite_ref };

        Button {
            show: img,
            pos: (0, 0),
            state: ButtonState::Normal,
            state_changed: Button::empty_state_changed_callback,
        }
    }
}

impl Control for Button<Image> {
    fn update(&mut self, args: &ControlState, res: &Resources) {
        let prev_state = self.state;

        let sprite = res.get_sprite(self.show.sprite_ref).unwrap();

        let mut real_size = sprite.size();
        real_size.1 /= 3;
        if !args.mouse_collision(self.pos, real_size) {
            self.state = ButtonState::Normal;
        } else {
            self.state = if args.mouse_down {
                ButtonState::Pressed
            } else {
                ButtonState::Hover
            };
        }

        if prev_state != self.state {
            let state = self.state;
            (self.state_changed)(self, state);
        }
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources) {
        let sprite = res.get_sprite(self.show.sprite_ref).unwrap();

        let mut draw_size = sprite.size();
        draw_size.1 /= 3;

        let height_offset = match self.state {
            ButtonState::Normal => 0,
            ButtonState::Hover => draw_size.1,
            ButtonState::Pressed => draw_size.1 * 2,
        } as i32;

        sprite.blit_rect(
            buffer,
            buffer_width,
            self.pos,
            (0, height_offset, draw_size.0, draw_size.1),
        );
    }

    fn control_type(&self) -> ControlType {
        ControlType::Button
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
