use crate::types::Facing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    Move(Facing),
}

impl Default for Action {
    fn default() -> Self {
        Self::None
    }
}
