use ultraviolet as uv;

use super::Int;

pub type GridPos = uv::IVec2;
pub type RealPos = uv::Vec2;

pub trait IntoGridPos {
    fn round(&self) -> GridPos;
    // fn ceil(&self) -> GridPos;
}

impl IntoGridPos for RealPos {
    fn round(&self) -> GridPos {
        GridPos {
            x: self.x.round() as Int,
            y: self.y.round() as Int,
        }
    }

    // fn ceil(&self) -> GridPos {
    //     let x = if 0.0 <= self.x {
    //         self.x.ceil()
    //     } else {
    //         self.x.floor()
    //     };

    //     let y = if 0.0 <= self.y {
    //         self.y.ceil()
    //     } else {
    //         self.y.floor()
    //     };
    //     GridPos {
    //         x: x as Int,
    //         y: y as Int,
    //     }
    // }
}

pub type GridPosPredicate<'a> = dyn Fn(GridPos) -> bool + 'a;
