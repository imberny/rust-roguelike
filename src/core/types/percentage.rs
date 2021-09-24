use super::Real;

const LOWER_BOUND: Real = 0.0;
const UPPER_BOUND: Real = 100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage {
    value: Real,
}

impl Default for Percentage {
    fn default() -> Self {
        Self { value: UPPER_BOUND }
    }
}

impl From<Real> for Percentage {
    fn from(value: Real) -> Self {
        Self {
            value: value.clamp(LOWER_BOUND, UPPER_BOUND),
        }
    }
}
