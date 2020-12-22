use crate::{ Scale3D, Rotation2D,
	Math::{Matrix4x4, Vector3D}
};

impl Matrix4x4 {
	pub const fn create_scale(scale: Scale3D) -> Matrix4x4 {
		let mut identity: [[f32; 4]; 4] = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		identity[0][0] = scale.x;
		identity[0][1] = 0.0;
		identity[0][2] = 0.0;
		identity[0][3] = 0.0;
		identity[1][0] = 0.0;
		identity[1][1] = scale.y;
		identity[1][2] = 0.0;
		identity[1][3] = 0.0;
		identity[2][0] = 0.0;
		identity[2][1] = 0.0;
		identity[2][2] = scale.z;
		identity[2][3] = 0.0;
		identity[3][0] = 0.0;
		identity[3][1] = 0.0;
		identity[3][2] = 0.0;
		identity[3][3] = 1.0;

		Matrix4x4 { identity, translation: Vector3D::new() }
	}


	pub fn create_orthographic_off_center(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4x4{
		let mut identity: [[f32; 4]; 4] = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		identity[0][0] = 2.0 / (right - left);
		identity[0][1] = 0.0;
		identity[0][2] = 0.0;
		identity[0][3] = 0.0;

		identity[1][1] = 2.0 / (top - bottom);
		identity[1][0] = 0.0;
		identity[1][2] = 0.0;
		identity[1][3] = 0.0;

		identity[2][2] = 1.0 / (near - far);
		identity[2][0] = 0.0;
		identity[2][1] = 0.0;
		identity[2][3] = 0.0;

		identity[3][0] = (left + right) / (left - right);
		identity[3][1] = (top + bottom) / (bottom - top);
		identity[3][2] = near / (near - far);
		identity[3][3] = 1.0;
		
		Matrix4x4 { identity, translation: Vector3D::new() }
	}

	pub fn create_tranlation(x: f32, y: f32, z: f32) -> Matrix4x4 {
		let mut identity: [[f32; 4]; 4] = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		identity[0][0] = 1.0;
		identity[0][1] = 0.0;
		identity[0][2] = 0.0;
		identity[0][3] = 0.0;
		identity[1][0] = 0.0;
		identity[1][1] = 1.0;
		identity[1][2] = 0.0;
		identity[1][3] = 0.0;
		identity[2][0] = 0.0;
		identity[2][1] = 0.0;
		identity[2][2] = 1.0;
		identity[2][3] = 0.0;

		identity[3][0] = x;
		identity[3][1] = y;
		identity[3][2] = z;
		identity[3][3] = 1.0;

		Matrix4x4 { identity, translation: Vector3D::new() }
	}

	pub fn create_rotation_z(radians: Rotation2D) -> Matrix4x4 {
		let mut identity: [[f32; 4]; 4] = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		let c: f32 = radians.get_rad().cos();
		let s: f32 = radians.get_rad().sin();

		// [  c  s  0  0 ]
		// [ -s  c  0  0 ]
		// [  0  0  1  0 ]
		// [  0  0  0  1 ]

		identity[0][0] = c;
		identity[0][1] = s;
		identity[0][2] = 0.0;
		identity[0][3] = 0.0;
		identity[1][0] = -s;
		identity[1][1] = c;
		identity[1][2] = 0.0;
		identity[1][3] = 0.0;
		identity[2][0] = 0.0;
		identity[2][1] = 0.0;
		identity[2][2] = 1.0;
		identity[2][3] = 0.0;
		identity[3][0] = 0.0;
		identity[3][1] = 0.0;
		identity[3][2] = 0.0;
		identity[3][3] = 1.0;

		Matrix4x4 { identity, translation: Vector3D::new() }
    }
}