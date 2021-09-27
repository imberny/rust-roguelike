use serde::Deserialize;

use crate::core::types::{Cardinal, Direction, Int};

#[derive(Debug, Deserialize)]
pub struct MoveTestCases {
    pub cases: Vec<(Direction, Cardinal, Int, Int, bool)>,
}
