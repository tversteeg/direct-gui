# direct-gui
Simple direct rendering GUI controls

<a href="https://github.com/tversteeg/direct-gui/actions"><img src="https://github.com/tversteeg/direct-gui/workflows/CI/badge.svg" alt="CI"/></a>
<a href="https://crates.io/crates/direct-gui"><img src="https://img.shields.io/crates/v/direct-gui.svg" alt="Version"/></a>
<a href="https://docs.rs/direct-gui"><img src="https://img.shields.io/badge/api-rustdoc-blue.svg" alt="Rust Documentation"/></a>
<img src="https://img.shields.io/crates/l/direct-gui.svg" alt="License"/>

### [Documentation](https://docs.rs/direct-gui/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
direct-gui = "0.1"
```

And this to your crate root:

```rust
extern crate direct_gui;
```

### Run the examples

On Linux you need the `xkb` & `wayland` packages as required by `minifb` -- `sudo apt install libwayland-cursor0 libxkbcommon-dev libwayland-dev`

    cargo run --example button

![Example](img/example-button.png?raw=true)

    cargo run --example label

![Example](img/example-label.png?raw=true)

    cargo run --example sprite

![Example](img/example-sprite.png?raw=true)

## Examples

```rust
use direct_gui::*;
use direct_gui::controls::*;

let screen_size = (800i32, 600i32);

// Create a buffer where we will render to
let mut buffer: Vec<u32> = vec![0; (screen_size.0 * screen_size.1) as usize];

// Create a new instance of the gui
let mut gui = Gui::new(screen_size);

// Load the sprite of a button
let button_img = gui.load_sprite_from_file("examples/button.png", 0xFF00FF).unwrap();

// Create a new button using the sprite loaded before at pixel (20, 10)
gui.register(Button::new(button_img).pos(20, 10));

// Handle "input events" by pretending that the mouse is hovering over the button.
let cs = ControlState {
    mouse_pos: (22, 12),
    ..ControlState::default()
};
gui.update(&cs);

// Finally render the current gui state to the buffer
gui.draw_to_buffer(&mut buffer);
```

## Credits

Sprite feature by [Hammster](https://github.com/hammster)

Default font by [usr_share](https://opengameart.org/content/the-collection-of-8-bit-fonts-for-grafx2-r2)
