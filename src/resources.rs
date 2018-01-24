use blit::{Color, BlitBuffer, BlitExt};
use image;
use std::fmt;
use std::path::Path;
use std::error::Error;

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

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// A newtype for handling objects externally by reference.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpriteRef(usize);

/// A internal handler of static resources such as sprites and fonts.
#[derive(Debug)]
pub struct Resources {
    sprites: Vec<BlitBuffer>,
    fonts: Vec<BlitBuffer>
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            sprites: Vec::new(),
            fonts: Vec::new(),
        }
    }

    /// Load image from a path. Accepts both PNG & BlitBuffer images which should have the `.png`
    /// and `.blit` extension respectively.
    ///
    /// The mask color is the color that will be used as alpha in the sprite, a common color to use
    /// for this is `0xFF00FF`.
    ///
    /// Returns a reference to the image.
    pub fn load_sprite_from_file<P>(&mut self, path: P, mask_color: Color) -> Result<SpriteRef, Box<Error>> where P: AsRef<Path> {
        let index = self.sprites.len();

        let buffer = Resources::load_blitbuffer(path.as_ref(), mask_color)?;
        self.sprites.push(buffer);

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

    fn load_blitbuffer(path: &Path, mask_color: Color) -> Result<BlitBuffer, Box<Error>> {
        let ext = path.extension().and_then(|s| s.to_str()).map_or("".to_string(), |s| s.to_ascii_lowercase());

        let buffer = match &ext[..] {
            "blit" => {
                BlitBuffer::open(path)?
            },
            "png" => {
                // Open the image from the path and convert it to a blit buffer
                let img = image::open(path)?;
                let rgb = img.as_rgb8().expect("Image is not of a valid type, consider removing the alpha channel");

                rgb.to_blit_buffer(mask_color)
            },
            format => return Err(Box::new(InvalidImageFormat))
        };

        Ok(buffer)
    }
}
