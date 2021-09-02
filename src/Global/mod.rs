use crate::{
	Camera2D,
	Debug,
	Math::{self, Matrix4x4, Vector2D},
	gl::{self, types::*},
	glfw::{self, Context, Action, Key},
	shader
};
use image::{self, RgbaImage};

use std::sync::mpsc::Receiver;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

pub(crate) mod color;
pub(crate) mod position;
pub(crate) mod rotation;
pub(crate) mod scale;
pub(crate) mod size;

pub use color::*;
pub use position::*;
pub use rotation::*;
pub use scale::*;
pub use size::*;

pub fn Run(win: Window, start: fn(), update: fn(), end: fn()) {

	start(); // user initialize function
	//Initialize();

	// glfw: initialize and configure
	// ------------------------------
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
	#[cfg(target_os = "macos")]
	glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
	glfw.window_hint(glfw::WindowHint::Resizable(win.resizable));
	glfw.window_hint(glfw::WindowHint::Visible(win.visible));
	glfw.window_hint(glfw::WindowHint::Decorated(win.decorations));
	glfw.window_hint(glfw::WindowHint::Focused(win.focused));

	// glfw window creation
	// --------------------
	let (mut window, events) = glfw
		.create_window(win.size.x as u32, win.size.y as u32, win.title, glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window!");

	//println!("Window '{}' ({}, {}) was successfully built!", win.title, win.size.x, win.size.y);
	Debug::log(&["info", "window"], format!("Window '{}' ({}, {}) was successfully built!", win.title, win.size.x, win.size.y));

	// Set window postion to center of screen
	let (x, y) = glfw.with_primary_monitor(|_: &mut _, monitor: Option<&glfw::Monitor>| {
		let screen = monitor.unwrap().get_workarea();
		return ((screen.2 - win.size.x as i32) / 2, (screen.3 - win.size.y as i32) / 2);
	});

	window.set_icon_from_pixels(gen_icon());
	window.set_pos(x, y);
	window.set_aspect_ratio(win.aspect_ratio.0, win.aspect_ratio.1);
	window.set_size_limits(
		Some(win.min.x as u32),
		Some(win.min.y as u32),
		Some(win.max.x as u32),
		Some(win.max.y as u32),
	);

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	// gl: load all OpenGL function pointers
	// ---------------------------------------
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe { gl::Viewport(0, 0, win.size.x as i32, win.size.y as i32) }

	glfw.set_swap_interval(glfw::SwapInterval::None); // VSync off (0)

	//load(); // user load content function
	// load content LoadContent();
	// -----------

	let (shader_program, VAO) = unsafe {

		let shader_program = shader::Load("shaders/VertexShader.vert", "shaders/FragmentShader.frag");

		let (mut VBO, mut VAO) = (0, 0);

		gl::GenVertexArrays(1, &mut VAO);
		gl::GenBuffers(1, &mut VBO);

		gl::BindVertexArray(VAO);

		gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

		let vertices: [f32; 30] = [
			-0.5,  0.5,	 1.0, 0.0, 0.0, // top left
			 0.5,  0.5,	 0.0, 1.0, 0.0, // top right
			-0.5, -0.5,	 0.0, 0.0, 1.0, // bottom left

			 0.5,  0.5,	 0.0, 1.0, 0.0, // top right
			 0.5, -0.5,	 0.0, 1.0, 1.0, // bottom right
			-0.5, -0.5,	 0.0, 0.0, 1.0, // bottom left
		];

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(mem::size_of::<GLfloat>() * vertices.len()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(0);

		//let my_num_ptr: *const GLsizei = &(5 * mem::size_of::<GLfloat>() as GLsizei);
		gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<GLfloat>() as GLsizei, ((2 * mem::size_of::<GLfloat>()) as GLsizei) as *const c_void);
		gl::EnableVertexAttribArray(1);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);

		(shader_program, VAO)
	};

	let cam = Camera2D::from(Vector2D::from(win.size.x / 2.0, win.size.y / 2.0), 1.0);
	let mut _delta_time: f64 = 0.0;
	let mut total_elapsed_seconds: f64 = 0.0;

	// render loop
	// -----------
	while !window.should_close() {

		_delta_time = glfw.get_time() - total_elapsed_seconds;
		total_elapsed_seconds = glfw.get_time();

		unsafe {
			Math::DELTA_TIME = _delta_time;
			Math::TOTAL_ELAPSED_SECONDS = total_elapsed_seconds;
		}

		// events
        // -----
		process_events(&mut window, &events);

		update(); // user update function
		//Update();

		glfw.poll_events();

		//render(); // user render function
		// render
		// ------
		unsafe {
			let win_color = win.color.unit_interval();
			gl::ClearColor(win_color[0], win_color[1], win_color[2], win_color[3]);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			let position = Position2D::from(400, 300);
			let scale = Scale3D::from(150, 100, 1);
			let rotation = Rotation2D::from_rad((total_elapsed_seconds.sin() * Math::TWO_PIE_F64) as f32);
			//Rotation2D::from_rad(std::f32::consts::PI / 4.0);

			let trans = Matrix4x4::create_tranlation(position.x, position.y, 0.0);
			let sca = Matrix4x4::create_scale(scale);
			let rot = Matrix4x4::create_rotation_z(rotation);

			shader::SetMatrix4x4(shader_program, "model", sca.mult(rot).mult(trans));

			shader::Use(vec![shader_program]);
			shader::SetMatrix4x4(shader_program, "projection", cam.get_projection_matrix(win));

			gl::BindVertexArray(VAO);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
			gl::BindVertexArray(0);

		}

		// glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
		// -------------------------------------------------------------------------------
		window.swap_buffers();
		glfw.poll_events();
		//glfw.wait_events();
	}

	end();
}


fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event { // general
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            _ => {}
		}
		KeyboardHandler(window, event);
    }
}

fn KeyboardHandler(window: &mut glfw::Window, event: glfw::WindowEvent) {
	match event {
		glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
		glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
			println!("Up ^");
		},
		glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
			println!("Left <-");
		},
		glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
			println!("Down v");
		},
		glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
			println!("Right ->");
		},
		_ => {}
	}
}

// todo: create unit tests and impls
#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub title: &'static str,
    pub icon: &'static str,
    pub size: Size2D,
    pub max: Size2D,
	pub min: Size2D,
	pub aspect_ratio: Ratio2D,
    pub resizable: bool,
    pub fullscreen: bool,
    pub maximized: bool,
    pub visible: bool,
    pub focused: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
	pub color: Color,
}

impl Window {
    pub const fn new() -> Window {
        Window {
            title: "window",
            icon: "",
            size: Size2D::from(856, 482),
            max: Size2D::from(856, 482),
			min: Size2D::from(160, 90),
			aspect_ratio: Ratio2D(16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
			color: Color::BLACK,
        }
    }

    pub const fn from(title: &'static str, size: Size2D, color: Color) -> Window {
        Window {
            title,
            icon: "",
            size,
            max: size,
			min: Size2D::from(160, 90),
			aspect_ratio: Ratio2D(16, 9),
            resizable: true,
            fullscreen: false,
            maximized: false,
            visible: true,
            focused: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
			color,
        }
    }

    pub const fn title(mut self, title: &'static str) -> Window {
        self.title = title;
        self
    }

    pub const fn icon(mut self, icon: &'static str) -> Window {
        self.icon = icon;
        self
    }

    pub const fn size(mut self, size: Size2D) -> Window {
        self.size = size;
        self
    }

    pub const fn max(mut self, max: Size2D) -> Window {
        self.max = max;
        self
    }

    pub const fn min(mut self, min: Size2D) -> Window {
        self.min = min;
        self
    }

    pub const fn resizable(mut self, resizable: bool) -> Window {
        self.resizable = resizable;
        self
    }

    pub const fn fullscreen(mut self, fullscreen: bool) -> Window {
        self.fullscreen = fullscreen;
        self
    }

    pub const fn maximized(mut self, maximized: bool) -> Window {
        self.maximized = maximized;
        self
    }

    pub const fn visible(mut self, visible: bool) -> Window {
        self.visible = visible;
        self
    }

    pub const fn focused(mut self, focused: bool) -> Window {
        self.focused = focused;
        self
    }

    pub const fn transparent(mut self, transparent: bool) -> Window {
        self.transparent = transparent;
        self
    }

    pub const fn decorations(mut self, decorations: bool) -> Window {
        self.decorations = decorations;
        self
    }

    pub const fn always_on_top(mut self, always_on_top: bool) -> Window {
        self.always_on_top = always_on_top;
        self
    }

    pub const fn color(mut self, color: Color) -> Window {
        self.color = color;
        self
    }

	pub const fn draw() {}
}

// todo: create unit tests and impls
#[derive(Copy, Clone, Debug)]
pub struct Transform2D {
	pub position: Position2D,
	pub rotation: Rotation2D,
	pub scale: Scale2D,
}

impl Transform2D {
	pub const fn new() -> Transform2D {
		Transform2D {
			position: Position2D::new(),
			rotation: Rotation2D::new(),
			scale: Scale2D::new(),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Ratio2D(u32, u32);

impl Eq for Ratio2D {}

impl PartialEq for Ratio2D {
    fn eq(&self, other: &Ratio2D) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}


// todo: create unit tests and impls
#[derive(Copy, Clone, Debug)]
pub struct Sprite2D {
    pub src: &'static str,
    pub size: Size2D,
    pub offset: Position2D,
    pub color: Color,
}

impl Sprite2D {
	pub const fn new() -> Sprite2D {
		Sprite2D { src: "", size: Size2D::new(), offset: Position2D::new(), color: Color::new() }
	}

    pub const fn from(src: &'static str, size: Size2D, offset: Position2D, color: Color) -> Sprite2D {
        Sprite2D { src, size, offset, color }
	}

	pub fn is_valid(self) -> bool {
		true
	}
}

// todo: fix icon data generator (make it work!)
fn gen_icon() -> Vec<glfw::PixelImage> {
	vec![
		// icon 1
		glfw::PixelImage {
			width: 100,
			height: 100,
			pixels: {
				let img: RgbaImage = image::open("resources/gear.png").unwrap().into_rgba8();
				let mut data_u8: Vec<u8> = Vec::new();
				for rgba in img.pixels() { data_u8.append(&mut rgba.0.to_vec()); }
				let mut data_u32: Vec<u32> = Vec::new();
				for x in data_u8 { data_u32.push(x as u32); }
				data_u32
			},
		}
	]
}

/*
#[derive(Copy, Clone, Debug)]
struct AnimationTrigger {
	pub index: usize,
	pub func: fn(),
}

impl AnimationTrigger {
	pub fn from(index: usize, func: fn()) -> AnimationTrigger {
		AnimationTrigger { index, func }
	}
}
/*
```rust
fn pork() {
	println!("i love pork");
}

let t1 = AnimationTrigger::from(3, pork);

let anim = Animation2D::new();

anim.insert_trigger(t1);

```
*/

pub enum Trigger {
	Animation,
	None,
}

pub struct Animation2D {
	pub sprite: Sprite2D,
	pub index: usize,
	pub tape: Vec<Sprite2D>,
	pub triggers: Vec<(usize, fn())>,
	pub pause: bool,
}

impl Animation2D {
	pub const fn new() -> Animation2D {
		Animation2D { sprite: Sprite2D::new(), index: 0, tape: Vec::<Sprite2D>::new(), triggers: Vec::<AnimationTrigger>::new(), pause: false }
	}

	pub const fn from(sprite: Sprite2D, tape: Vec<Sprite2D>, triggers: Vec<AnimationTrigger>) -> Animation2D {
		Animation2D { sprite, index: 0, tape, triggers, pause: false }
	}

	pub fn insert_sprite(&mut self, sprite: Sprite2D) {
		self.tape.push(sprite);
	}

	pub fn set_sprite(&mut self, index: usize) {
		self.index = index;
		self.sprite = self.tape[index];
	}

	pub fn insert_trigger(&mut self, index: usize, func: fn()) {
		self.triggers.push((index, func));
	}

	pub fn play(&mut self) {
		if !self.pause {
			self.index += 1;
			self.sprite = self.tape[self.index];


			for trigger in &self.triggers {
				if trigger.0 == self.index {
					(trigger.1)();
				}
			}
		}
	}

	pub fn stop(&mut self) {
		self.pause = true;
	}

	pub fn Trigger(index: usize, trigger: fn()) ->
}
*/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ratio_2d() {
		let ratio = Ratio2D(16, 9);
		assert_eq!(ratio, Ratio2D(16, 9));
	}
}
