use bevy::math::IVec2;
use serde::Deserialize;

use crate::util::helpers::deserialize;

use super::visibility::from_ascii_axis_positions;

#[derive(Debug, Clone)]
pub struct RotationTestCase {
    pub shape: String,
    pub pattern: Vec<IVec2>,
    pub expected: [Vec<IVec2>; 8],
}

pub fn cases() -> impl Iterator<Item = RotationTestCase> {
    let test_cases: RotationTestCases = deserialize("src/test/data/rotations.ron");
    test_cases.into_iter()
}

#[derive(Debug, Clone, Deserialize)]
struct RotationTestCases {
    cases: Vec<RotationTestCaseInternal>,
}

#[derive(Debug, Clone, Deserialize)]
struct RotationTestCaseInternal {
    shape: String,
    expected: [String; 8],
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

const fn grid_pos_vec() -> Vec<IVec2> {
    Vec::new()
}

const NEW_GRID_POS_VEC: Vec<IVec2> = grid_pos_vec();

impl Iterator for RotationTestCasesIterator {
    type Item = RotationTestCase;

    fn next(&mut self) -> Option<Self::Item> {
        if self.test_cases.len() <= self.index {
            return None;
        }
        let shape = self.test_cases[self.index].shape.clone();
        let pattern = from_ascii_axis_positions(&self.test_cases[self.index].shape);
        let mut expected: [Vec<IVec2>; 8] = [NEW_GRID_POS_VEC; 8];
        for (index, shape) in self.test_cases[self.index].expected.iter().enumerate() {
            expected[index] = from_ascii_axis_positions(shape);
        }
        self.index += 1;
        Some(RotationTestCase {
            shape,
            pattern,
            expected,
        })
    }
}
