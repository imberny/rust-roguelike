use crate::core::types::Increment;

use super::Action;

#[derive(Debug, Default)]
pub struct Activity {
    pub time_to_complete: Increment,
    pub action: Action,
}
