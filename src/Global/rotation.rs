use crate::Math;
use std::cmp::{Eq, PartialEq};
use std::convert::Into;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Rotation2D {
	degrees: f32,
	radians: f32,
}

impl Rotation2D {
	pub const fn new() -> Rotation2D {
		Rotation2D { degrees: 0.0, radians: 0.0 }
	}

    pub fn from<T: Into<f64> + Copy>(degrees: T) -> Rotation2D {
        Rotation2D {
            degrees: (degrees.into() as f32) % 360.0,
            radians: Math::DEG_RAD * ((degrees.into() as f32) % 360.0),
        }
	}

    pub fn from_deg<T: Into<f64> + Copy>(degrees: T) -> Rotation2D {
        Rotation2D {
            degrees: (degrees.into() as f32) % 360.0,
            radians: Math::DEG_RAD * ((degrees.into() as f32) % 360.0),
        }
	}

    pub fn from_rad<T: Into<f64> + Copy>(radians: T) -> Rotation2D {
        Rotation2D {
            degrees: Math::RAD_DEG * ((radians.into() as f32) % Math::TWO_PIE),
            radians: (radians.into() as f32) % Math::TWO_PIE,
        }
	}

	pub fn deg(self) -> f32 { self.degrees }

	pub fn rad(self) -> f32 { self.radians }

	pub fn set_deg(&mut self, degrees: f32) {
		self.degrees = degrees % 360.0;
		self.radians = Math::DEG_RAD * self.degrees;
	}

	pub fn set_rad(&mut self, radians: f32) {
		self.radians = radians % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * self.radians;
		
	}

	pub fn add_deg(&mut self, degrees: f32) {
		self.degrees = (self.degrees + degrees) % 360.0;
		self.radians = Math::DEG_RAD * self.degrees;
	}

	pub fn add_rad(&mut self, radians: f32) {
		self.radians = (self.radians + radians) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * self.radians;
	}

	pub fn sub_deg(&mut self, degrees: f32) {
		self.degrees = (self.degrees - degrees) % 360.0;
		self.radians = Math::DEG_RAD * self.degrees;
	}

	pub fn sub_rad(&mut self, radians: f32) {
		self.radians = (self.radians - radians) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * self.radians;
	}

	pub fn mul_deg(&mut self, scale: f32) {
		self.degrees = (self.degrees * scale) % 360.0;
		self.radians = Math::DEG_RAD * self.degrees;
	}

	pub fn mul_rad(&mut self, scale: f32) {
		self.radians = (self.radians * scale) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * self.radians;
	}

	pub fn div_deg(&mut self, scale: f32) {
		self.degrees = (self.degrees / scale) % 360.0;
		self.radians = Math::DEG_RAD * self.degrees;
	}

	pub fn div_rad(&mut self, scale: f32) {
		self.radians = (self.radians / scale) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * self.radians;
	}
}

impl Eq for Rotation2D {}

impl PartialEq for Rotation2D {
    fn eq(&self, other: &Rotation2D) -> bool {
        self.degrees == other.degrees && self.radians == other.radians
    }
}

impl Add for Rotation2D {
	type Output = Rotation2D;

	fn add(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees + other.degrees) % 360.0,
            radians: Math::DEG_RAD * ((self.degrees + other.degrees) % 360.0),
        }
	}
}

impl AddAssign for Rotation2D {
	fn add_assign(&mut self, other: Rotation2D) {
        self.degrees = (self.degrees + other.degrees) % 360.0;
        self.radians = Math::DEG_RAD * self.degrees;
	}
}

impl Sub for Rotation2D {
	type Output = Rotation2D;

	fn sub(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees - other.degrees) % 360.0,
            radians: Math::DEG_RAD * ((self.degrees - other.degrees) % 360.0),
        }
	}
}

impl SubAssign for Rotation2D {
	fn sub_assign(&mut self, other: Rotation2D) {
		self.degrees = (self.degrees - other.degrees) % 360.0;
        self.radians = Math::DEG_RAD * self.degrees;
	}
}

impl <T> Mul<T> for Rotation2D
	where T: Into<f32> {
	type Output = Rotation2D;

	fn mul(self, value: T) -> Rotation2D {
		let scale = value.into();

		Rotation2D {
            degrees: (self.degrees * scale) % 360.0,
            radians: (self.radians * scale) % Math::TWO_PIE,
        }
	}
}

impl <T> MulAssign<T> for Rotation2D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.degrees = (self.degrees * scale) % 360.0;
        self.radians = (self.radians * scale) % Math::TWO_PIE;
	}
}


