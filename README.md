# What is this?

Engine is a Rust library for creating 2D games.

This is not meant to be a working game engine. It is
just an opengl test project for educational purposes.

## Usage

## Basic Project Template

```rust,no_run
use Engine::{Window, Size2D, Color};

fn main() {
	let window = Window::from("Title", Size2D::from(856, 482), Color::BLACK);

	Engine::Run(window, start, update, end);
}

fn start() {
    println!("Hello World!");
}

fn update() {
   
}

fn end() {
    println!("Goodbye World :(");
}
```
