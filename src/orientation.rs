use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Orientation {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Orientation {
    pub fn rotate(&mut self, rot: Quat) {
        self.x = rot * self.x;
        self.y = rot * self.y;
        self.z = rot * self.z;
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            x: Vec3::X,
            y: Vec3::Y,
            z: Vec3::Z,
        }
    }
}

impl From<Orientation> for Quat {
    fn from(orientation: Orientation) -> Self {
        let rot_mat = Mat3::from_cols(orientation.x, orientation.y, orientation.z);
        Quat::from_rotation_mat3(&rot_mat)
    }
}