impl <T> Div<T> for Rotation2D
	where T: Into<f32> {
	type Output = Rotation2D;

	fn div(self, value: T) -> Rotation2D {
		let scale = value.into();

		Rotation2D {
            degrees: (self.degrees / scale) % 360.0,
            radians: (self.radians / scale) % Math::TWO_PIE,
        }
	}
}

impl <T> DivAssign<T> for Rotation2D
	where T: Into<f32> {
	fn div_assign(&mut self, value: T) {
		let scale = value.into();

		self.degrees = (self.degrees / scale) % 360.0;
        self.radians = (self.radians / scale) % Math::TWO_PIE;
	}
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn rotation_new() {
		let rot = Rotation2D::new();
        assert_eq!(rot, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_from() {
        let rotation_def = Rotation2D::from(127);
        let rotation_fdef = Rotation2D::from(127.0);
        let rotation_u8 = Rotation2D::from(127u8);
        let rotation_u16 = Rotation2D::from(127u16);
        let rotation_u32 = Rotation2D::from(127u32);
        let rotation_i8 = Rotation2D::from(127i8);
        let rotation_i16 = Rotation2D::from(127i16);
        let rotation_i32 = Rotation2D::from(127i32);
        let rotation_f32 = Rotation2D::from(127.0f32);
        let rotation_f64 = Rotation2D::from(127.0f64);
        let rotation = Rotation2D { degrees: 127.0, radians: 127.0 * Math::DEG_RAD };
        assert_eq!(rotation_def, rotation);
        assert_eq!(rotation_fdef, rotation);
		assert_eq!(rotation_u8, rotation);
		assert_eq!(rotation_u16, rotation);
		assert_eq!(rotation_u32, rotation);
		assert_eq!(rotation_i8, rotation);
		assert_eq!(rotation_i16, rotation);
		assert_eq!(rotation_i32, rotation);
		assert_eq!(rotation_f32, rotation);
		assert_eq!(rotation_f64, rotation);
	}

	#[test]
	fn rotation_from_deg() {
        let rotation_def = Rotation2D::from_deg(127);
        let rotation_fdef = Rotation2D::from_deg(127.0);
        let rotation_u8 = Rotation2D::from_deg(127u8);
        let rotation_u16 = Rotation2D::from_deg(127u16);
        let rotation_u32 = Rotation2D::from_deg(127u32);
        let rotation_i8 = Rotation2D::from_deg(127i8);
        let rotation_i16 = Rotation2D::from_deg(127i16);
        let rotation_i32 = Rotation2D::from_deg(127i32);
        let rotation_f32 = Rotation2D::from_deg(127.0f32);
        let rotation_f64 = Rotation2D::from_deg(127.0f64);
        let rotation = Rotation2D { degrees: 127.0, radians: 127.0 * Math::DEG_RAD };
        assert_eq!(rotation_def, rotation);
        assert_eq!(rotation_fdef, rotation);
		assert_eq!(rotation_u8, rotation);
		assert_eq!(rotation_u16, rotation);
		assert_eq!(rotation_u32, rotation);
		assert_eq!(rotation_i8, rotation);
		assert_eq!(rotation_i16, rotation);
		assert_eq!(rotation_i32, rotation);
		assert_eq!(rotation_f32, rotation);
		assert_eq!(rotation_f64, rotation);
	}

	#[test]
	fn rotation_from_rad() {
        let rotation_def = Rotation2D::from_rad(3);
        let rotation_fdef = Rotation2D::from_rad(3.14159265358979323846264338327950288);
        let rotation_u8 = Rotation2D::from_rad(3u8);
        let rotation_u16 = Rotation2D::from_rad(3u16);
        let rotation_u32 = Rotation2D::from_rad(3u32);
        let rotation_i8 = Rotation2D::from_rad(3i8);
        let rotation_i16 = Rotation2D::from_rad(3i16);
        let rotation_i32 = Rotation2D::from_rad(3i32);
        let rotation_f32 = Rotation2D::from_rad(3.14159265358979323846264338327950288f32);
        let rotation_f64 = Rotation2D::from_rad(3.14159265358979323846264338327950288f64);
        let rotation1 = Rotation2D { degrees: 3.0 * Math::RAD_DEG, radians: 3.0 };
        let rotation2 = Rotation2D { degrees: 180.0, radians: std::f32::consts::PI };
        assert_eq!(rotation_def, rotation1);
        assert_eq!(rotation_fdef, rotation2);
		assert_eq!(rotation_u8, rotation1);
		assert_eq!(rotation_u16, rotation1);
		assert_eq!(rotation_u32, rotation1);
		assert_eq!(rotation_i8, rotation1);
		assert_eq!(rotation_i16, rotation1);
		assert_eq!(rotation_i32, rotation1);
		assert_eq!(rotation_f32, rotation2);
		assert_eq!(rotation_f64, rotation2);
	}

	#[test]
	fn rotation_deg() {
		let rot = Rotation2D::from_deg(180.0);
		let deg = rot.deg();
		assert_eq!(deg, 180.0);
	}

	#[test]
	fn rotation_rad() {
		let rot = Rotation2D::from_rad(std::f32::consts::PI);
		let rad = rot.rad();
		assert_eq!(rad, std::f32::consts::PI);
	}

	#[test]
	fn rotation_set_deg() {
		let mut rot = Rotation2D::new();
		rot.set_deg(180.0);
		assert_eq!(rot, Rotation2D { degrees: 180.0, radians: std::f32::consts::PI });
	}

	#[test]
	fn rotation_set_rad() {
		let mut rot = Rotation2D::new();
		rot.set_rad(std::f32::consts::PI);
		assert_eq!(rot, Rotation2D { degrees: 180.0, radians: std::f32::consts::PI });
	}

	#[test]
	fn rotation_self_add_deg() {
		let mut rot = Rotation2D::new();
		rot.add_deg(180.0);
		assert_eq!(rot, Rotation2D { degrees: 180.0, radians: std::f32::consts::PI });
	}

	#[test]
	fn rotation_self_add_rad() {
		let mut rot = Rotation2D::new();
		rot.add_rad(std::f32::consts::PI);
		assert_eq!(rot, Rotation2D { degrees: 180.0, radians: std::f32::consts::PI });
	}

	#[test]
	fn rotation_self_sub_deg() {
		let mut rot = Rotation2D::from_deg(180.0);
		rot.sub_deg(180.0);
		assert_eq!(rot, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_self_sub_rad() {
		let mut rot = Rotation2D::from_rad(std::f32::consts::PI);
		rot.sub_rad(std::f32::consts::PI);
		assert_eq!(rot, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_self_mul_deg() {
		let mut rot = Rotation2D::from_deg(180.0);
		rot.mul_deg(2.0);
		assert_eq!(rot, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_self_mul_rad() {
		let mut rot = Rotation2D::from_rad(std::f32::consts::PI);
		rot.mul_rad(2.0);
		assert_eq!(rot, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_self_div_deg() {
		let mut rot = Rotation2D::from_deg(180.0);
		rot.div_deg(2.0);
		assert_eq!(rot, Rotation2D { degrees: 90.0, radians: std::f32::consts::PI / 2.0 });
	}

	#[test]
	fn rotation_self_div_rad() {
		let mut rot = Rotation2D::from_rad(std::f32::consts::PI);
		rot.div_rad(2.0);
		assert_eq!(rot, Rotation2D { degrees: 90.0, radians: std::f32::consts::PI / 2.0 });
	}

	#[test]
	fn rotation_add() {
		let rot1 = Rotation2D::from_deg(180.0);
		let rot2 = Rotation2D::from_deg(180.0);
		let rot3 = rot1 + rot2;
		assert_eq!(rot3, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_sub() {
		let rot1 = Rotation2D::from_deg(180.0);
		let rot2 = Rotation2D::from_deg(180.0);
		let rot3 = rot1 - rot2;
		assert_eq!(rot3, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_mul_num() {
		let rot1 = Rotation2D::from_deg(180.0);
		let rot2 = rot1 * 2.0;
		assert_eq!(rot2, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_div_num() {
		let rot1 = Rotation2D::from_deg(180.0);
		let rot2 = rot1 / 2.0;
		assert_eq!(rot2, Rotation2D { degrees: 90.0, radians: std::f32::consts::PI / 2.0 });
	}

	#[test]
	fn rotation_add_assign() {
		let mut rot1 = Rotation2D::from_deg(180.0);
		let rot2 = Rotation2D::from_deg(180.0);
		rot1 += rot2;
		assert_eq!(rot1, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_sub_assign() {
		let mut rot1 = Rotation2D::from_deg(180.0);
		let rot2 = Rotation2D::from_deg(180.0);
		rot1 -= rot2;
		assert_eq!(rot1, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_mul_num_assign() {
		let mut rot1 = Rotation2D::from_deg(180.0);
		rot1 *= 2.0;
		assert_eq!(rot1, Rotation2D { degrees: 0.0, radians: 0.0 });
	}

	#[test]
	fn rotation_div_num_assign() {
		let mut rot1 = Rotation2D::from_deg(180.0);
		rot1 /= 2.0;
		assert_eq!(rot1, Rotation2D { degrees: 90.0, radians: std::f32::consts::PI / 2.0 });
	}
}