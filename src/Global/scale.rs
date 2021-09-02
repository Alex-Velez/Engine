use std::cmp::{Eq, PartialEq};
use std::convert::{From, Into};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Scale2D {
	pub x: f32,
	pub y: f32,
}

impl Scale2D {
	pub const fn new() -> Scale2D {
		Scale2D { x: 1.0, y: 1.0 }
	}

	pub const fn from(x: u32, y: u32) -> Scale2D {
		Scale2D { x: x as f32, y: y as f32 }
	}

    pub const fn from_f32(x: f32, y: f32) -> Scale2D {
		Scale2D { x, y }
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

impl Eq for Scale2D {}

impl PartialEq for Scale2D {
    fn eq(&self, other: &Scale2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Scale2D {
	type Output = Scale2D;

	fn add(self, other: Scale2D) -> Scale2D {
		Scale2D {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl AddAssign for Scale2D {
	fn add_assign(&mut self, other: Scale2D) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
	}
}

impl Sub for Scale2D {
	type Output = Scale2D;

	fn sub(self, other: Scale2D) -> Scale2D {
		Scale2D {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl SubAssign for Scale2D {
	fn sub_assign(&mut self, other: Scale2D) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
	}
}

impl Mul for Scale2D {
	type Output = Scale2D;

	fn mul(self, other: Scale2D) -> Scale2D {
		Scale2D {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl MulAssign for Scale2D {
	fn mul_assign(&mut self, other: Scale2D) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
	}
}

impl <T> Mul<T> for Scale2D
	where T: Into<f32> {
	type Output = Scale2D;

	fn mul(self, value: T) -> Scale2D {
		let scale = value.into();

		Scale2D {
			x: self.x * scale,
			y: self.y * scale,
		}
	}
}

impl <T> MulAssign<T> for Scale2D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x * scale;
		self.y = self.y * scale;
	}
}

impl Div for Scale2D {
	type Output = Scale2D;

	fn div(self, other: Scale2D) -> Scale2D {
		Scale2D {
			x: self.x / other.x,
			y: self.y / other.y,
		}
	}
}

impl DivAssign for Scale2D {
	fn div_assign(&mut self, other: Scale2D) {
		self.x = self.x / other.x;
		self.y = self.y / other.y;
	}
}

impl <T> Div<T> for Scale2D
	where T: Into<f32> {
	type Output = Scale2D;

	fn div(self, value: T) -> Scale2D {
		let scale = value.into();

		Scale2D {
			x: self.x / scale,
			y: self.y / scale,
		}
	}
}

impl <T> DivAssign<T> for Scale2D
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x / scale;
		self.y = self.y / scale;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Scale3D {
	pub x: f32,
	pub y: f32,
    pub z: f32,
}

impl Scale3D {
	pub const fn new() -> Scale3D {
		Scale3D { x: 1.0, y: 1.0, z: 1.0 }
	}

    pub const fn from(x: u32, y: u32, z: u32) -> Scale3D {
		Scale3D { x: x as f32, y: y as f32, z: z as f32 }
	}

	pub const fn from_f32(x: f32, y: f32, z: f32) -> Scale3D {
		Scale3D { x, y, z }
	}

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
		self.x = x;
		self.y = y;
        self.z = z;
	}

	pub fn add_num(&mut self, x: f32, y: f32, z: f32) {
		self.x = self.x + x;
		self.y = self.y + y;
        self.z = self.z + z;
	}

	pub fn sub_num(&mut self, x: f32, y: f32, z: f32) {
		self.x = self.x - x;
		self.y = self.y - y;
        self.z = self.z - z;
	}

	pub fn mul_num(&mut self, x: f32, y: f32, z: f32) {
		self.x = self.x * x;
		self.y = self.y * y;
        self.z = self.z * z;
	}

	pub fn div_num(&mut self, x: f32, y: f32, z: f32) {
		self.x = self.x / x;
		self.y = self.y / y;
        self.z = self.y / z;
	}
}

impl Eq for Scale3D {}

impl PartialEq for Scale3D {
    fn eq(&self, other: &Scale3D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Scale3D {
	type Output = Scale3D;

	fn add(self, other: Scale3D) -> Scale3D {
		Scale3D {
			x: self.x + other.x,
			y: self.y + other.y,
            z: self.z + other.z,
		}
	}
}

impl AddAssign for Scale3D {
	fn add_assign(&mut self, other: Scale3D) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
		self.z = self.z + other.z;
	}
}

impl Sub for Scale3D {
	type Output = Scale3D;

	fn sub(self, other: Scale3D) -> Scale3D {
		Scale3D {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl SubAssign for Scale3D {
	fn sub_assign(&mut self, other: Scale3D) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
		self.z = self.z - other.z;
	}
}

impl Mul for Scale3D {
	type Output = Scale3D;

	fn mul(self, other: Scale3D) -> Scale3D {
		Scale3D {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl MulAssign for Scale3D {
	fn mul_assign(&mut self, other: Scale3D) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
		self.z = self.z * other.z;
	}
}

impl <T> Mul<T> for Scale3D
	where T: Into<f32> {
	type Output = Scale3D;

	fn mul(self, value: T) -> Scale3D {
		let scale = value.into();

		Scale3D {
			x: self.x * scale,
			y: self.y * scale,
			z: self.z * scale,
		}
	}
}

impl <T> MulAssign<T> for Scale3D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x * scale;
		self.y = self.y * scale;
		self.z = self.z * scale;
	}
}

impl Div for Scale3D {
	type Output = Scale3D;

	fn div(self, other: Scale3D) -> Scale3D {
		Scale3D {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}

impl DivAssign for Scale3D {
	fn div_assign(&mut self, other: Scale3D) {
		self.x = self.x / other.x;
		self.y = self.y / other.y;
		self.z = self.z / other.z;
	}
}

impl <T> Div<T> for Scale3D
	where T: Into<f32> {
	type Output = Scale3D;

	fn div(self, value: T) -> Scale3D {
		let scale = value.into();

		Scale3D {
			x: self.x / scale,
			y: self.y / scale,
			z: self.z / scale,
		}
	}
}

impl <T> DivAssign<T> for Scale3D
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x / scale;
		self.y = self.y / scale;
		self.z = self.z / scale;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn scale_2d_new() {
		let scale2d = Scale2D::new();
		assert_eq!(scale2d, Scale2D { x: 0.0, y: 0.0 });
	}

    #[test]
	fn scale_2d_from() {
		let scale2d = Scale2D::from(120, 74);
		assert_eq!(scale2d, Scale2D { x: 120.0, y: 74.0 });
	}

    #[test]
	fn scale_2d_from_f32() {
		let scale2d = Scale2D::from_f32(120.0, 74.0);
		assert_eq!(scale2d, Scale2D { x: 120.0, y: 74.0 });
	}

	#[test]
	fn scale_2d_set() {
		let mut scale2d = Scale2D::new();
		scale2d.set(28.0, 25.0);
		assert_eq!(scale2d, Scale2D { x: 28.0, y: 25.0 });
	}

	#[test]
	fn scale_2d_self_add() {
		let mut scale2d = Scale2D::from(90, 100);
		scale2d.add_num(10.0, 10.0);
		assert_eq!(scale2d, Scale2D { x: 100.0, y: 110.0 });
	}

	#[test]
	fn scale_2d_self_sub() {
		let mut scale2d = Scale2D::from(90, 100);
		scale2d.sub_num(10.0, 10.0);
		assert_eq!(scale2d, Scale2D { x: 80.0, y: 90.0 });
	}

	#[test]
	fn scale_2d_self_mul() {
		let mut scale2d = Scale2D::from(50, 100);
		scale2d.mul_num(2.0, 1.0);
		assert_eq!(scale2d, Scale2D { x: 100.0, y: 100.0 });
	}

	#[test]
	fn scale_2d_self_div() {
		let mut scale2d = Scale2D::from(50, 100);
		scale2d.div_num(2.0, 1.0);
		assert_eq!(scale2d, Scale2D { x: 25.0, y: 100.0 });
	}

	#[test]
	fn scale_2d_add() {
		let scale1 = Scale2D::from(255, 0);
		let scale2 = Scale2D::from(0, 255);
		let scale3 = scale1 + scale2;
		assert_eq!(scale3, Scale2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn scale_2d_sub() {
		let scale1 = Scale2D::from(255, 0);
		let scale2 = Scale2D::from(255, 0);
		let scale3 = scale1 - scale2;
		assert_eq!(scale3, Scale2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_mul() {
		let scale1 = Scale2D::from(50, 0);
		let scale2 = Scale2D::from(3, 0);
		let scale3 = scale1 * scale2;
		assert_eq!(scale3, Scale2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_div() {
		let scale1 = Scale2D::from(100, 0);
		let scale2 = Scale2D::from(2, 1);
		let scale3 = scale1 / scale2;
		assert_eq!(scale3, Scale2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_mul_num() {
		let scale2d = Scale2D::from(50, 0);
		let scale2: Scale2D = scale2d * 2.0;
		assert_eq!(scale2, Scale2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_div_num() {
		let scale2d = Scale2D::from(100, 0);
		let scale2: Scale2D = scale2d / 2.0;
		assert_eq!(scale2, Scale2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_add_assign() {
		let mut scale1 = Scale2D::from(255, 0);
		let scale2 = Scale2D::from(0, 255);
		scale1 += scale2;
		assert_eq!(scale1, Scale2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn scale_2d_sub_assign() {
		let mut scale1 = Scale2D::from(255, 0);
		let scale2 = Scale2D::from(255, 0);
		scale1 -= scale2;
		assert_eq!(scale1, Scale2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_mul_assign() {
		let mut scale1 = Scale2D::from(50, 0);
		let scale2 = Scale2D::from(3, 0);
		scale1 *= scale2;
		assert_eq!(scale1, Scale2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_div_assign() {
		let mut scale1 = Scale2D::from(100, 0);
		let scale2 = Scale2D::from(2, 1);
		scale1 /= scale2;
		assert_eq!(scale1, Scale2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_mul_num_assign() {
		let mut scale2d = Scale2D::from(50, 0);
		scale2d *= 2.0;
		assert_eq!(scale2d, Scale2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn scale_2d_div_num_assign() {
		let mut scale2d = Scale2D::from(100, 0);
		scale2d /= 2.0;
		assert_eq!(scale2d, Scale2D { x: 50.0, y: 0.0 });
	}

    #[test]
	fn scale_3d_new() {
		let scale3d = Scale3D::new();
		assert_eq!(scale3d, Scale3D { x: 0.0, y: 0.0, z: 0.0 });
	}

    #[test]
	fn scale_3d_from() {
		let scale3d = Scale3D::from(120, 74, 33);
		assert_eq!(scale3d, Scale3D { x: 120.0, y: 74.0, z: 33.0 });
	}

    #[test]
	fn scale_3d_from_f32() {
		let scale3d = Scale3D::from_f32(120.0, 74.0, 33.0);
		assert_eq!(scale3d, Scale3D { x: 120.0, y: 74.0, z: 33.0 });
	}

	#[test]
	fn scale_3d_set() {
		let mut scale3d = Scale3D::new();
		scale3d.set(28.0, 25.0, 33.0);
		assert_eq!(scale3d, Scale3D { x: 28.0, y: 25.0, z: 33.0 });
	}

	#[test]
	fn scale_3d_self_add() {
		let mut scale3d = Scale3D::from(90, 100, 110);
		scale3d.add_num(10.0, 10.0, 10.0);
		assert_eq!(scale3d, Scale3D { x: 100.0, y: 110.0, z: 120.0 });
	}

	#[test]
	fn scale_3d_self_sub() {
		let mut scale3d = Scale3D::from(90, 100, 110);
		scale3d.sub_num(10.0, 10.0, 10.0);
		assert_eq!(scale3d, Scale3D { x: 80.0, y: 90.0, z: 100.0 });
	}

	#[test]
	fn scale_3d_self_mul() {
		let mut scale3d = Scale3D::from(50, 100, 25);
		scale3d.mul_num(2.0, 1.0, 3.0);
		assert_eq!(scale3d, Scale3D { x: 100.0, y: 100.0, z: 75.0 });
	}

	#[test]
	fn scale_3d_self_div() {
		let mut scale3d = Scale3D::from(50, 100, 75);
		scale3d.div_num(2.0, 1.0, 3.0);
		assert_eq!(scale3d, Scale3D { x: 25.0, y: 100.0, z: 25.0 });
	}

	#[test]
	fn scale_3d_add() {
		let scale1 = Scale3D::from(255, 0, 255);
		let scale2 = Scale3D::from(0, 255, 0);
		let scale3 = scale1 + scale2;
		assert_eq!(scale3, Scale3D { x: 255.0, y: 255.0, z: 255.0 });
	}

	#[test]
	fn scale_3d_sub() {
		let scale1 = Scale3D::from(255, 0, 255);
		let scale2 = Scale3D::from(255, 0, 255);
		let scale3 = scale1 - scale2;
		assert_eq!(scale3, Scale3D { x: 0.0, y: 0.0, z: 0.0 });
	}

	#[test]
	fn scale_3d_mul() {
		let scale1 = Scale3D::from(50, 0, 25);
		let scale2 = Scale3D::from(3, 0, 2);
		let scale3 = scale1 * scale2;
		assert_eq!(scale3, Scale3D { x: 150.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn scale_3d_div() {
		let scale1 = Scale3D::from(100, 0, 75);
		let scale2 = Scale3D::from(2, 1, 3);
		let scale3 = scale1 / scale2;
		assert_eq!(scale3, Scale3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn scale_3d_mul_num() {
		let scale3d = Scale3D::from(50, 0, 25);
		let scale2: Scale3D = scale3d * 2.0;
		assert_eq!(scale2, Scale3D { x: 100.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn scale_3d_div_num() {
		let scale3d = Scale3D::from(100, 0, 50);
		let scale2: Scale3D = scale3d / 2.0;
		assert_eq!(scale2, Scale3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn scale_3d_add_assign() {
		let mut scale1 = Scale3D::from(255, 0, 255);
		let scale2 = Scale3D::from(0, 255, 0);
		scale1 += scale2;
		assert_eq!(scale1, Scale3D { x: 255.0, y: 255.0, z: 255.0 });
	}

	#[test]
	fn scale_3d_sub_assign() {
		let mut scale1 = Scale3D::from(255, 0, 255);
		let scale2 = Scale3D::from(255, 0, 255);
		scale1 -= scale2;
		assert_eq!(scale1, Scale3D { x: 0.0, y: 0.0, z: 0.0 });
	}

	#[test]
	fn scale_3d_mul_assign() {
		let mut scale1 = Scale3D::from(50, 0, 25);
		let scale2 = Scale3D::from(3, 0, 2);
		scale1 *= scale2;
		assert_eq!(scale1, Scale3D { x: 150.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn scale_3d_div_assign() {
		let mut scale1 = Scale3D::from(100, 0, 75);
		let scale2 = Scale3D::from(2, 1, 3);
		scale1 /= scale2;
		assert_eq!(scale1, Scale3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn scale_3d_mul_num_assign() {
		let mut scale3d = Scale3D::from(50, 0, 25);
		scale3d *= 2.0;
		assert_eq!(scale3d, Scale3D { x: 100.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn scale_3d_div_num_assign() {
		let mut scale3d = Scale3D::from(100, 0, 50);
		scale3d /= 2.0;
		assert_eq!(scale3d, Scale3D { x: 50.0, y: 0.0, z: 25.0 });
	}
}