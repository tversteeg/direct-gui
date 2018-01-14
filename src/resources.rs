use blit::{BlitBuffer, BlitExt};
use image;
use std::path::Path;
use std::error::Error;

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

    pub fn get_sprite(&self, sprite_ref: usize) -> Option<&BlitBuffer> {
        if sprite_ref < self.sprites.len() {
            Some(&self.sprites[sprite_ref])
        } else {
            None
        }
    }
}
