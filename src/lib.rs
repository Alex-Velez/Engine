//! # What is this?
//!
//! Engine is a Rust library for creating 2D games.
//!
//! This is not meant to be a working game engine. It is
//! just an opengl test project for educational purposes.
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
//!		let window = Window::from("Title", Size2D::from(856, 482), Color::BLACK);
//!
//!		Engine::Run(window, start, update, end);
//!	}
//!
//! fn start() {
//!     println!("Hello World!");
//! }
//!
//! fn update() {
//!    
//! }
//!
//! fn end() {
//!     println!("Goodbye World :(");
//! }
//! ```

// * I'll name my functions however I want >:(
#![allow(non_snake_case)]

pub(crate) extern crate glfw;
pub(crate) extern crate gl;

pub mod Collision;
pub mod Visual;
pub mod shader;
pub mod Math;
pub mod Debug;

mod Global;
mod Object;

// * Global Imports
pub use Global::*;
pub use Object::*;

/*
todo: Unit tests for Vector
todo: Rewrite arithmetic operations for Vector
*/