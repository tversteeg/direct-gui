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

### Run the example

On Linux you need the `xorg-dev` package as required by `minifb` -- `sudo apt install xorg-dev`

    cargo run --example minifb

This should produce the following window:

![Example](img/example.png?raw=true)

## Examples
