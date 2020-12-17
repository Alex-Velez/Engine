use crate::Settings::Window;
use super::{Collision, Math, glfw::{self, Context}};

const DEG_RAD: f32 = std::f32::consts::PI / 180.0;
const RAD_DEG: f32 = 180.0 / std::f32::consts::PI;
const TWO_PIE: f32 = std::f32::consts::PI * 2.000;

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
	let (mut window, events) = glfw
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

	LoadContent();

	// render loop
    // -----------
	while !window.should_close() {

		unsafe { Math::DELTA_TIME = glfw.get_time() - Math::TOTAL_ELAPSED_SECONDS; };
		unsafe { Math::TOTAL_ELAPSED_SECONDS = glfw.get_time(); };

		Update();

		glfw.poll_events();

		// Render();
		// render
		// ------
		unsafe {
			gl::ClearColor(Math::TotalElapsedSeconds().sin() as f32, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
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
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {	
    pub const fn new() -> Color {
        Color { r: 255, g: 255, b: 255, a: 255 }
	}
	
	pub const fn from(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

impl Color {
	pub const WHITE: Color = Color::from(255, 255, 255, 255);
    pub const RED: Color = Color::from(255, 0, 0, 255);
    pub const GREEN: Color = Color::from(0, 255, 0, 255);
    pub const BLUE: Color = Color::from(0, 0, 255, 255);
    pub const BLACK: Color = Color::from(0, 0, 0, 255);
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
}

pub enum RotationMath {
	Add(f32),
	Sub(f32),
	Mul(f32),
	Div(f32),
	Pow(f32),
	Mod(f32),
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
		Rotation2D { degrees, radians: DEG_RAD * degrees }
	}

	pub fn from_rad(radians: f32) -> Rotation2D {
		Rotation2D { degrees: RAD_DEG * radians, radians }
	}

	pub fn deg(self) -> f32 { self.degrees }

	pub fn rad(self) -> f32 { self.radians }

	pub fn set_deg(mut self, degrees: f32) -> Rotation2D {
		self.degrees = degrees;
		self.radians = DEG_RAD * degrees;
		self
	}

	pub fn set_rad(mut self, radians: f32) -> Rotation2D {
		self.degrees = RAD_DEG * radians;
		self.radians = radians;
		self
	}

	pub fn add_deg(mut self, degrees: f32) -> Rotation2D {
		self.degrees = (self.degrees + degrees) % 360.0;
		self.radians = DEG_RAD * degrees;
		self
	}

	pub fn add_rad(mut self, radians: f32) -> Rotation2D {
		self.radians = (self.radians + radians) % TWO_PIE;
		self.degrees = RAD_DEG * radians;
		self
	}

	pub fn deg_math(mut self, op: RotationMath) -> Rotation2D {
		match op {
			RotationMath::Add(degrees) => { self.degrees = (self.degrees + degrees) % 360.0; }
			RotationMath::Sub(degrees) => { self.degrees = (self.degrees - degrees) % 360.0; }
			RotationMath::Mul(degrees) => { self.degrees = (self.degrees * degrees) % 360.0; }
			RotationMath::Div(degrees) => { self.degrees = (self.degrees / degrees) % 360.0; }
			RotationMath::Pow(degrees) => { self.degrees = (self.degrees.powf(degrees)) % 360.0; }
			RotationMath::Mod(degrees) => { self.degrees = (self.degrees % degrees) % 360.0; }
		}
		self.radians = DEG_RAD * self.degrees;
		self
	}
	
	pub fn rad_math(mut self, op: RotationMath) -> Rotation2D {
		match op {
			RotationMath::Add(radians) => { self.radians = (self.radians + radians) % TWO_PIE; }
			RotationMath::Sub(radians) => { self.radians = (self.radians - radians) % TWO_PIE; }
			RotationMath::Mul(radians) => { self.radians = (self.radians * radians) % TWO_PIE; }
			RotationMath::Div(radians) => { self.radians = (self.radians / radians) % TWO_PIE; }
			RotationMath::Pow(radians) => { self.radians = (self.radians.powf(radians)) % TWO_PIE; }
			RotationMath::Mod(radians) => { self.radians = (self.radians % radians) % TWO_PIE; }
		}
		self.degrees = RAD_DEG * self.radians;
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