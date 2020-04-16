extern crate blit;
extern crate direct_gui;
extern crate minifb;

use blit::Color;
use direct_gui::controls::*;
use direct_gui::*;
use minifb::*;

const WIDTH: usize = 120;
const HEIGHT: usize = 50;

fn on_button_state_changed<S>(button: &mut Button<S>, state: ButtonState) {
    println!(
        "Button at position {:?} state changed to {:?}",
        button.pos(),
        state
    );
}

fn main() {
    let mut buffer: Vec<u32> = vec![0x22_22_22; WIDTH * HEIGHT];

    let mut window = Window::new(
        "direct-gui button example - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let button_img = gui
        .load_sprite_from_file("examples/button.png", Color::from_u32(0xFF_00_FF))
        .unwrap();
    gui.register(
        Button::new_with_sprite(button_img)
            .with_pos(10, 10)
            .with_callback(on_button_state_changed),
    );
    gui.register(
        Button::new((30, 30), Color::from_u32(0xFF_00_00))
            .with_pos(80, 10)
            .with_callback(on_button_state_changed),
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        if let Some(mouse) = window.get_mouse_pos(MouseMode::Pass) {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);
        }

        gui.update(&cs);
        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
