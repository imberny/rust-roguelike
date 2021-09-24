// use crate::core::constants::facings::SOUTH;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Facing {
//     pub x: i32,
//     pub y: i32,
// }

pub type Facing = uv::Rotor2;

// impl Facing {
//     pub const fn constant(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }

//     pub fn to_vec2(self) -> Vec2i {
//         Vec2i::new(self.x, self.y)
//     }
// }

// impl Default for Facing {
//     fn default() -> Self {
//         SOUTH
//     }
// }

// impl From<Vec2i> for Facing {
//     fn from(point: Vec2i) -> Self {
//         Self {
//             x: point.x,
//             y: point.y,
//         }
//     }
// }
// }

mod percentage {
    const PERCENTAGE_LOWER_BOUND: f32 = 0.0;
    const PERCENTAGE_UPPER_BOUND: f32 = 100.0;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Percentage {
        value: f32,
    }

    impl Default for Percentage {
        fn default() -> Self {
            Self {
                value: PERCENTAGE_UPPER_BOUND,
            }
        }
    }

    impl From<f32> for Percentage {
        fn from(value: f32) -> Self {
            Self {
                value: value.clamp(PERCENTAGE_LOWER_BOUND, PERCENTAGE_UPPER_BOUND),
            }
        }
    }
}
pub use percentage::Percentage;
use ultraviolet as uv;

pub type GridPos = uv::IVec2;
pub type RealPos = uv::Vec2;

pub trait IntoGridPos {
    fn as_grid_pos(&self) -> GridPos;
}

impl IntoGridPos for RealPos {
    fn as_grid_pos(&self) -> GridPos {
        GridPos {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }
}

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Position {
//     pub x: i32,
//     pub y: i32,
// }

// impl Position {
//     pub fn new(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }

//     pub const fn constant(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }

//     pub fn as_vec2(&self) -> uv::Vec2 {
//         uv::Vec2 {
//             x: self.x as f32,
//             y: self.y as f32,
//         }
//     }
// }

// impl From<uv::IVec2> for Position {
//     fn from(vec: uv::IVec2) -> Self {
//         Self { x: vec.x, y: vec.y }
//     }
// }

// impl From<uv::Vec2> for Position {
//     fn from(vec: uv::Vec2) -> Self {
//         Self {
//             x: vec.x.round() as i32,
//             y: vec.y.round() as i32,
//         }
//     }
// }

// impl From<Point> for Position {
//     fn from(point: Point) -> Self {
//         Self::new(point.x, point.y)
//     }
// }
