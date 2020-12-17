pub(crate) static mut DELTA_TIME: f64 = 0.0;
pub(crate) static mut TOTAL_ELAPSED_SECONDS: f64 = 0.0;

pub fn DeltaTime() -> f64 {
	unsafe { DELTA_TIME }
}
pub fn TotalElapsedSeconds() -> f64 {
	unsafe { TOTAL_ELAPSED_SECONDS }
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
	start * (1.0 - t) + end * t
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