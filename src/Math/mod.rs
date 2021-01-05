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

pub fn DeltaTime() -> f32 {
    unsafe { DELTA_TIME as f32 }
}

pub fn TotalElapsedSeconds() -> f32 {
    unsafe { TOTAL_ELAPSED_SECONDS as f32 }
}

pub fn DeltaTimef64() -> f64 {
    unsafe { DELTA_TIME }
}

pub fn TotalElapsedSecondsf64() -> f64 {
    unsafe { TOTAL_ELAPSED_SECONDS }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 - t) + end * t
}

pub enum MatrixMath4D {

}

pub mod Debug {
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
    pub fn calc(mut mmr1: f32, mut mmr2: f32) -> (f32, f32) {

        let gain = clamp(-((mmr1-mmr2)/10.0)+10.0, 0.0, 20.0);

        mmr1 += gain;
        mmr2 = (mmr2-gain).max(0.0);

        print!("+{}", gain);

        (mmr1, mmr2)
    }

    pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
        assert!(min <= max);
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