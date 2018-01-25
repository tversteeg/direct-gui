extern crate direct_gui;
extern crate minifb;
extern crate blit;

use direct_gui::*;
use direct_gui::controls::*;
use minifb::*;
use blit::Color;

const WIDTH: usize = 200;
const HEIGHT: usize = 50;

fn on_button_state_changed<S>(button: &mut Button<S>, state: ButtonState) {
    println!("Button at position {:?} state changed to {:?}", button.pos(), state);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0x222222; WIDTH * HEIGHT];

    let mut window = Window::new("direct-gui example - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let button_img = gui.load_sprite_from_file("examples/button.png", Color::from_u32(0xFF00FF)).unwrap();
    gui.register(Button::new_with_sprite(button_img).with_pos(20, 10).with_callback(on_button_state_changed));
    gui.register(Button::new((80, 30), Color::from_u32(0xFF0000)).with_pos(100, 10).with_callback(on_button_state_changed));

    let font = gui.load_font_sprite_from_file("assets/TorusSans.png", FontSettings {
        start: '!',
        chars: '~' as u8 - '!' as u8,
        char_size: (8, 8),
        mask_color: Color::from_u32(0xFF00FF)
    });
    //gui.register(Label::new(button_img).pos(100, 10).text("This is a label"));

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
