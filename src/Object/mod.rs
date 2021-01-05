use crate::{
	Transform2D,
	Scale3D,
	Sprite2D,
	Animation2D,
	Collision,
	Color,
	Math::{Matrix4x4, Vector2D},
	Window,
};

#[derive(Copy, Clone, Debug)]
pub struct Camera2D {
	pub focus_position: Vector2D,
	pub zoom: f32,
}

impl Camera2D {
	pub const fn new() -> Camera2D {
		Camera2D { focus_position: Vector2D::new(), zoom: 1.0 }
	}

	pub const fn from(focus_position: Vector2D, zoom: f32) -> Camera2D {
		Camera2D { focus_position, zoom }
	}

	pub fn get_projection_matrix(self, window: Window) -> Matrix4x4 {
		let left = self.focus_position.x - window.size.width / 2.0;
		let right = self.focus_position.x + window.size.width / 2.0;
		let top = self.focus_position.y - window.size.height / 2.0;
		let bottom = self.focus_position.y + window.size.height / 2.0;

		let orthoMatrix: Matrix4x4 = Matrix4x4::create_orthographic_off_center(left, right, bottom, top, 0.01, 100.0);
		let zoomMatrix: Matrix4x4 = Matrix4x4::create_scale(Scale3D::from(self.zoom, self.zoom, self.zoom));

		orthoMatrix.mult(zoomMatrix)
	}
}

pub struct SimpleBody2D {
	transform: Transform2D,
	sprite: Sprite2D,
	body: Collision::Collision2D,
	visible: bool,
	modulate: Color,
}

impl SimpleBody2D {
	pub fn new(shape: Collision::Shape2D) -> SimpleBody2D {
		SimpleBody2D {
			transform: Transform2D::new(),
			sprite: Sprite2D::new(),
			body: Collision::Collision2D::new(shape),
			visible: true,
			modulate: Color::new(),
		}
	}
}

pub struct AnimatedBody2D {
	transform: Transform2D,
	animation: Animation2D,
	body: Collision::Collision2D,
	visible: bool,
	modulate: Color,
}

impl AnimatedBody2D {
	pub fn new(shape: Collision::Shape2D) -> AnimatedBody2D {
		AnimatedBody2D {
			transform: Transform2D::new(),
			animation: Animation2D::new(),
			body: Collision::Collision2D::new(shape),
			visible: true,
			modulate: Color::new(),
		}
	}
}

pub struct RigidBody2D {
	pub transform: Transform2D,
	pub mass: f32,
	pub weight: f32,
	pub drag: f32,
	pub angular_drag: f32,
	pub friction: f32,
	pub gravity_scale: f32,
}