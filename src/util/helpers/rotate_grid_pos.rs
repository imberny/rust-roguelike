use crate::core::types::{Facing, GridPos, RealPos};

pub trait GridPosRotator {
    fn rot_grid(&self, pos: &GridPos) -> GridPos;
    fn rot_real(&self, pos: &RealPos) -> RealPos;
}

impl GridPosRotator for Facing {
    fn rot_grid(&self, pos: &GridPos) -> GridPos {
        self.rot_real(&pos.as_vec2()).round().as_ivec2()
    }

    fn rot_real(&self, pos: &RealPos) -> RealPos {
        self.mul_vec3(pos.extend(0.)).truncate()
    }
}
