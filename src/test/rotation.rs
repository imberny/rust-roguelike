use serde::Deserialize;

use crate::{
    core::types::{Cardinal, GridPos, Int},
    util::helpers::deserialize,
};

#[derive(Debug, Clone)]
pub struct RotationTestCase {
    pub shape: Vec<GridPos>,
    pub cardinal: Cardinal,
    pub result: Vec<GridPos>,
}

pub fn get_cases(path: &str) -> impl Iterator<Item = RotationTestCase> {
    let test_cases: RotationTestCases = deserialize(path);
    test_cases.into_iter()
}

#[derive(Debug, Clone, Deserialize)]
struct RotationTestCases {
    cases: Vec<RotationTestCaseInternal>,
}

#[derive(Debug, Clone, Deserialize)]
struct RotationTestCaseInternal {
    shape: String,
    cardinal: Cardinal,
    result: String,
}

impl IntoIterator for RotationTestCases {
    type Item = RotationTestCase;

    type IntoIter = RotationTestCasesIterator;

    fn into_iter(self) -> Self::IntoIter {
        RotationTestCasesIterator {
            test_cases: self.cases,
            index: 0,
        }
    }
}

pub struct RotationTestCasesIterator {
    test_cases: Vec<RotationTestCaseInternal>,
    index: usize,
}

impl RotationTestCasesIterator {
    fn extract_positions(&self, ascii: &str) -> Vec<GridPos> {
        let mut positions: Vec<GridPos> = vec![];

        let row_count = ascii.split('\n').count();

        let width = ascii.trim_start().find('\n').unwrap() as Int;
        let rows = ascii.split('\n');
        let mut y = -(row_count as Int / 2);
        for row in rows {
            assert_eq!(width, row.trim_start().len() as Int);
            for (index, char) in row.trim_start().char_indices() {
                if char == '0' {
                    let x = (index as Int) - (width / 2);
                    positions.push(GridPos::new(x, y))
                }
            }
            y += 1;
        }
        positions
    }
}

impl Iterator for RotationTestCasesIterator {
    type Item = RotationTestCase;

    fn next(&mut self) -> Option<Self::Item> {
        if self.test_cases.len() <= self.index {
            return None;
        }
        let shape = self.extract_positions(&self.test_cases[self.index].shape);
        let result = self.extract_positions(&self.test_cases[self.index].result);
        let cardinal = self.test_cases[self.index].cardinal;
        self.index += 1;
        Some(RotationTestCase {
            shape,
            cardinal,
            result,
        })
    }
}
