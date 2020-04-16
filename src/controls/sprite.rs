use super::*;

/// The skin of the button is a sprite.
#[derive(Debug)]
pub struct Sprite {
    pub sprite_ref: SpriteRef,
    pos: (i32, i32),
}

impl Sprite {
    pub fn new_with_sprite(sprite_ref: SpriteRef) -> Self {
        Sprite {
            sprite_ref,
            pos: (0, 0),
        }
    }

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
}

impl Control for Sprite {
    fn update(&mut self, _args: &ControlState, _res: &Resources) {
        // NoOp
    }

    fn draw(&self, buffer: &mut Vec<u32>, buffer_width: usize, res: &Resources) {
        let sprite = res.get_sprite(self.sprite_ref).unwrap();

        let draw_size = sprite.size();

        sprite.blit_rect(
            buffer,
            buffer_width,
            self.pos,
            (0, 0, draw_size.0, draw_size.1),
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn control_type(&self) -> ControlType {
        ControlType::Sprite
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
