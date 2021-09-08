use std::cmp::{Eq, PartialEq};
use std::convert::Into;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Position2D {
    pub x: f32,
	pub y: f32,
}

impl Position2D {
    pub const fn new() -> Position2D {
        Position2D { x: 0.0, y: 0.0 }
	}

	pub fn from<T: Into<f64>>(x: T, y: T) -> Position2D {
        Position2D { x: x.into() as f32, y: y.into() as f32 }
    }

    pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn add_num(&mut self, x: f32, y: f32) {
		self.x = self.x + x;
		self.y = self.y + y;
	}

	pub fn sub_num(&mut self, x: f32, y: f32) {
		self.x = self.x - x;
		self.y = self.y - y;
	}

	pub fn mul_num(&mut self, x: f32, y: f32) {
		self.x = self.x * x;
		self.y = self.y * y;
	}

	pub fn div_num(&mut self, x: f32, y: f32) {
		self.x = self.x / x;
		self.y = self.y / y;
	}
}

impl Eq for Position2D {}

impl PartialEq for Position2D {
    fn eq(&self, other: &Position2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Position2D {
	type Output = Position2D;

	fn add(self, other: Position2D) -> Position2D {
		Position2D {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl AddAssign for Position2D {
	fn add_assign(&mut self, other: Position2D) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
	}
}

impl Sub for Position2D {
	type Output = Position2D;

	fn sub(self, other: Position2D) -> Position2D {
		Position2D {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl SubAssign for Position2D {
	fn sub_assign(&mut self, other: Position2D) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
	}
}

impl Mul for Position2D {
	type Output = Position2D;

	fn mul(self, other: Position2D) -> Position2D {
		Position2D {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl MulAssign for Position2D {
	fn mul_assign(&mut self, other: Position2D) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
	}
}

impl <T> Mul<T> for Position2D
	where T: Into<f32> {
	type Output = Position2D;

	fn mul(self, value: T) -> Position2D {
		let scale = value.into();

		Position2D {
			x: self.x * scale,
			y: self.y * scale,
		}
	}
}

impl <T> MulAssign<T> for Position2D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x * scale;
		self.y = self.y * scale;
	}
}

impl Div for Position2D {
	type Output = Position2D;

	fn div(self, other: Position2D) -> Position2D {
		Position2D {
			x: self.x / other.x,
			y: self.y / other.y,
		}
	}
}

impl DivAssign for Position2D {
	fn div_assign(&mut self, other: Position2D) {
		self.x = self.x / other.x;
		self.y = self.y / other.y;
	}
}

impl <T> Div<T> for Position2D
	where T: Into<f32> {
	type Output = Position2D;

	fn div(self, value: T) -> Position2D {
		let scale = value.into();

		Position2D {
			x: self.x / scale,
			y: self.y / scale,
		}
	}
}

impl <T> DivAssign<T> for Position2D
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x / scale;
		self.y = self.y / scale;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn postion_2d_new() {
		let position = Position2D::new();
		assert_eq!(position, Position2D { x: 0.0, y: 0.0 });
	}

    #[test]
	fn postion_2d_from() {
        let position_def = Position2D::from(120, 74);
        let position_fdef = Position2D::from(120.0, 74.0);
        let position_u8 = Position2D::from(120u8, 74u8);
        let position_u16 = Position2D::from(120u16, 74u16);
        let position_u32 = Position2D::from(120u32, 74u32);
        let position_i8 = Position2D::from(120i8, 74i8);
        let position_i16 = Position2D::from(120i16, 74i16);
        let position_i32 = Position2D::from(120i32, 74i32);
        let position_f32 = Position2D::from(120.0f32, 74.0f32);
        let position_f64 = Position2D::from(120.0f64, 74.0f64);
        let position = Position2D { x: 120.0, y: 74.0 };
        assert_eq!(position_def, position);
        assert_eq!(position_fdef, position);
		assert_eq!(position_u8, position);
		assert_eq!(position_u16, position);
		assert_eq!(position_u32, position);
		assert_eq!(position_i8, position);
		assert_eq!(position_i16, position);
		assert_eq!(position_i32, position);
		assert_eq!(position_f32, position);
		assert_eq!(position_f64, position);
	}

	#[test]
	fn postion_2d_set() {
		let mut position = Position2D::new();
		position.set(28.0, 25.0);
		assert_eq!(position, Position2D { x: 28.0, y: 25.0 });
	}

	#[test]
	fn postion_2d_self_add() {
		let mut position = Position2D::from(90, 100);
		position.add_num(10.0, 10.0);
		assert_eq!(position, Position2D { x: 100.0, y: 110.0 });
	}

	#[test]
	fn postion_2d_self_sub() {
		let mut position = Position2D::from(90, 100);
		position.sub_num(10.0, 10.0);
		assert_eq!(position, Position2D { x: 80.0, y: 90.0 });
	}

	#[test]
	fn postion_2d_self_mul() {
		let mut position = Position2D::from(50, 100);
		position.mul_num(2.0, 1.0);
		assert_eq!(position, Position2D { x: 100.0, y: 100.0 });
	}

	#[test]
	fn postion_2d_self_div() {
		let mut position = Position2D::from(50, 100);
		position.div_num(2.0, 1.0);
		assert_eq!(position, Position2D { x: 25.0, y: 100.0 });
	}

	#[test]
	fn postion_2d_add() {
		let position1 = Position2D::from(255, 0);
		let position2 = Position2D::from(0, 255);
		let position3 = position1 + position2;
		assert_eq!(position3, Position2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn postion_2d_sub() {
		let position1 = Position2D::from(255, 0);
		let position2 = Position2D::from(255, 0);
		let position3 = position1 - position2;
		assert_eq!(position3, Position2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_mul() {
		let position1 = Position2D::from(50, 0);
		let position2 = Position2D::from(3, 0);
		let position3 = position1 * position2;
		assert_eq!(position3, Position2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_div() {
		let position1 = Position2D::from(100, 0);
		let position2 = Position2D::from(2, 1);
		let position3 = position1 / position2;
		assert_eq!(position3, Position2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_mul_num() {
		let position = Position2D::from(50, 0);
		let position2: Position2D = position * 2.0;
		assert_eq!(position2, Position2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_div_num() {
		let position = Position2D::from(100, 0);
		let position2: Position2D = position / 2.0;
		assert_eq!(position2, Position2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_add_assign() {
		let mut position1 = Position2D::from(255, 0);
		let position2 = Position2D::from(0, 255);
		position1 += position2;
		assert_eq!(position1, Position2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn postion_2d_sub_assign() {
		let mut position1 = Position2D::from(255, 0);
		let position2 = Position2D::from(255, 0);
		position1 -= position2;
		assert_eq!(position1, Position2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_mul_assign() {
		let mut position1 = Position2D::from(50, 0);
		let position2 = Position2D::from(3, 0);
		position1 *= position2;
		assert_eq!(position1, Position2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_div_assign() {
		let mut position1 = Position2D::from(100, 0);
		let position2 = Position2D::from(2, 1);
		position1 /= position2;
		assert_eq!(position1, Position2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_mul_num_assign() {
		let mut position = Position2D::from(50, 0);
		position *= 2.0;
		assert_eq!(position, Position2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn postion_2d_div_num_assign() {
		let mut position = Position2D::from(100, 0);
		position /= 2.0;
		assert_eq!(position, Position2D { x: 50.0, y: 0.0 });
	}
}