use super::{
	Position2D,
	Rotation2D,
	Color,
	GL,
};

pub struct Triangle {
	pub points: (Position2D, Position2D, Position2D),
	pub offset: Position2D,
	pub rotation: Rotation2D,
	pub color: Color,
}

impl Triangle {
	pub const fn new() -> Triangle {
		Triangle {
			points: (Position2D::from(0.0, 0.0), Position2D::from(1.0, 0.0), Position2D::from(0.0, -1.0)),
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
			color: Color::new(),
		}
	}

	pub const fn from(point1: Position2D, point2: Position2D, point3: Position2D) -> Triangle {
		Triangle {
			points: (point1, point2, point3),
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
			color: Color::new(),
		}
	}
}

pub struct Rectangle {
	pub width: f32,
	pub height: f32,
	pub offset: Position2D,
	pub rotation: Rotation2D,
	pub color: Color,
}

impl Rectangle {
	pub const fn new(width: f32, height: f32) -> Rectangle {
		Rectangle {
			width,
			height,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
			color: Color::new(),
		}
	}
}

pub struct Circle {
	pub radius: f32,
	pub offset: Position2D,
	pub color: Color,
}

impl Circle {
	pub const fn new(radius: f32) -> Circle {
		Circle {
			radius,
			offset: Position2D::new(),
			color: Color::new(),
		}
	}
}

pub struct Capsule2D {
	pub radius: f32,
	pub height: f32,
	pub offset: Position2D,
	pub rotation: Rotation2D,
	pub color: Color,
}

impl Capsule2D {
	pub const fn new(radius: f32, height: f32) -> Capsule2D {
		Capsule2D {
			radius,
			height,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
			color: Color::new(),
		}
	}
}

pub struct Polygon2D {
	pub points: Vec<Position2D>,
	pub offset: Position2D,
	pub rotation: Rotation2D,
	pub color: Color,
}

impl Polygon2D {
	pub const fn new(points: Vec<Position2D>) -> Polygon2D {
		Polygon2D {
			points,
			offset: Position2D::new(),
			rotation: Rotation2D::new(),
			color: Color::new(),
		}
	}
}

pub enum Shape2D {
	Rectangle(Rectangle),
	Circle(Circle),
	Capsule(Capsule2D),
	Polygon(Polygon2D),
}