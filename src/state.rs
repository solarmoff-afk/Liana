use glam::{Mat4, Vec3};

pub struct LianaState {
    pub view: Mat4,
    pub projection: Mat4,
}

impl LianaState {
    pub const fn new() -> Self {
        Self {
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY, 
        }
    }
}