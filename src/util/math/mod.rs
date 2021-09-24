use fraction::{Fraction, ToPrimitive};

use crate::core::types::Int;

pub trait RealToInt {
    fn int(&self) -> Int;
}

impl RealToInt for Fraction {
    fn int(&self) -> Int {
        self.round().to_i32().unwrap_or_default()
    }
}
