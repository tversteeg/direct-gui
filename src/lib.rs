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
//! let button_img = gui.load_sprite_from_file("examples/button.png", Color::from_u32(0xFF00FF)).unwrap();
//!
//! // Create a new button using the sprite loaded before at pixel (20, 10)
//! gui.register(Button::new_with_sprite(button_img).with_pos(20, 10));
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

use std::error::Error;
use std::fmt;
use std::path::Path;

pub mod controls;
mod font;
mod resources;

pub use blit::Color;

use controls::*;
pub use font::FontSettings;
use resources::*;
pub use resources::{FontRef, SpriteRef};

/// An error type for when a reference is not valid anymore.
#[derive(Debug, Clone)]
pub struct InvalidControlReference;

impl fmt::Display for InvalidControlReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "reference to control object doesn't exist anymore")
    }
}

impl Error for InvalidControlReference {
    fn description(&self) -> &str {
        "reference to control object doesn't exist anymore"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

/// A newtype used to as a reference for controls.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ControlRef(usize);

/// The main entry point.
///
/// Typically a game has one instance of this struct where the resources are loaded before the main loop.
pub struct Gui {
    size: (i32, i32),

    resources: Resources,
    controls: Vec<(ControlRef, Box<dyn Control>)>,
    control_ref: usize,
}

impl Gui {
    /// Creates a new GUI.
    pub fn new(size: (i32, i32)) -> Self {
        Gui {
            size,
            resources: Resources::new(),
            controls: Vec::new(),
            control_ref: 0,
        }
    }

    /// Handle the user input and information as supplied by the windowing library.
    pub fn update(&mut self, state: &ControlState) {
        for control_tuple in self.controls.iter_mut() {
            control_tuple.1.update(state, &self.resources);
        }
    }

    /// Draw the drawable GUI controls on a target buffer.
    pub fn draw_to_buffer(&mut self, buffer: &mut Vec<u32>) {
        for control_tuple in self.controls.iter_mut() {
            control_tuple
                .1
                .draw(buffer, self.size.0 as usize, &self.resources);
        }
    }

    /// Draw a label a single frame.
    pub fn draw_label<S: Into<String>>(
        &mut self,
        buffer: &mut Vec<u32>,
        font_ref: FontRef,
        string: S,
        pos: (i32, i32),
    ) {
        let font = self.resources.get_font(font_ref).unwrap();

        font.draw_string(buffer, self.size.0 as usize, string.into(), pos);
    }

    /// Register a control.
    pub fn register<T: 'static + Control>(&mut self, ctrl: T) -> ControlRef {
        self.control_ref += 1;

        self.controls
            .push((ControlRef(self.control_ref), Box::new(ctrl)));

        ControlRef(self.control_ref)
    }

    /// Retrieve a control by reference.
    pub fn get<T: 'static + Control>(&self, control_ref: ControlRef) -> Result<&T, Box<dyn Error>> {
        match self.controls.iter().find(|&c| c.0 == control_ref) {
            Some(c) => match c.1.as_any().downcast_ref::<T>() {
                Some(obj) => Ok(obj),
                None => Err(Box::new(InvalidControlReference)),
            },
            None => Err(Box::new(InvalidControlReference)),
        }
    }

    /// Retrieve a control by mutable reference.
    pub fn get_mut<T: 'static + Control>(
        &mut self,
        control_ref: ControlRef,
    ) -> Result<&mut T, Box<dyn Error>> {
        match self.controls.iter_mut().find(|c| c.0 == control_ref) {
            Some(c) => match c.1.as_any_mut().downcast_mut::<T>() {
                Some(obj) => Ok(obj),
                None => Err(Box::new(InvalidControlReference)),
            },
            None => Err(Box::new(InvalidControlReference)),
        }
    }

    /// Return the default font loaded from the `assets/` folder and parsed by `build.rs`. Which is
    /// always the first item added to the fonts array.
    pub fn default_font(&self) -> FontRef {
        self.resources.default_font()
    }

    /// Load image from a path.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF00FF`.
    ///
    /// Returns a reference to the image.
    pub fn load_sprite_from_file<P>(
        &mut self,
        path: P,
        mask_color: Color,
    ) -> Result<SpriteRef, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        self.resources.load_sprite_from_file(path, mask_color)
    }

    /// Load image from serialized memory. Returns a reference to the image.
    pub fn load_sprite_from_memory(&mut self, buffer: &[u8]) -> Result<SpriteRef, Box<dyn Error>> {
        self.resources.load_sprite_from_memory(buffer)
    }

    /// Load font image from a path.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF00FF`.
    ///
    /// Returns a reference to the font.
    pub fn load_font_sprite_from_file<P>(
        &mut self,
        path: P,
        settings: FontSettings,
    ) -> Result<FontRef, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        self.resources.load_font_sprite_from_file(path, settings)
    }

    /// Load image from serialized memory. Returns a reference to the image.
    pub fn load_font_sprite_from_memory(
        &mut self,
        buffer: &[u8],
        settings: FontSettings,
    ) -> Result<FontRef, Box<dyn Error>> {
        self.resources
            .load_font_sprite_from_memory(buffer, settings)
    }
}
