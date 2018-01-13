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

mod controls;

use controls::*;

pub struct Gui {
    width: usize,
    height: usize,

    drawables: Vec<Box<Drawable>>
}

impl Gui {
    /// Creates a new GUI.
    pub fn new(width: usize, height: usize) -> Self {
        Gui {
            width,
            height,
            drawables: Vec::new()
        }
    }

    /// Draw the drawable GUI controls on a target buffer.
    pub fn draw_to_buffer(&self, buffer: &mut [u32]) {
    }
}
