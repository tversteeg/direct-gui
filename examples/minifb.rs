extern crate direct_gui;
extern crate minifb;

use direct_gui::*;
use direct_gui::controls::*;
use minifb::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 50;

fn main() {
    let mut buffer: Vec<u32> = vec![0x222222; WIDTH * HEIGHT];

    let mut window = Window::new("direct-gui example - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let button_img = gui.load_sprite_from_file("examples/button.png", 0xFF00FF).unwrap();
    gui.register(Button::new(button_img).pos(20, 10));
    gui.register(Label::new(button_img).pos(100, 10).text("This is a label"));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        window.get_mouse_pos(MouseMode::Pass).map(|mouse| {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);
        });

        gui.update(&cs);
        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}
