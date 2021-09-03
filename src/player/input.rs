use crate::actor::Action;

#[derive(Default)]
pub struct PlayerInput {
    pub action: Action,
    pub cursor_pos: (i32, i32),
    pub left_click: bool,
    pub is_strafing: bool,
    pub skew_move: bool,
    pub alt: bool,
}

impl PlayerInput {
    pub fn is_valid(&self) -> bool {
        match self.action {
            Action::None => false,
            _ => true,
        }
    }
}
