use fraction::Fraction;

use crate::{
    core::types::{GridPos, Int},
    util::math::RealToInt,
};

#[derive(Debug, Clone, Copy)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub fn iterator() -> impl Iterator<Item = Cardinal> {
        [
            Cardinal::North,
            Cardinal::South,
            Cardinal::East,
            Cardinal::West,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quadrant {
    cardinal: Cardinal,
    origin: GridPos,
}

impl Quadrant {
    pub fn new(cardinal: Cardinal, origin: GridPos) -> Self {
        Self { cardinal, origin }
    }

    pub fn transform(&self, point: GridPos) -> GridPos {
        match self.cardinal {
            Cardinal::North => GridPos::new(self.origin.x + point.y, self.origin.y - point.x),
            Cardinal::East => GridPos::new(self.origin.x + point.x, self.origin.y + point.y),
            Cardinal::South => GridPos::new(self.origin.x + point.y, self.origin.y + point.x),
            Cardinal::West => GridPos::new(self.origin.x - point.x, self.origin.y + point.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadrantRow {
    pub quadrant: Quadrant,
    pub depth: u16,
    pub start_slope: Fraction,
    pub end_slope: Fraction,
}

impl QuadrantRow {
    pub fn from_quadrant(quadrant: Quadrant) -> Self {
        Self {
            quadrant,
            depth: 1,
            start_slope: Fraction::new_neg(1u16, 1u32),
            end_slope: Fraction::new(1u16, 1u32),
        }
    }

    pub fn next(&self) -> QuadrantRow {
        let mut next = self.clone();
        next.depth += 1;
        next
    }

    pub fn is_symmetric(&self, tile: &QuadrantTile) -> bool {
        let col = tile.column;
        let depth_fraction = Fraction::new(self.depth, 1u32);

        let start_slope = depth_fraction * self.start_slope;
        let start_slope_val = start_slope.int();

        let end_slope = depth_fraction * self.end_slope;
        let end_slope_val = end_slope.int();

        col as Int + 1 >= start_slope_val && col as Int - 1 <= end_slope_val
    }

    pub fn round_ties_up(&self, n: Fraction) -> Fraction {
        let sloped = self.start_slope * n;
        let sum = sloped + Fraction::new(1u16, 2u32);
        if sum.is_sign_negative() {
            sum.ceil()
        } else {
            sum.floor()
        }
    }

    pub fn round_ties_down(&self, n: Fraction) -> Fraction {
        let sloped = n * self.end_slope;
        let sum = sloped - Fraction::new(1u16, 2u32);
        if sum.is_sign_negative() {
            sum.floor()
        } else {
            sum.ceil()
        }
    }
}

pub struct QuadrantTile {
    pub row_depth: u16,
    pub column: u32,
    pub position: GridPos,
}
