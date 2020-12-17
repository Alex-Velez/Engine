use super::{
	Position2D,
	Rotation2D,
};

pub struct Rectangle {
	pub width: f32,
	pub height: f32,
	pub offset: Position2D,
	pub rotation: Rotation2D,
}

impl Rectangle {
	pub const fn new(width: f32, height: f32) -> Rectangle {
		Rectangle {
			width,
			height,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
		}
	}
}

pub struct Circle {
	pub radius: f32,
	pub offset: Position2D,
}

impl Circle {
	pub const fn new(radius: f32) -> Circle {
		Circle {
			radius,
			offset: Position2D::new(),
		}
	}
}

pub struct Capsule2D {
	pub radius: f32,
	pub height: f32,
	pub offset: Position2D,
	pub rotation: Rotation2D,
}

impl Capsule2D {
	pub const fn new(radius: f32, height: f32) -> Capsule2D {
		Capsule2D {
			radius,
			height,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
		}
	}
}

pub struct Polygon2D {
	pub points: Vec<Position2D>,
	pub offset: Position2D,
	pub rotation: Rotation2D,
}

impl Polygon2D {
	pub const fn new(points: Vec<Position2D>) -> Polygon2D {
		Polygon2D {
			points,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
		}
	}
}

pub enum Shape2D {
	Rectangle(Rectangle),
	Circle(Circle),
	Capsule(Capsule2D),
	Polygon(Polygon2D),
}

pub struct Collision2D {
	pub shape: Shape2D,
	pub colliding: bool,
	pub monitering: bool,
	pub layers: Vec<i32>,
}

impl Collision2D {
	pub fn new(shape: Shape2D) -> Collision2D {
		Collision2D {
			shape,
			colliding: false,
			monitering: true,
			layers: vec![0i32],
		}
	}
}