use crate::{Object::Camera2D, shader,
	Math::{self, Matrix4x4, Vector2D},
	glfw::{self, Context}, 
	gl::{self, types::*},
};

//use std::sync::mpsc::Receiver;
//use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

pub(crate) mod rotation;
pub(crate) mod color;

use rotation::*;
use color::*;

pub fn Run(win: Window, init: fn(), load: fn(), update: fn(), render: fn()) {

	init(); // user initialize function
	Initialize();

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
	let (mut window, _events) = glfw
		.create_window(win.size.width as u32, win.size.height as u32, win.title, glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window!");

	println!("Window '{}' ({}, {}) was successfully built!", win.title, win.size.width, win.size.height);


	let (x, y) = glfw.with_primary_monitor(|_: &mut _, monitor: Option<&glfw::Monitor>| {
		let screen = monitor.unwrap().get_workarea();
		return ((screen.2 - win.size.width as i32) / 2, (screen.3 - win.size.height as i32) / 2);
	});

	window.set_pos(x, y);
	window.set_aspect_ratio(win.aspect_ratio.numer, win.aspect_ratio.denum);
	window.set_size_limits(
		Some(win.min.width as u32),
		Some(win.min.height as u32),
		Some(win.max.width as u32),
		Some(win.max.height as u32),
	);

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	// gl: load all OpenGL function pointers
	// ---------------------------------------
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe { gl::Viewport(0, 0, win.size.width as i32, win.size.height as i32) }

	glfw.set_swap_interval(glfw::SwapInterval::None); // VSync off (0)

	load(); // user load content function
	// load content LoadContent();
	// -----------

	let (shaderProgram, VAO) = unsafe {

		let shaderProgram = shader::Load("shaders/VertexShader.vert", "shaders/FragmentShader.frag");

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

		(shaderProgram, VAO)
	};

	let cam = Camera2D::from(Vector2D::from(win.size.width / 2.0, win.size.height / 2.0), 1.0);
	let mut _delta_time: f64 = 0.0;
	let mut total_elapsed_seconds: f64 = 0.0;

	// render loop
	// -----------
	while !window.should_close() {

		_delta_time = glfw.get_time() - total_elapsed_seconds;
		total_elapsed_seconds = glfw.get_time();

		update(); // user update function
		Update();

		glfw.poll_events();

		render(); // user render function
		// render
		// ------
		unsafe {
			gl::ClearColor(win.color.r, win.color.g, win.color.b, win.color.a);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			let position = Position2D::from(400.0, 300.0);
			let scale = Scale3D::from(150.0, 100.0, 1.0);
			let rotation = Rotation2D::from_rad((total_elapsed_seconds.sin() * Math::TWO_PIE_F64) as f32);
			//Rotation2D::from_rad(std::f32::consts::PI / 4.0);

			let trans = Matrix4x4::create_tranlation(position.x, position.y, 0.0);
			let sca = Matrix4x4::create_scale(scale);
			let rot = Matrix4x4::create_rotation_z(rotation);

			shader::SetMatrix4x4(shaderProgram, "model", sca.mult(rot).mult(trans));

			shader::Use(vec![shaderProgram]);
			shader::SetMatrix4x4(shaderProgram, "projection", cam.get_projection_matrix(win));

			gl::BindVertexArray(VAO);
			gl::DrawArrays(gl::TRIANGLES, 0, 6);
			gl::BindVertexArray(0);

		}

		// glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
		// -------------------------------------------------------------------------------
		window.swap_buffers();
		//glfw.poll_events();
		//glfw.wait_events();
	}

	//window.close();
	window.set_should_close(true);
}

pub fn Initialize() {
	
}

pub fn LoadContent() {

}

pub fn Update() {

}

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
            size: Size2D::from(856.0, 482.0),
            max: Size2D::from(856.0, 482.0),
			min: Size2D::from(160.0, 90.0),
			aspect_ratio: Ratio2D::from(16, 9),
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
			min: Size2D::from(160.0, 90.0),
			aspect_ratio: Ratio2D::from(16, 9),
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

#[derive(Copy, Clone, Debug)]
pub struct Position2D {
    pub x: f32,
	pub y: f32,
}

impl Position2D {
    pub const fn new() -> Position2D {
        Position2D { x: 0.0, y: 0.0 }
	}

	pub const fn from(x: f32, y: f32) -> Position2D {
        Position2D { x, y }
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Scale2D {
	pub x: f32,
	pub y: f32,
}

impl Scale2D {
	pub const fn new() -> Scale2D {
		Scale2D { x: 1.0, y: 1.0 }
	}

	pub const fn from(x: f32, y: f32) -> Scale2D {
		Scale2D { x, y }
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Scale3D {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Scale3D {
	pub const fn new() -> Scale3D {
		Scale3D { x: 1.0, y: 1.0, z: 1.0 }
	}

	pub const fn from(x: f32, y: f32, z: f32) -> Scale3D {
		Scale3D { x, y, z }
	}
}

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

#[derive(Clone, Copy, Debug)]
pub struct Size2D {
	pub width: f32,
	pub height: f32,
}

impl Size2D {
	pub const fn new() -> Size2D {
		Size2D { width: 0.0, height: 0.0 }
	}

	pub const fn from(width: f32, height: f32) -> Size2D {
		Size2D { width, height }
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Ratio2D {
	pub numer: u32,
	pub denum: u32,
}

impl Ratio2D {
	pub const fn new() -> Ratio2D {
		Ratio2D { numer: 16, denum: 9 }
	}

	pub const fn from(numer: u32, denum: u32) -> Ratio2D {
		Ratio2D { numer, denum }
	}
}

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

#[derive(Copy, Clone, Debug)]
pub struct AnimationTrigger {
	pub index: usize,
	pub func: fn(),
}

impl AnimationTrigger {
	pub fn from(index: usize, func: fn()) -> AnimationTrigger {
		AnimationTrigger { index, func }
	}
}

pub struct Animation2D {
	pub sprite: Sprite2D,
	pub index: usize,
	pub tape: Vec<Sprite2D>,
	pub triggers: Vec<AnimationTrigger>,
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

	pub fn insert_trigger(&mut self, trigger: AnimationTrigger) {
		self.triggers.push(trigger);
	}

	pub fn next(&mut self) {
		if !self.pause {
			self.index += 1;
			self.sprite = self.tape[self.index];
			for trigger in &self.triggers {
				if trigger.index == self.index {
					(trigger.func)();
				}
			}
		}
	}

	pub fn stop(&mut self) {
		self.pause = true;
	}
}