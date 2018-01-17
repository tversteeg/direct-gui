#![crate_name = "direct_gui"]

//! Draw GUI controls directly on a buffer
//!
//! # Usage
//!
//! This crate is [on crates.io](htpps://crates.io/crates/direct-gui) and can be used by adding
//! `direct-gui` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! direct-gui = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate direct_gui;
//! ```
//!
//! # Examples
//!
//! ```rust
//! use direct_gui::*;
//! use direct_gui::controls::*;
//!
//! let screen_size = (800i32, 600i32);
//!
//! // Create a buffer where we will render to
//! let mut buffer: Vec<u32> = vec![0; (screen_size.0 * screen_size.1) as usize];
//!
//! // Create a new instance of the gui
//! let mut gui = Gui::new(screen_size);
//!
//! // Load the sprite of a button
//! let button_img = gui.load_sprite_from_file("examples/button.png", 0xFF00FF).unwrap();
//!
//! // Create a new button using the sprite loaded before at pixel (20, 10)
//! gui.register(Button::new(button_img).pos(20, 10));
//!
//! // Handle "input events" by pretending that the mouse is hovering over the button.
//! let cs = ControlState {
//!     mouse_pos: (22, 12),
//!     ..ControlState::default()
//! };
//! gui.update(&cs);
//!
//! // Finally render the current gui state to the buffer
//! gui.draw_to_buffer(&mut buffer);
//! ```

extern crate blit;
extern crate image;

use std::path::Path;
use std::error::Error;

pub mod controls;
mod resources;

use blit::*;

use controls::*;
use resources::*;

/// The main entry point.
///
/// Typically a game has one instance of this struct where the resources are loaded before the main loop.
pub struct Gui {
    size: (i32, i32),

    resources: Resources,
    controls: Vec<Box<Control>>
}

impl Gui {
    /// Creates a new GUI.
    pub fn new(size: (i32, i32)) -> Self {
        Gui {
            size,
            resources: Resources::new(),
            controls: Vec::new()
        }
    }

    /// Handle the user input and information as supplied by the windowing library.
    pub fn update(&mut self, state: &ControlState) {
        for control in self.controls.iter_mut() {
            control.update(state, &self.resources);
        }
    }

    /// Draw the drawable GUI controls on a target buffer.
    pub fn draw_to_buffer(&self, buffer: &mut Vec<u32>) {
        for control in self.controls.iter() {
            control.draw(buffer, self.size.0 as usize, &self.resources);
        }
    }

    /// Register a control.
    pub fn register<T: 'static + Control>(&mut self, ctrl: T) {
        self.controls.push(Box::new(ctrl));
    }

    /// Load image from a path.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF00FF`.
    ///
    /// Returns a reference to the image.
    pub fn load_sprite_from_file<P>(&mut self, path: P, mask_color: Color) -> Result<SpriteRef, Box<Error>> where P: AsRef<Path> {
        self.resources.load_sprite_from_file(path, mask_color)
    }
}
