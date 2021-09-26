use serde::Deserialize;

use crate::core::types::{Cardinal, Int, Real};

#[derive(Debug, Deserialize)]
pub struct TestMap {
    pub range: Int,
    pub a: Real,
    pub b: Real,
    pub cardinal: Cardinal,
    pub layout: String,
    pub expected_visible: String,
}

#[derive(Debug, Deserialize)]
pub struct TestMapCases {
    pub cases: Vec<TestMap>,
}
