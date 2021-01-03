use crate::Math;

use std::convert::{From, Into};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
pub struct Rotation2D {
	degrees: f32,
	radians: f32,
}

impl From<i32> for Rotation2D {
    fn from(degrees: i32) -> Self {
        Rotation2D { degrees: degrees as f32, radians: Math::DEG_RAD * (degrees as f32) }
    }
}

impl From<f32> for Rotation2D {
    fn from(degrees: f32) -> Self {
        Rotation2D { degrees, radians: Math::DEG_RAD * degrees }
    }
}

impl Rotation2D {
	pub const fn new() -> Rotation2D {
		Rotation2D { degrees: 0.0, radians: 0.0 }
	}

	pub fn from_deg(degrees: f32) -> Rotation2D {
		Rotation2D { degrees, radians: Math::DEG_RAD * degrees }
	}

	pub fn from_rad(radians: f32) -> Rotation2D {
		Rotation2D { degrees: Math::RAD_DEG * radians, radians }
	}

	pub fn deg(self) -> f32 { self.degrees }

	pub fn rad(self) -> f32 { self.radians }

	pub fn set_deg(&mut self, degrees: f32) {
		self.degrees = degrees;
		self.radians = Math::DEG_RAD * degrees;
	}

	pub fn set_rad(&mut self, radians: f32) {
		self.degrees = Math::RAD_DEG * radians;
		self.radians = radians;
	}

	pub fn add_deg(&mut self, degrees: f32) {
		self.degrees = (self.degrees + degrees) % 360.0;
		self.radians = Math::DEG_RAD * degrees;
	}

	pub fn add_rad(&mut self, radians: f32) {
		self.radians = (self.radians + radians) % Math::TWO_PIE;
		self.degrees = Math::RAD_DEG * radians;
	}
}

impl Add for Rotation2D {
	type Output = Rotation2D;

	fn add(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees + other.degrees) % 360.0,
            radians: (self.radians + other.radians) % Math::TWO_PIE,
        }
	}
}

impl AddAssign for Rotation2D {
	fn add_assign(&mut self, other: Rotation2D) {
        self.degrees = (self.degrees + other.degrees) % 360.0;
        self.radians = (self.radians + other.radians) % Math::TWO_PIE;
	}
}

impl Sub for Rotation2D {
	type Output = Rotation2D;

	fn sub(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees - other.degrees) % 360.0,
            radians: (self.radians - other.radians) % Math::TWO_PIE,
        }
	}
}

impl SubAssign for Rotation2D {
	fn sub_assign(&mut self, other: Rotation2D) {
		self.degrees = (self.degrees - other.degrees) % 360.0;
        self.radians = (self.radians - other.radians) % Math::TWO_PIE;
	}
}

impl Mul for Rotation2D {
	type Output = Rotation2D;

	fn mul(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees * other.degrees) % 360.0,
            radians: (self.radians * other.radians) % Math::TWO_PIE,
        }
	}
}

impl MulAssign for Rotation2D {
	fn mul_assign(&mut self, other: Rotation2D) {
		self.degrees = (self.degrees * other.degrees) % 360.0;
        self.radians = (self.radians * other.radians) % Math::TWO_PIE;
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

impl Div for Rotation2D {
	type Output = Rotation2D;

	fn div(self, other: Rotation2D) -> Rotation2D {
		Rotation2D {
            degrees: (self.degrees / other.degrees) % 360.0,
            radians: (self.radians / other.radians) % Math::TWO_PIE,
        }
	}
}

impl DivAssign for Rotation2D {
	fn div_assign(&mut self, other: Rotation2D) {
		self.degrees = (self.degrees / other.degrees) % 360.0;
        self.radians = (self.radians / other.radians) % Math::TWO_PIE;
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


#[derive(Default)]
pub struct Employee {
    pub emp_code: i32,
    pub emp_name: String,
    emp_salary: f64,
}

impl Employee {
    pub fn new() -> Employee {
        Employee {
            emp_code: 101,
            emp_name: "Amita".to_string(),
            emp_salary: 0.0
        }
    }

    pub fn salary(self) -> f64 {
        self.emp_salary
    }

    pub fn set_salary(&mut self,emp_salary: f64) {
        self.emp_salary = emp_salary;
    }
}