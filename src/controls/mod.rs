pub mod button;

pub use self::button::Button;

use blit::BlitBuffer;

pub struct ControlState {
    pub mouse_pos: (i32, i32),
    pub mouse_down: bool
}

impl ControlState {
    pub fn mouse_collision(&self, pos: (i32, i32), size: (i32, i32)) -> bool {
        self.mouse_pos.0 >= pos.0 && self.mouse_pos.1 >= pos.1
            && self.mouse_pos.0 < pos.0 + size.0 as i32
            && self.mouse_pos.1 < pos.1 + (size.1 / 3) as i32
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
    fn update(&mut self, args: &ControlState, sprites: &Vec<BlitBuffer>);

    fn draw(&self, buffer: &mut Vec<u32>, buffer_size: (i32, i32), images: &Vec<BlitBuffer>);
}
