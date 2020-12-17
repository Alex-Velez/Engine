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
//! use Engine;
//!
//! fn main() {
//!
//! }
//! ```

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub(crate) extern crate glfw;
pub(crate) use glfw::{Context, Key, Action};

pub(crate) extern crate gl;
//pub(crate) use gl::types::*;

//pub(crate) use shader::Shader;

pub(crate) use std::sync::mpsc::Receiver;
pub(crate) use std::ffi::CString;
pub(crate) use std::ptr;
pub(crate) use std::str;
pub(crate) use std::mem;
pub(crate) use std::os::raw::c_void;

pub mod Collision;
pub mod Visual;
pub mod Math;
pub(crate) mod GL;

mod Settings;
mod Global;
mod Object;

pub use Settings::*;
pub use Global::*;
pub use Object::*;

macro_rules! impl_cc {
    (for $($t:ty),+) => {
        $(impl Copy for $t {})*
		$(impl Clone for $t { fn clone(&self) -> Self { *self } })*
    }
}

impl_cc!(for
	GL::Vertex,
	Window,
	Color,
	Position2D,
	Rotation2D,
	Scale2D,
	Transform2D,
	Size2D,
	Sprite2D
);