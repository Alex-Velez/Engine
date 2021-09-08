use std::cmp::{Eq, PartialEq};
use crate::{Math::{Vector2D, Matrix4x4}, Window, Scale3D};

#[derive(Copy, Clone, Debug)]
pub struct Camera2D {
	pub focus_position: Vector2D,
	pub zoom: f32,
}

impl Camera2D {
	pub const fn new() -> Camera2D {
		Camera2D { focus_position: Vector2D::new(), zoom: 1.0 }
	}

    pub fn from<T: Into<f64>>(focus_position: Vector2D, zoom: T) -> Camera2D {
        Camera2D { focus_position, zoom: zoom.into() as f32 }
    }

	pub fn get_projection_matrix(self, window: Window) -> Matrix4x4 {
		let left = self.focus_position.x - window.size.x / 2.0;
		let right = self.focus_position.x + window.size.x / 2.0;
		let top = self.focus_position.y - window.size.y / 2.0;
		let bottom = self.focus_position.y + window.size.y / 2.0;

		let orthoMatrix: Matrix4x4 = Matrix4x4::create_orthographic_off_center(left, right, bottom, top, 0.01, 100.0);
		let zoomMatrix: Matrix4x4 = Matrix4x4::create_scale(Scale3D::from(self.zoom, self.zoom, self.zoom));

		orthoMatrix.mult(zoomMatrix)
	}
}

impl Eq for Camera2D {}

impl PartialEq for Camera2D {
    fn eq(&self, other: &Camera2D) -> bool {
        self.focus_position == other.focus_position && self.zoom == other.zoom
    }
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn camera_new() {
        let cam = Camera2D::new();
        assert_eq!(cam, Camera2D { focus_position: Vector2D { x: 0.0, y: 0.0 }, zoom: 1.0 });
    }

    #[test]
    fn camera_from() {
        let cam = Camera2D::from(Vector2D::from(10.0, 10.0), 2.0);
        assert_eq!(cam, Camera2D { focus_position: Vector2D { x: 10.0, y: 10.0 }, zoom: 2.0 });
    }
}