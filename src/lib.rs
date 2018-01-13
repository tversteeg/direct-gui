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

extern crate blit;
extern crate image;

use blit::{BlitBuffer, BlitExt};
use std::path::Path;
use std::error::Error;

pub mod controls;

use controls::*;

pub struct Gui {
    size: (i32, i32),

    sprites: Vec<BlitBuffer>,

    controls: Vec<Box<Control>>
}

impl Gui {
    /// Creates a new GUI.
    pub fn new(size: (i32, i32)) -> Self {
        Gui {
            size,
            sprites: Vec::new(),
            controls: Vec::new()
        }
    }

    /// Load image from a path.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF00FF`.
    ///
    /// Returns a reference to the image.
    pub fn load_sprite_from_file<P>(&mut self, path: P, mask_color: u32) -> Result<usize, Box<Error>> where P: AsRef<Path> {
        // Open the image from the path and convert it to a blit buffer
        let img = image::open(path)?;
        let rgb = img.as_rgb8().expect("Image is not of a valid type, consider removing the alpha channel");

        let index = self.sprites.len();

        self.sprites.push(rgb.as_blit_buffer(mask_color));

        Ok(index)
    }

    pub fn update(&mut self, state: &ControlState) {
        for control in self.controls.iter_mut() {
            control.update(state, &self.sprites);
        }
    }

    /// Draw the drawable GUI controls on a target buffer.
    pub fn draw_to_buffer(&self, buffer: &mut Vec<u32>) {
        for control in self.controls.iter() {
            control.draw(buffer, self.size, &self.sprites);
        }
    }

    /// Register a button.
    pub fn register_button(&mut self, button: Button) {
        self.controls.push(Box::new(button));
    }
}
