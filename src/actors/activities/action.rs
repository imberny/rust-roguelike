use crate::core::types::Facing;

use super::Message;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    None,
    Wait,
    Move(Facing),
    Face(Facing),
    Say(Message),
}

impl Default for Action {
    fn default() -> Self {
        Self::Wait
    }
}
