use fraction::Fraction;

use crate::{
    core::types::{GridPos, Int},
    util::math::RealToInt,
};

use super::field_of_view::FieldOfView;

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

    fn transform(&self, point: GridPos) -> GridPos {
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
    quadrant: Quadrant,
    depth: u16,
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

    pub fn tiles<F>(&self, is_visible: &F, from: GridPos) -> Vec<QuadrantTile>
    where
        F: Fn(&GridPos) -> bool,
    {
        let min_col = self.round_ties_up(Fraction::new(self.depth, 1u32));
        let max_col = self.round_ties_down(Fraction::new(self.depth, 1u32));
        let mut tiles: Vec<QuadrantTile> = Vec::new();
        for column in min_col.round().int()..=max_col.round().int() {
            let local_quadrant_position = GridPos::new(self.depth as Int, column);
            let position = self.quadrant.transform(local_quadrant_position);
            let delta = GridPos::new(position.x - from.x, position.y - from.y);
            if is_visible(&delta) {
                tiles.push(QuadrantTile {
                    row_depth: self.depth,
                    column: column as u32,
                    position,
                });
            }
        }
        tiles
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

    fn round_ties_up(&self, n: Fraction) -> Fraction {
        let sloped = self.start_slope * n;
        let sum = sloped + Fraction::new(1u16, 2u32);
        if sum.is_sign_negative() {
            sum.ceil()
        } else {
            sum.floor()
        }
    }

    fn round_ties_down(&self, n: Fraction) -> Fraction {
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