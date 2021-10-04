use crate::core::types::Direction;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Wait,
    Move(Direction),
    Turn(Direction),
    InitiateAttack,
    Attack,
}

impl Default for Action {
    fn default() -> Self {
        Self::Wait
    }
}
