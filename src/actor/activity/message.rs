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
