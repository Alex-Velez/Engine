use std::cmp::{Eq, PartialEq};
use std::convert::Into;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy, Debug)]
pub struct Size2D {
	pub x: f32,
	pub y: f32,
}

impl Size2D {
	pub const fn new() -> Size2D {
		Size2D { x: 0.0, y: 0.0 }
	}

    pub fn from<T: Into<f64>>(x: T, y: T) -> Size2D {
        Size2D { x: x.into() as f32, y: y.into() as f32 }
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

impl Eq for Size2D {}

impl PartialEq for Size2D {
    fn eq(&self, other: &Size2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Size2D {
	type Output = Size2D;

	fn add(self, other: Size2D) -> Size2D {
		Size2D {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl AddAssign for Size2D {
	fn add_assign(&mut self, other: Size2D) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
	}
}

impl Sub for Size2D {
	type Output = Size2D;

	fn sub(self, other: Size2D) -> Size2D {
		Size2D {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl SubAssign for Size2D {
	fn sub_assign(&mut self, other: Size2D) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
	}
}

impl Mul for Size2D {
	type Output = Size2D;

	fn mul(self, other: Size2D) -> Size2D {
		Size2D {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl MulAssign for Size2D {
	fn mul_assign(&mut self, other: Size2D) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
	}
}

impl <T> Mul<T> for Size2D
	where T: Into<f32> {
	type Output = Size2D;

	fn mul(self, value: T) -> Size2D {
		let scale = value.into();

		Size2D {
			x: self.x * scale,
			y: self.y * scale,
		}
	}
}

impl <T> MulAssign<T> for Size2D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x * scale;
		self.y = self.y * scale;
	}
}

impl Div for Size2D {
	type Output = Size2D;

	fn div(self, other: Size2D) -> Size2D {
		Size2D {
			x: self.x / other.x,
			y: self.y / other.y,
		}
	}
}

impl DivAssign for Size2D {
	fn div_assign(&mut self, other: Size2D) {
		self.x = self.x / other.x;
		self.y = self.y / other.y;
	}
}

impl <T> Div<T> for Size2D
	where T: Into<f32> {
	type Output = Size2D;

	fn div(self, value: T) -> Size2D {
		let scale = value.into();

		Size2D {
			x: self.x / scale,
			y: self.y / scale,
		}
	}
}

impl <T> DivAssign<T> for Size2D
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x / scale;
		self.y = self.y / scale;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Size3D {
	pub x: f32,
	pub y: f32,
    pub z: f32,
}

impl Size3D {
	pub const fn new() -> Size3D {
		Size3D { x: 0.0, y: 0.0, z: 0.0 }
	}

    pub fn from<T: Into<f64>>(x: T, y: T, z: T) -> Size3D {
        Size3D { x: x.into() as f32, y: y.into() as f32, z: z.into() as f32 }
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

impl Eq for Size3D {}

impl PartialEq for Size3D {
    fn eq(&self, other: &Size3D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Size3D {
	type Output = Size3D;

	fn add(self, other: Size3D) -> Size3D {
		Size3D {
			x: self.x + other.x,
			y: self.y + other.y,
            z: self.z + other.z,
		}
	}
}

impl AddAssign for Size3D {
	fn add_assign(&mut self, other: Size3D) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
		self.z = self.z + other.z;
	}
}

impl Sub for Size3D {
	type Output = Size3D;

	fn sub(self, other: Size3D) -> Size3D {
		Size3D {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl SubAssign for Size3D {
	fn sub_assign(&mut self, other: Size3D) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
		self.z = self.z - other.z;
	}
}

impl Mul for Size3D {
	type Output = Size3D;

	fn mul(self, other: Size3D) -> Size3D {
		Size3D {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl MulAssign for Size3D {
	fn mul_assign(&mut self, other: Size3D) {
		self.x = self.x * other.x;
		self.y = self.y * other.y;
		self.z = self.z * other.z;
	}
}

impl <T> Mul<T> for Size3D
	where T: Into<f32> {
	type Output = Size3D;

	fn mul(self, value: T) -> Size3D {
		let scale = value.into();

		Size3D {
			x: self.x * scale,
			y: self.y * scale,
			z: self.z * scale,
		}
	}
}

impl <T> MulAssign<T> for Size3D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x = self.x * scale;
		self.y = self.y * scale;
		self.z = self.z * scale;
	}
}

impl Div for Size3D {
	type Output = Size3D;

	fn div(self, other: Size3D) -> Size3D {
		Size3D {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}
}

impl DivAssign for Size3D {
	fn div_assign(&mut self, other: Size3D) {
		self.x = self.x / other.x;
		self.y = self.y / other.y;
		self.z = self.z / other.z;
	}
}

impl <T> Div<T> for Size3D
	where T: Into<f32> {
	type Output = Size3D;

	fn div(self, value: T) -> Size3D {
		let scale = value.into();

		Size3D {
			x: self.x / scale,
			y: self.y / scale,
			z: self.z / scale,
		}
	}
}

impl <T> DivAssign<T> for Size3D
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
	fn size_2d_new() {
		let size2d = Size2D::new();
		assert_eq!(size2d, Size2D { x: 0.0, y: 0.0 });
	}

    #[test]
	fn size_2d_from() {
        let size_2d_def = Size2D::from(120, 74);
        let size_2d_fdef = Size2D::from(120.0, 74.0);
        let size_2d_u8 = Size2D::from(120u8, 74u8);
        let size_2d_u16 = Size2D::from(120u16, 74u16);
        let size_2d_u32 = Size2D::from(120u32, 74u32);
        let size_2d_i8 = Size2D::from(120i8, 74i8);
        let size_2d_i16 = Size2D::from(120i16, 74i16);
        let size_2d_i32 = Size2D::from(120i32, 74i32);
        let size_2d_f32 = Size2D::from(120.0f32, 74.0f32);
        let size_2d_f64 = Size2D::from(120.0f64, 74.0f64);
        let size_2d = Size2D { x: 120.0, y: 74.0 };
        assert_eq!(size_2d_def, size_2d);
        assert_eq!(size_2d_fdef, size_2d);
		assert_eq!(size_2d_u8, size_2d);
		assert_eq!(size_2d_u16, size_2d);
		assert_eq!(size_2d_u32, size_2d);
		assert_eq!(size_2d_i8, size_2d);
		assert_eq!(size_2d_i16, size_2d);
		assert_eq!(size_2d_i32, size_2d);
		assert_eq!(size_2d_f32, size_2d);
		assert_eq!(size_2d_f64, size_2d);
	}

	#[test]
	fn size_2d_set() {
		let mut size2d = Size2D::new();
		size2d.set(28.0, 25.0);
		assert_eq!(size2d, Size2D { x: 28.0, y: 25.0 });
	}

	#[test]
	fn size_2d_self_add() {
		let mut size2d = Size2D::from(90, 100);
		size2d.add_num(10.0, 10.0);
		assert_eq!(size2d, Size2D { x: 100.0, y: 110.0 });
	}

	#[test]
	fn size_2d_self_sub() {
		let mut size2d = Size2D::from(90, 100);
		size2d.sub_num(10.0, 10.0);
		assert_eq!(size2d, Size2D { x: 80.0, y: 90.0 });
	}

	#[test]
	fn size_2d_self_mul() {
		let mut size2d = Size2D::from(50, 100);
		size2d.mul_num(2.0, 1.0);
		assert_eq!(size2d, Size2D { x: 100.0, y: 100.0 });
	}

	#[test]
	fn size_2d_self_div() {
		let mut size2d = Size2D::from(50, 100);
		size2d.div_num(2.0, 1.0);
		assert_eq!(size2d, Size2D { x: 25.0, y: 100.0 });
	}

	#[test]
	fn size_2d_add() {
		let size1 = Size2D::from(255, 0);
		let size2 = Size2D::from(0, 255);
		let size3 = size1 + size2;
		assert_eq!(size3, Size2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn size_2d_sub() {
		let size1 = Size2D::from(255, 0);
		let size2 = Size2D::from(255, 0);
		let size3 = size1 - size2;
		assert_eq!(size3, Size2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn size_2d_mul() {
		let size1 = Size2D::from(50, 0);
		let size2 = Size2D::from(3, 0);
		let size3 = size1 * size2;
		assert_eq!(size3, Size2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn size_2d_div() {
		let size1 = Size2D::from(100, 0);
		let size2 = Size2D::from(2, 1);
		let size3 = size1 / size2;
		assert_eq!(size3, Size2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn size_2d_mul_num() {
		let size2d = Size2D::from(50, 0);
		let size2: Size2D = size2d * 2.0;
		assert_eq!(size2, Size2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn size_2d_div_num() {
		let size2d = Size2D::from(100, 0);
		let size2: Size2D = size2d / 2.0;
		assert_eq!(size2, Size2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn size_2d_add_assign() {
		let mut size1 = Size2D::from(255, 0);
		let size2 = Size2D::from(0, 255);
		size1 += size2;
		assert_eq!(size1, Size2D { x: 255.0, y: 255.0 });
	}

	#[test]
	fn size_2d_sub_assign() {
		let mut size1 = Size2D::from(255, 0);
		let size2 = Size2D::from(255, 0);
		size1 -= size2;
		assert_eq!(size1, Size2D { x: 0.0, y: 0.0 });
	}

	#[test]
	fn size_2d_mul_assign() {
		let mut size1 = Size2D::from(50, 0);
		let size2 = Size2D::from(3, 0);
		size1 *= size2;
		assert_eq!(size1, Size2D { x: 150.0, y: 0.0 });
	}

	#[test]
	fn size_2d_div_assign() {
		let mut size1 = Size2D::from(100, 0);
		let size2 = Size2D::from(2, 1);
		size1 /= size2;
		assert_eq!(size1, Size2D { x: 50.0, y: 0.0 });
	}

	#[test]
	fn size_2d_mul_num_assign() {
		let mut size2d = Size2D::from(50, 0);
		size2d *= 2.0;
		assert_eq!(size2d, Size2D { x: 100.0, y: 0.0 });
	}

	#[test]
	fn size_2d_div_num_assign() {
		let mut size2d = Size2D::from(100, 0);
		size2d /= 2.0;
		assert_eq!(size2d, Size2D { x: 50.0, y: 0.0 });
	}

    #[test]
	fn size_3d_new() {
		let size3d = Size3D::new();
		assert_eq!(size3d, Size3D { x: 0.0, y: 0.0, z: 0.0 });
	}

    #[test]
	fn size_3d_from() {
		let size_3d_def = Size3D::from(120, 74, 33);
        let size_3d_fdef = Size3D::from(120.0, 74.0, 33.0);
        let size_3d_u8 = Size3D::from(120u8, 74u8, 33u8);
        let size_3d_u16 = Size3D::from(120u16, 74u16, 33u16);
        let size_3d_u32 = Size3D::from(120u32, 74u32, 33u32);
        let size_3d_i8 = Size3D::from(120i8, 74i8, 33i8);
        let size_3d_i16 = Size3D::from(120i16, 74i16, 33i16);
        let size_3d_i32 = Size3D::from(120i32, 74i32, 33i32);
        let size_3d_f32 = Size3D::from(120.0f32, 74.0f32, 33.0f32);
        let size_3d_f64 = Size3D::from(120.0f64, 74.0f64, 33.0f64);
        let size_3d = Size3D { x: 120.0, y: 74.0, z: 33.0 };
        assert_eq!(size_3d_def, size_3d);
        assert_eq!(size_3d_fdef, size_3d);
		assert_eq!(size_3d_u8, size_3d);
		assert_eq!(size_3d_u16, size_3d);
		assert_eq!(size_3d_u32, size_3d);
		assert_eq!(size_3d_i8, size_3d);
		assert_eq!(size_3d_i16, size_3d);
		assert_eq!(size_3d_i32, size_3d);
		assert_eq!(size_3d_f32, size_3d);
		assert_eq!(size_3d_f64, size_3d);
	}

	#[test]
	fn size_3d_set() {
		let mut size3d = Size3D::new();
		size3d.set(28.0, 25.0, 33.0);
		assert_eq!(size3d, Size3D { x: 28.0, y: 25.0, z: 33.0 });
	}

	#[test]
	fn size_3d_self_add() {
		let mut size3d = Size3D::from(90, 100, 110);
		size3d.add_num(10.0, 10.0, 10.0);
		assert_eq!(size3d, Size3D { x: 100.0, y: 110.0, z: 120.0 });
	}

	#[test]
	fn size_3d_self_sub() {
		let mut size3d = Size3D::from(90, 100, 110);
		size3d.sub_num(10.0, 10.0, 10.0);
		assert_eq!(size3d, Size3D { x: 80.0, y: 90.0, z: 100.0 });
	}

	#[test]
	fn size_3d_self_mul() {
		let mut size3d = Size3D::from(50, 100, 25);
		size3d.mul_num(2.0, 1.0, 3.0);
		assert_eq!(size3d, Size3D { x: 100.0, y: 100.0, z: 75.0 });
	}

	#[test]
	fn size_3d_self_div() {
		let mut size3d = Size3D::from(50, 100, 75);
		size3d.div_num(2.0, 1.0, 3.0);
		assert_eq!(size3d, Size3D { x: 25.0, y: 100.0, z: 25.0 });
	}

	#[test]
	fn size_3d_add() {
		let size1 = Size3D::from(255, 0, 255);
		let size2 = Size3D::from(0, 255, 0);
		let size3 = size1 + size2;
		assert_eq!(size3, Size3D { x: 255.0, y: 255.0, z: 255.0 });
	}

	#[test]
	fn size_3d_sub() {
		let size1 = Size3D::from(255, 0, 255);
		let size2 = Size3D::from(255, 0, 255);
		let size3 = size1 - size2;
		assert_eq!(size3, Size3D { x: 0.0, y: 0.0, z: 0.0 });
	}

	#[test]
	fn size_3d_mul() {
		let size1 = Size3D::from(50, 0, 25);
		let size2 = Size3D::from(3, 0, 2);
		let size3 = size1 * size2;
		assert_eq!(size3, Size3D { x: 150.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn size_3d_div() {
		let size1 = Size3D::from(100, 0, 75);
		let size2 = Size3D::from(2, 1, 3);
		let size3 = size1 / size2;
		assert_eq!(size3, Size3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn size_3d_mul_num() {
		let size3d = Size3D::from(50, 0, 25);
		let size2: Size3D = size3d * 2.0;
		assert_eq!(size2, Size3D { x: 100.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn size_3d_div_num() {
		let size3d = Size3D::from(100, 0, 50);
		let size2: Size3D = size3d / 2.0;
		assert_eq!(size2, Size3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn size_3d_add_assign() {
		let mut size1 = Size3D::from(255, 0, 255);
		let size2 = Size3D::from(0, 255, 0);
		size1 += size2;
		assert_eq!(size1, Size3D { x: 255.0, y: 255.0, z: 255.0 });
	}

	#[test]
	fn size_3d_sub_assign() {
		let mut size1 = Size3D::from(255, 0, 255);
		let size2 = Size3D::from(255, 0, 255);
		size1 -= size2;
		assert_eq!(size1, Size3D { x: 0.0, y: 0.0, z: 0.0 });
	}

	#[test]
	fn size_3d_mul_assign() {
		let mut size1 = Size3D::from(50, 0, 25);
		let size2 = Size3D::from(3, 0, 2);
		size1 *= size2;
		assert_eq!(size1, Size3D { x: 150.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn size_3d_div_assign() {
		let mut size1 = Size3D::from(100, 0, 75);
		let size2 = Size3D::from(2, 1, 3);
		size1 /= size2;
		assert_eq!(size1, Size3D { x: 50.0, y: 0.0, z: 25.0 });
	}

	#[test]
	fn size_3d_mul_num_assign() {
		let mut size3d = Size3D::from(50, 0, 25);
		size3d *= 2.0;
		assert_eq!(size3d, Size3D { x: 100.0, y: 0.0, z: 50.0 });
	}

	#[test]
	fn size_3d_div_num_assign() {
		let mut size3d = Size3D::from(100, 0, 50);
		size3d /= 2.0;
		assert_eq!(size3d, Size3D { x: 50.0, y: 0.0, z: 25.0 });
	}
}