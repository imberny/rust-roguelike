use ultraviolet as uv;

use super::Int;

pub type GridPos = uv::IVec2;
pub type RealPos = uv::Vec2;

pub trait IntoGridPos {
    fn round(&self) -> GridPos;
}

impl IntoGridPos for RealPos {
    fn round(&self) -> GridPos {
        GridPos {
            x: self.x.round() as Int,
            y: self.y.round() as Int,
        }
    }
}

pub type GridPosPredicate<'a> = dyn Fn(&GridPos) -> bool + 'a;
