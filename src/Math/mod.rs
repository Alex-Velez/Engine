pub mod Matrix;

pub use PositionMath2D as Pos;
pub use RotationMath2D as Rot;
pub use RotationMath2DType as RotT;

use crate::{Camera2D, Window};

// Math Constants
pub const DEG_RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD_DEG: f32 = 180.0 / std::f32::consts::PI;
pub const TWO_PIE: f32 = std::f32::consts::PI * 2.000;
pub const DEG_RAD_F64: f64 = std::f64::consts::PI / 180.0;
pub const RAD_DEG_F64: f64 = 180.0 / std::f64::consts::PI;
pub const TWO_PIE_F64: f64 = std::f64::consts::PI * 2.000;

// Time Globals (unsafe)
// pub(crate) static mut DELTA_TIME: f64 = 0.0;
// pub(crate) static mut TOTAL_ELAPSED_SECONDS: f64 = 0.0;

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 - t) + end * t
}

pub struct Matrix4x4 {
    pub identity: [[f32; 4]; 4],
    pub translation: Vector3D,
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4x4 { identity, translation: Vector3D::new() }
    }

    pub fn is_identity(self) -> bool {
        (self.identity[0][0] == 1.0) && (self.identity[1][1] == 1.0) && (self.identity[2][2] == 1.0) && (self.identity[3][3] == 1.0) &&
        (self.identity[0][1] == 0.0) && (self.identity[0][2] == 0.0) && (self.identity[0][3] == 0.0) &&
        (self.identity[1][0] == 0.0) && (self.identity[1][2] == 0.0) && (self.identity[1][3] == 0.0) &&
        (self.identity[2][0] == 0.0) && (self.identity[2][1] == 0.0) && (self.identity[2][3] == 0.0) &&
        (self.identity[3][0] == 0.0) && (self.identity[3][1] == 0.0) && (self.identity[3][2] == 0.0)
    }

    pub fn mult(self, other: Matrix4x4) -> Matrix4x4 {
        let mut mat = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum = sum + self.identity[i][j] * other.identity[k][j];
                }
                mat.identity[i][j] = sum;
            }
        }
        mat
    }

    pub const fn array(self) -> [f32; 16] {
        [
            self.identity[0][0], self.identity[0][1], self.identity[0][2], self.identity[0][3],
            self.identity[1][0], self.identity[1][1], self.identity[1][2], self.identity[1][3],
            self.identity[2][0], self.identity[2][1], self.identity[2][2], self.identity[2][3],
            self.identity[3][0], self.identity[3][1], self.identity[3][2], self.identity[3][3],
        ]
    }
}

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

pub enum MatrixMath4D {

}

pub enum PositionMath2D {
    Add(f32, f32),
    Sub(f32, f32),
    Mul(f32, f32),
    Div(f32, f32),
    Mod(f32, f32),
    Pow(f32, f32),
}

pub enum RotationMath2D {
    Add(f32),
    Sub(f32),
    Mul(f32),
    Div(f32),
    Mod(f32),
    Pow(f32),
}

pub enum RotationMath2DType {
    Deg,
    Rad,
}

pub mod Test {
    pub mod Random {
        pub fn num(min: u32, max:u32) -> u32 {
            let mut num:u32 = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as u32;
            for x in [13,17,5,11,3,19,7].iter() {
                num ^= num << x;
                num ^= num >> x % 9;
            }
            (num % (max + 1)) + min
        }
    }
}

pub mod Unstable {
    pub fn calc(mut mmr1: i32, mut mmr2: i32) -> (i32, i32) {
    
        let gain = clamp(-((mmr1-mmr2)/10)+10, 0, 20);
        
        mmr1 += gain;
        mmr2 = (mmr2-gain).max(0);
        
        print!("+{}", gain);
        
        (mmr1, mmr2)
    }
    
    pub fn clamp(num: i32, min: i32, max: i32) -> i32 {
        if num < min {
            min
        } else if num > max {
            max
        } else {
            num
        }
    }
    
    pub fn lazy_mmr(mut mmr1: f32, mut mmr2: f32) -> (f32, f32) {
    
        mmr1 += 10.0;
        mmr2 -= 10.0;
        
        (mmr1, mmr2)
    }
}