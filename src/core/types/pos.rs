use ultraviolet as uv;

use super::Int;

pub type GridPos = uv::IVec2;
pub type RealPos = uv::Vec2;

pub trait IntoGridPos {
    fn as_grid_pos(&self) -> GridPos;
}

impl IntoGridPos for RealPos {
    fn as_grid_pos(&self) -> GridPos {
        GridPos {
            x: self.x.round() as Int,
            y: self.y.round() as Int,
        }
    }
}

pub type GridPosPredicate = dyn Fn(GridPos) -> bool;
