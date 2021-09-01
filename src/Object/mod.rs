pub(crate) mod camera;

pub use camera::*;

/* 
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
*/