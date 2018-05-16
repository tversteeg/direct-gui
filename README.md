# direct-gui
Simple direct rendering GUI controls

[![Build Status](https://travis-ci.org/tversteeg/direct-gui.svg?branch=master)](https://travis-ci.org/tversteeg/direct-gui) [![Cargo](https://img.shields.io/crates/v/direct-gui.svg)](https://crates.io/crates/direct-gui) [![License: GPL-3.0](https://img.shields.io/crates/l/direct-gui.svg)](#license) [![Downloads](https://img.shields.io/crates/d/direct-gui.svg)](#downloads)

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

On Linux you need the `xorg-dev` package as required by `minifb` -- `sudo apt install xorg-dev`

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
