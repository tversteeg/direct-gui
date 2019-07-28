mod button;
mod text;
mod sprite;

pub use self::button::{Button, ButtonState, Image, Flat};
pub use self::text::Label;
pub use self::sprite::Sprite;

use std::any::Any;

use super::resources::*;

/// Enum used to check what events should happen on certain controls.
pub enum ControlType {
    Button,
    Label,
    Sprite
}

/// Data that needs to be supplied to the `update` function.
pub struct ControlState {
    /// The position of the mouse cursor. Is not required to be inside the bounds of the screen.
    pub mouse_pos: (i32, i32),
    /// If the left mouse button is pressed or not.
    pub mouse_down: bool
}

impl ControlState {
    /// Determines if the mouse is inside a rectangle. Mostly used internally.
    pub fn mouse_collision(&self, pos: (i32, i32), size: (i32, i32)) -> bool {
        self.mouse_pos.0 >= pos.0 && self.mouse_pos.1 >= pos.1
            && self.mouse_pos.0 < pos.0 + size.0
            && self.mouse_pos.1 < pos.1 + size.1
    }
}

impl Default for ControlState {
    fn default() -> Self {
        ControlState {
            mouse_pos: (0, 0),
            mouse_down: false
        }
    }
}

pub trait Control {
    /// Update the control.
    fn update(&mut self, args: &ControlState, res: &Resources);

    /// Draw the control on the output buffer.
    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources);

    /// Retrieve what type of control this is.
    fn control_type(&self) -> ControlType;

    /// For downcasting.
    fn as_any(&self) -> &dyn Any;

    /// For downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
