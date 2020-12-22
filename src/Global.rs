use super::{ Collision, Shader, Settings::Window, Camera2D,
	Math::{self, Matrix4x4, Vector2D},
	glfw::{self, Context, Key, Action},
	gl::{self, types::*},
};

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

/*
pub fn Run(mut win: Window, init: fn(), load: fn(), update: fn(), render: fn()) {
	Initialize();

	let window = win.build();

	LoadContent();

	// render loop
	// -----------
	while !window.should_close() {

		unsafe { Math::DELTA_TIME = glfw.get_time() - Math::TOTAL_ELAPSED_SECONDS; };
		unsafe { Math::TOTAL_ELAPSED_SECONDS = glfw.get_time(); };

		Update();

		glfw.poll_events();

		Render();
	}
	
	//window.close();
	window.set_should_close(true);
}
*/

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
	let new_shader = Shader::Shader::from("shaders/VertexShader.vert", "shaders/FragmentShader.frag");

	let (shaderProgram, VAO) = unsafe {

		let shaderProgram = Shader::Load(new_shader);

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
	let mut delta_time: f64 = 0.0;
	let mut total_elapsed_seconds: f64 = 0.0;

	// render loop
	// -----------
	while !window.should_close() {

		delta_time = glfw.get_time() - total_elapsed_seconds;
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

			Shader::SetMatrix4x4(shaderProgram, "model", sca.mult(rot).mult(trans));

			Shader::Use(vec![shaderProgram]);
			Shader::SetMatrix4x4(shaderProgram, "projection", cam.get_projection_matrix(win));

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

pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {	
	pub const fn new() -> Color {
		Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
	}
	
	pub fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
		Color { r: (r as f32) / 255.0, g: (g as f32) / 255.0, b: (b as f32) / 255.0, a: (a as f32) / 255.0 }
	}
	
	pub const fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color { r, g, b, a }
	}
}

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

	pub const fn set(mut self, x: f32, y: f32) -> Position2D {
		self.x = x;
		self.y = y;
		self
	}

	pub fn math(mut self, op: Math::Pos) -> Position2D {
		match op {
			Math::Pos::Add(x, y) => { self.x = self.x + x; self.y = self.x + y; }
			Math::Pos::Sub(x, y) => { self.x = self.x - x; self.y = self.x - y; }
			Math::Pos::Mul(x, y) => { self.x = self.x * x; self.y = self.x * y; }
			Math::Pos::Div(x, y) => { self.x = self.x / x; self.y = self.x / y; }
			Math::Pos::Mod(x, y) => { self.x = self.x % x; self.y = self.x % y; }
			Math::Pos::Pow(x, y) => { self.x = self.x.powf(x); self.y = self.y.powf(y); }
		}
		self
	}
}

pub struct Rotation2D {
	degrees: f32,
	radians: f32,
}

impl Rotation2D {
	pub const fn new() -> Rotation2D {
		Rotation2D { degrees: 0.0, radians: 0.0 }
	}

	pub fn from_deg(degrees: f32) -> Rotation2D {
		Rotation2D { degrees, radians: Math::DEG_RAD * degrees }
	}

	pub fn from_rad(radians: f32) -> Rotation2D {
		Rotation2D { degrees: Math::RAD_DEG * radians, radians }
	}

	pub fn get_deg(self) -> f32 { self.degrees }

	pub fn get_rad(self) -> f32 { self.radians }

	pub fn set_deg(mut self, degrees: f32) -> Rotation2D {
		self.degrees = degrees;
		self.radians = Math::DEG_RAD * degrees;
		self
	}

	pub fn set_rad(mut self, radians: f32) -> Rotation2D {
		self.degrees = Math::RAD_DEG * radians;
		self.radians = radians;
		self
	}

	pub fn add_deg(mut self, degrees: f32) -> Rotation2D {
		self.degrees = (self.degrees + degrees) % 360.0;
		self.radians = Math::DEG_RAD * degrees;
		self
	}

	pub fn add_rad(mut self, radians: f32) -> Rotation2D {
		self.radians = (self.radians + radians) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * radians;
		self
	}
	
	pub fn math(mut self, op: Math::Rot, unit: Math::RotT) -> Rotation2D {
		match unit {
			Math::RotT::Deg => {
				match op {
					Math::Rot::Add(degrees) => { self.degrees = (self.degrees + degrees) % 360.0; }
					Math::Rot::Sub(degrees) => { self.degrees = (self.degrees - degrees) % 360.0; }
					Math::Rot::Mul(degrees) => { self.degrees = (self.degrees * degrees) % 360.0; }
					Math::Rot::Div(degrees) => { self.degrees = (self.degrees / degrees) % 360.0; }
					Math::Rot::Mod(degrees) => { self.degrees = (self.degrees % degrees) % 360.0; }
					Math::Rot::Pow(degrees) => { self.degrees = (self.degrees.powf(degrees)) % 360.0; }
				}
				self.radians = Math::DEG_RAD * self.degrees;
			}
			Math::RotT::Rad => {
				match op {
					Math::Rot::Add(radians) => { self.radians = (self.radians + radians) % Math::TWO_PIE; }
					Math::Rot::Sub(radians) => { self.radians = (self.radians - radians) % Math::TWO_PIE; }
					Math::Rot::Mul(radians) => { self.radians = (self.radians * radians) % Math::TWO_PIE; }
					Math::Rot::Div(radians) => { self.radians = (self.radians / radians) % Math::TWO_PIE; }
					Math::Rot::Mod(radians) => { self.radians = (self.radians % radians) % Math::TWO_PIE; }
					Math::Rot::Pow(radians) => { self.radians = (self.radians.powf(radians)) % Math::TWO_PIE; }
				}
				self.degrees = Math::RAD_DEG * self.radians;
			}
		}
		self
	}
}

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
}

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

	pub const fn from(sprite: Sprite2D, tape: Vec<Sprite2D>, triggers: Vec<AnimationTrigger>, pause: bool) -> Animation2D {
		Animation2D { sprite, index: 0, tape, triggers, pause }
	}

	pub fn insert_sprite(mut self, sprite: Sprite2D) -> Animation2D {
		self.tape.push(sprite);
		self
	}

	pub fn set_sprite(mut self, index: usize) -> Animation2D {
		self.index = index;
		self.sprite = self.tape[index];
		self
	}

	pub fn insert_trigger(mut self, trigger: AnimationTrigger) -> Animation2D {
		self.triggers.push(trigger);
		self
	}

	pub fn play(mut self) {
		if !self.pause {
			self.index += 1;
			self.sprite = self.tape[self.index];
			for trigger in self.triggers {
				if trigger.index == self.index {
					(trigger.func)();
				}
			}
		}
	}

	pub fn stop(mut self) {
		self.pause = true;
	}


}