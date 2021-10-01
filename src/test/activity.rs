use serde::Deserialize;

use crate::{
    core::types::{Cardinal, Direction, Int},
    util::helpers::deserialize,
};

pub fn cases() -> impl Iterator<Item = MoveTestCase> {
    let test_cases: MoveTestCases = deserialize("src/test/data/moves.ron");
    test_cases.cases.into_iter()
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct MoveTestCase {
    pub direction: Direction,
    pub cardinal: Cardinal,
    pub expected: (Int, Int),
    pub is_blocked: bool,
}

#[derive(Debug, Deserialize)]
struct MoveTestCases {
    cases: Vec<MoveTestCase>,
}
