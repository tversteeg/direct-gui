extern crate direct_gui;
extern crate minifb;

use direct_gui::*;
use minifb::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0xFFFFFF; WIDTH * HEIGHT];

    let mut window = Window::new("direct-gui example - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
