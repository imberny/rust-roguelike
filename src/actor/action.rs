use crate::types::Facing;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MessageType {
    Insult,
    Threaten,
    Compliment,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Compliment
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Message {
    pub kind: MessageType,
}
