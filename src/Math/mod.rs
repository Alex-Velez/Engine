pub(crate) mod vector;
pub(crate) mod matrix;

pub use vector::*;
pub use matrix::*;

// Math Constants
pub const DEG_RAD: f32 = std::f32::consts::PI / 180.0;
pub const RAD_DEG: f32 = 180.0 / std::f32::consts::PI;
pub const TWO_PIE: f32 = std::f32::consts::PI * 2.000;
pub const DEG_RAD_F64: f64 = std::f64::consts::PI / 180.0;
pub const RAD_DEG_F64: f64 = 180.0 / std::f64::consts::PI;
pub const TWO_PIE_F64: f64 = std::f64::consts::PI * 2.000;

// Time Globals (unsafe)
pub(crate) static mut DELTA_TIME: f64 = 0.0;
pub(crate) static mut TOTAL_ELAPSED_SECONDS: f64 = 0.0;

pub fn delta_time() -> f32 {
    unsafe { DELTA_TIME as f32 }
}

pub fn total_elapsed_seconds() -> f32 {
    unsafe { TOTAL_ELAPSED_SECONDS as f32 }
}

pub fn delta_time_f64() -> f64 {
    unsafe { DELTA_TIME }
}

pub fn total_elapsed_seconds_f64() -> f64 {
    unsafe { TOTAL_ELAPSED_SECONDS }
}

pub mod Debug {
    pub mod Random {
        pub fn range(min: u32, max:u32) -> u32 {
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
    pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start * (1.0 - t) + end * t
    }
}