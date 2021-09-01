// This file is modified code from 'rust-vector' (https://github.com/Wiseluster/rust-vector) created by Wiseluster
use std::cmp::{Eq, PartialEq};
use std::convert::{From, Into};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Rem, RemAssign, Neg, Not};

#[derive(Copy, Clone, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub const ZERO: Vector2D = Vector2D { x: 0.0, y: 0.0 };

    pub const fn new() -> Vector2D {
        Vector2D {x: 0.0, y: 0.0}
    }

    pub const fn from(x: f32, y: f32) -> Vector2D {
        Vector2D {x, y}
    }

    pub fn set(mut self, x: f32, y: f32) -> Vector2D {
        self.x = x;
        self.y = y;
        self
    }
}

impl Eq for Vector2D {}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Vector2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Vector2D {
	type Output = Vector2D;

	fn add(self, other: Vector2D) -> Vector2D {
		Vector2D {x: self.x + other.x, y: self.y + other.y}
	}
}

impl AddAssign for Vector2D {
	fn add_assign(&mut self, other: Vector2D) {
		self.x += other.x;
		self.y += other.y;
	}
}

impl Sub for Vector2D {
	type Output = Vector2D;

	fn sub(self, other: Vector2D) -> Vector2D {
		Vector2D {x: self.x - other.x, y: self.y - other.y}
	}
}

impl SubAssign for Vector2D {
	fn sub_assign(&mut self, other: Vector2D) {
		self.x -= other.x;
		self.y -= other.y;
	}
}

impl Mul for Vector2D {
	type Output = f32;

	fn mul(self, other: Vector2D) -> f32 {
		self.x * other.x + self.y * other.y
	}
}

impl <T> Mul<T> for Vector2D
	where T: Into<f32> {
	type Output = Vector2D;

	fn mul(self, value: T) -> Vector2D {
		let scale = value.into();

		Vector2D {x: self.x * scale, y: self.y * scale}
	}
}

impl <T> MulAssign<T> for Vector2D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x *= scale;
		self.y *= scale;
	}
}

impl Rem for Vector2D {
	type Output = Vector2D;

	fn rem(self, other: Vector2D) -> Vector2D {
		Vector2D {x: self.x % other.x, y: self.y % other.y}
	}
}

impl RemAssign for Vector2D {
	fn rem_assign(&mut self, other: Vector2D) {
		self.x %= other.x;
		self.y %= other.y;
	}
}

impl Neg for Vector2D {
	type Output = Vector2D;

	fn neg(self) -> Vector2D {
		Vector2D {x: -self.x, y: -self.y}
	}
}

impl Not for Vector2D {
	type Output = f32;

	fn not(self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub const fn new() -> Vector3D {
        Vector3D {x: 0.0, y: 0.0, z: 0.0}
    }

    pub const fn from(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {x, y, z}
    }

    pub fn set(mut self, x: f32, y: f32, z: f32) -> Vector3D {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
}

impl Add for Vector3D {
	type Output = Vector3D;

	fn add(self, other: Vector3D) -> Vector3D {
		Vector3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}

impl AddAssign for Vector3D {
	fn add_assign(&mut self, other: Vector3D) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

impl Sub for Vector3D {
	type Output = Vector3D;

	fn sub(self, other: Vector3D) -> Vector3D {
		Vector3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
	}
}

impl SubAssign for Vector3D {
	fn sub_assign(&mut self, other: Vector3D) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
	}
}

impl Mul for Vector3D {
	type Output = f32;

	fn mul(self, other: Vector3D) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

impl <T> Mul<T> for Vector3D
	where T: Into<f32> {
	type Output = Vector3D;

	fn mul(self, value: T) -> Vector3D {
		let scale = value.into();

		Vector3D {x: self.x * scale, y: self.y * scale, z: self.z * scale}
	}
}

impl <T> MulAssign<T> for Vector3D
	where T: Into<f32> {
	fn mul_assign(&mut self, value: T) {
		let scale = value.into();

		self.x *= scale;
		self.y *= scale;
		self.z *= scale;
	}
}

impl Rem for Vector3D {
	type Output = Vector3D;

	fn rem(self, other: Vector3D) -> Vector3D {
		Vector3D {x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x}
	}
}

impl RemAssign for Vector3D {
	fn rem_assign(&mut self, other: Vector3D) {
		let x = self.y * other.z - self.z * other.y;
		let y = self.z * other.x - self.x * other.z;
		let z = self.x * other.y - self.y * other.x;

		self.x = x;
		self.y = y;
		self.z = z;
	}
}

impl Neg for Vector3D {
	type Output = Vector3D;

	fn neg(self) -> Vector3D {
		Vector3D {x: -self.x, y: -self.y, z: -self.z}
	}
}

impl Not for Vector3D {
	type Output = f32;

	fn not(self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
}