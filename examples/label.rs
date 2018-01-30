extern crate direct_gui;
extern crate minifb;
extern crate blit;

use direct_gui::*;
use direct_gui::controls::*;
use minifb::*;
use blit::Color;

const WIDTH: usize = 400;
const HEIGHT: usize = 120;

fn main() {
    let mut buffer: Vec<u32> = vec![0x222222; WIDTH * HEIGHT];

    let mut window = Window::new("direct-gui label example - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let default_font = gui.default_font();
    gui.register(Label::new(default_font).pos(10, 10).text("This is a label with a default font\nand a line break."));

    let font = gui.load_font_sprite_from_file("assets/TorusSans.png", FontSettings {
        start: '!',
        char_size: (9, 9),
        leading_offset: 2,
        mask_color: Color::from_u32(0xFF00FF)
    }).unwrap();
    gui.register(Label::new(font).pos(10, 40).text("This is a label with a custom font."));

    let font = gui.load_font_sprite_from_file("assets/TorusSansGradient.png", FontSettings {
        start: '!',
        char_size: (9, 9),
        leading_offset: 2,
        mask_color: Color::from_u32(0xFF00FF)
    }).unwrap();
    gui.register(Label::new(font).pos(10, 60).text("This is a label with a custom font\nand fancy colors."));

    let label_ref = gui.register(Label::new(default_font).pos(10, 100).text("This label will be updated."));
    {
        let label = gui.get::<Label>(label_ref);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}
