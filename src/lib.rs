//! # What is this?
//!
//! Engine is a Rust library for creating 2D games.
//!
//! More specifically, Engine is a lightweight game framework for making
//! 2D games with minimum friction. It aims to implement a simple API made
//! for first time programmers and  game devs. It is supposed to avoid
//! overcomplicating the game making process and focuses on getting a game
//! running in as little time (and code) as possible.
//!
//! For a fuller outline, see the README.md on Github.
//!
//! ## Usage
//!
//! ## Basic Project Template
//!
//! ```rust,no_run
//! use Engine::{self, Window, Size2D, Color};
//! 
//! fn main() {
//!		let window = Window::from("TestGame", Size2D::from(856.0, 482.0), Color::BLACK);
//!
//!		Engine::Run(window, initialize, load_content, update, render);
//!	}
//!
//! fn initialize() {}
//! fn load_content() {}
//! fn update() {}
//! fn render() {}
//! ```

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub(crate) extern crate glfw;
pub(crate) extern crate gl;

//pub(crate) use shader::Shader;

pub mod Collision;
pub mod Visual;
pub mod Shader;
pub mod Math;
pub(crate) mod GL;
pub(crate) mod Verbose;

mod Settings;
mod Global;
mod Object;
mod Vector;

pub use Settings::*;
pub use Global::*;
pub use Object::*;
pub use Vector::*;

// Don't judge me!
macro_rules! impl_cc {
    (for $($t:ty),+) => {
        $(impl Copy for $t {})*
		$(impl Clone for $t { fn clone(&self) -> Self { *self } })*
    }
}

impl_cc!(for
	GL::Vertex,
	Math::Vector2D,
	Math::Vector3D,
	Window,
	Camera2D,
	Color,
	Position2D,
	Rotation2D,
	Scale2D,
	Transform2D,
	Size2D,
	Sprite2D
);