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
//! use Engine::{Window, Size2D, Color};
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

pub mod Collision;
pub mod Visual;
pub mod shader;
pub mod Math;
pub mod Object;
pub mod Log;

mod Global;
mod Vector;

pub use Global::*;
pub use Vector::*;

pub use Global::color::*;
pub use Global::rotation::*;