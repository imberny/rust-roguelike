use crate::core::types::{Cardinal, Direction};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Wait,
    Move(Direction),
    Face(Cardinal),
    Turn(Direction),
    InitiateAttack,
    Attack,
}

impl Default for Action {
    fn default() -> Self {
        Self::Wait
    }
}
