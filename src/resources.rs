use blit::{BlitBuffer, Color};
use std::{error::Error, fmt};

#[cfg(feature = "file-loading")]
use blit::BlitExt;
#[cfg(feature = "file-loading")]
use image;
#[cfg(feature = "file-loading")]
use std::path::Path;

use super::font::*;

/// An error type for when a image has the wrong extension.
#[derive(Debug, Clone)]
pub struct InvalidImageFormat;

impl fmt::Display for InvalidImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file format doesn't match '.png' or '.blit'")
    }
}

impl Error for InvalidImageFormat {
    fn description(&self) -> &str {
        "file format doesn't match '.png' or '.blit'"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

/// A newtype for handling sprites objects externally by reference.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpriteRef(usize);

/// A newtype for handling font objects externally by reference.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FontRef(usize);

/// A internal handler of static resources such as sprites and fonts.
#[derive(Debug, Default)]
pub struct Resources {
    sprites: Vec<BlitBuffer>,
    fonts: Vec<Font>,
}

impl Resources {
    pub fn new() -> Self {
        let mut fonts = Vec::new();

        // Load the default font
        let default_font_buffer =
            BlitBuffer::from_memory(include_bytes!("../resources/ArtosSans.png.blit")).unwrap();

        let default_font_settings = FontSettings {
            start: '!',
            char_size: (9, 9),
            leading_offset: 2,
            mask_color: Color::from_u32(0xFF_00_FF),
        };
        fonts.push(Font::new(default_font_buffer, default_font_settings));

        Resources {
            fonts,
            sprites: Vec::new(),
        }
    }

    /// Return the default font loaded from the `assets/` folder and parsed by `build.rs`.
    pub fn default_font(&self) -> FontRef {
        FontRef(0)
    }

    /// Load image from a path.
    ///
    /// This function is only available when the `"file-loading"` feature is enabled.
    ///
    /// Accepts both PNG & BlitBuffer images which should have the `.png` and `.blit` extension respectively.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF_00_FF`.
    ///
    /// Returns a reference to the image.
    #[cfg(feature = "file-loading")]
    pub fn load_sprite_from_file<P>(
        &mut self,
        path: P,
        mask_color: Color,
    ) -> Result<SpriteRef, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let index = self.sprites.len();

        let buffer = Resources::load_blitbuffer(path.as_ref(), mask_color)?;
        self.sprites.push(buffer);

        Ok(SpriteRef(index))
    }

    /// Load image from serialized memory.
    pub fn load_sprite_from_memory(&mut self, buffer: &[u8]) -> Result<SpriteRef, Box<dyn Error>> {
        let index = self.sprites.len();

        let blitbuffer = BlitBuffer::from_memory(buffer)?;
        self.sprites.push(blitbuffer);

        Ok(SpriteRef(index))
    }

    /// Retrieves the sprite if it exists.
    pub fn get_sprite(&self, sprite_ref: SpriteRef) -> Option<&BlitBuffer> {
        if sprite_ref.0 < self.sprites.len() {
            Some(&self.sprites[sprite_ref.0])
        } else {
            None
        }
    }

    /// Load font image from a path. Accepts both PNG & BlitBuffer images which should have the `.png`
    /// and `.blit` extension respectively.
    ///
    /// This function is only available when the `"file-loading"` feature is enabled.
    ///
    /// Returns a reference to the font.
    #[cfg(feature = "file-loading")]
    pub fn load_font_sprite_from_file<P>(
        &mut self,
        path: P,
        settings: FontSettings,
    ) -> Result<FontRef, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let index = self.fonts.len();

        let buffer = Resources::load_blitbuffer(path.as_ref(), settings.mask_color)?;
        self.fonts.push(Font::new(buffer, settings));

        Ok(FontRef(index))
    }

    /// Load image from serialized memory.
    pub fn load_font_sprite_from_memory(
        &mut self,
        buffer: &[u8],
        settings: FontSettings,
    ) -> Result<FontRef, Box<dyn Error>> {
        let index = self.sprites.len();

        let blitbuffer = BlitBuffer::from_memory(buffer)?;
        self.fonts.push(Font::new(blitbuffer, settings));

        Ok(FontRef(index))
    }

    /// Retrieves the font if it exists.
    pub fn get_font(&self, font_ref: FontRef) -> Option<&Font> {
        if font_ref.0 < self.fonts.len() {
            Some(&self.fonts[font_ref.0])
        } else {
            None
        }
    }

    /// Load a encoded image from a file.
    ///
    /// This function is only available when the `"file-loading"` feature is enabled.
    #[cfg(feature = "file-loading")]
    pub fn load_blitbuffer(path: &Path, mask_color: Color) -> Result<BlitBuffer, Box<dyn Error>> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map_or("".to_string(), |s| s.to_ascii_lowercase());

        let buffer = match &ext[..] {
            "blit" => BlitBuffer::open(path)?,
            "png" => {
                // Open the image from the path and convert it to a blit buffer
                let img = image::open(path)?;
                let rgb = img
                    .as_rgb8()
                    .expect("Image is not of a valid type, consider removing the alpha channel");

                rgb.to_blit_buffer(mask_color)
            }
            _ => return Err(Box::new(InvalidImageFormat)),
        };

        Ok(buffer)
    }
}
