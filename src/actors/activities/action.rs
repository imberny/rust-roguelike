use crate::core::types::{Cardinal, Direction};

use super::Message;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    None,
    Wait,
    Move(Direction),
    Face(Cardinal),
    Turn(Direction),
    Say(Message),
    InitiateAttack,
    Attack,
}

impl Default for Action {
    fn default() -> Self {
        Self::Wait
    }
}
