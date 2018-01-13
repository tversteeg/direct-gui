pub mod button;

/// GUI controls that can be rendered.
pub trait Drawable {
    fn draw(&self, buffer: &mut [u32]);
}

/// GUI controls that respond to keyboard input.
pub trait KeyboardInput {

}

/// GUI controls that respond to mouse input.
pub trait MouseInput {

}
