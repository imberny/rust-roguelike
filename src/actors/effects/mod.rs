pub mod systems;

use crate::core::types::Increment;

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub time_left: Increment,
}
