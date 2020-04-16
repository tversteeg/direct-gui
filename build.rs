//! Convert the default fonts in assets to a blitbuffer file which can be used by `default_font`.

extern crate blit;
extern crate image;

use std::fs;

use blit::*;

fn save_blit_buffer_from_image(name: &str, mask_color: u32) {
    let path = format!("assets/{}", name);

    println!("Converting image \"{}\" to blit buffer", path);

    let img = image::open(path).unwrap();
    let blit_buf = blit_buffer(&img, Color::from_u32(mask_color));

    blit_buf.save(format!("resources/{}.blit", name)).unwrap();
}

fn main() {
    fs::create_dir_all("resources").unwrap();

    let asset_paths = fs::read_dir("assets").unwrap();

    for path in asset_paths {
        let filepath = path.unwrap().path();
        let filename = filepath.file_name().unwrap();
        save_blit_buffer_from_image(filename.to_str().unwrap(), 0xFF_00_FF);
    }
}
