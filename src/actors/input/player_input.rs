use crate::{actors::Action, core::types::Int};

#[derive(Default)]
pub struct PlayerInput {
    pub action: Action,
    pub cursor_pos: (Int, Int),
    pub left_click: bool,
    pub is_strafing: bool,
    pub skew_move: bool,
    pub alt: bool,
}

impl PlayerInput {
    pub fn is_valid(&self) -> bool {
        !matches!(self.action, Action::None)
    }
}
