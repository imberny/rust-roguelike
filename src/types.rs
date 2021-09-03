use crate::constants::facings::SOUTH;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Facing {
    pub x: i32,
    pub y: i32,
}

impl Facing {
    pub const fn constant(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Facing {
    fn default() -> Self {
        SOUTH
    }
}

mod percentage {

    const PERCENTAGE_LOWER_BOUND: f32 = 0.0;
    const PERCENTAGE_UPPER_BOUND: f32 = 100.0;

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
