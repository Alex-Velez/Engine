use crate::{Position2D, Rotation2D, Scale2D};
use std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Debug)]
pub struct Transform2D {
	pub position: Position2D,
	pub rotation: Rotation2D,
	pub scale: Scale2D,
}

impl Transform2D {
	pub const fn new() -> Transform2D {
		Transform2D {
			position: Position2D::new(),
			rotation: Rotation2D::new(),
			scale: Scale2D::new(),
		}
	}

    pub const fn from(position: Position2D, rotation: Rotation2D, scale: Scale2D) -> Transform2D {
        Transform2D { position, rotation, scale }
    }
}

impl Eq for Transform2D {}

impl PartialEq for Transform2D {
    fn eq(&self, other: &Transform2D) -> bool {
        self.position == other.position && self.rotation == other.rotation && self.rotation == other.rotation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_2d_new() {
        let transform = Transform2D::new();
        let raw = Transform2D {
			position: Position2D::new(),
			rotation: Rotation2D::new(),
			scale: Scale2D::new(),
		};
        assert_eq!(transform, raw);
    }
    
    #[test]
    fn transform_2d_from() {
        let transform = Transform2D::from(Position2D::new(), Rotation2D::new(), Scale2D::new());
        let raw = Transform2D {
			position: Position2D::new(),
			rotation: Rotation2D::new(),
			scale: Scale2D::new(),
		};
        assert_eq!(transform, raw);
    }
}

