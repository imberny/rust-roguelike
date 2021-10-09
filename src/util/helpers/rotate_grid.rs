use bevy::math::{IVec2, Vec2};

use crate::core::types::Facing;

pub trait GridRotator {
    fn rot_i(&self, pos: &IVec2) -> IVec2;
    fn rot_f(&self, pos: &Vec2) -> Vec2;
}

impl GridRotator for Facing {
    fn rot_i(&self, pos: &IVec2) -> IVec2 {
        self.rot_f(&pos.as_vec2()).round().as_ivec2()
    }

    fn rot_f(&self, pos: &Vec2) -> Vec2 {
        self.mul_vec3(pos.extend(0.)).truncate()
    }
}
