pub mod field_of_view;

use bevy::math::IVec2;
use fraction::Fraction;
use std::collections::HashSet;

use crate::{
    core::types::{Cardinal, Facing, Int, Predicate},
    util::{helpers::GridRotator, math::RealToInt},
};

type MarkPosition<'a> = dyn FnMut(&IVec2) -> bool + 'a;

pub fn symmetric_shadowcasting(
    origin: &IVec2,
    is_visible: &Predicate<IVec2>,
    is_blocking: &Predicate<IVec2>,
) -> Vec<IVec2> {
    let mut visible_positions: HashSet<IVec2> = HashSet::new();
    if is_visible(&IVec2::ZERO) {
        visible_positions.insert(*origin);
    }

    let cardinals = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ];

    for cardinal in cardinals {
        scan(
            origin,
            cardinal,
            &mut |pos| visible_positions.insert(*pos),
            is_visible,
            is_blocking,
        )
    }

    visible_positions.into_iter().collect()
}

fn scan(
    origin: &IVec2,
    cardinal: Cardinal,
    mark: &mut MarkPosition,
    is_visible: &Predicate<IVec2>,
    is_blocking: &Predicate<IVec2>,
) {
    QuadrantRow::new(*origin, cardinal).scan(mark, is_visible, is_blocking);
}
#[derive(Debug, Clone, Copy)]
struct Quadrant {
    facing: Facing,
    origin: IVec2,
}

impl Quadrant {
    pub fn new(facing: Facing, origin: IVec2) -> Self {
        Self { facing, origin }
    }

    pub fn transform(&self, point: IVec2) -> IVec2 {
        self.origin + self.facing.rot_i(&point)
    }
}

#[derive(Debug, Clone, Copy)]
struct QuadrantTile {
    row_depth: u16,
    column: u32,
    position: IVec2,
}

impl From<QuadrantTile> for Fraction {
    fn from(tile: QuadrantTile) -> Self {
        let row_depth = tile.row_depth;
        let column = tile.column as i16;
        let num = 2i16 * column - 1i16;
        if 0 < num {
            Fraction::new(num as u16, 2u16 * row_depth)
        } else {
            Fraction::new_neg(num.abs() as u16, 2u16 * row_depth)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadrantRow {
    quadrant: Quadrant,
    depth: u16,
    start_slope: Fraction,
    end_slope: Fraction,
}

impl QuadrantRow {
    pub fn new(origin: IVec2, cardinal: Cardinal) -> Self {
        Self {
            quadrant: Quadrant::new(cardinal.into(), origin),
            depth: 1,
            start_slope: Fraction::new_neg(1u16, 1u32),
            end_slope: Fraction::new(1u16, 1u32),
        }
    }

    pub fn scan(
        &mut self,
        mark_visible: &mut MarkPosition,
        is_visible: &Predicate<IVec2>,
        is_blocking: &Predicate<IVec2>,
    ) {
        let mut previous = IVec2::ZERO;
        for tile in self.tiles(is_visible) {
            if previous != IVec2::ZERO {
                self.try_update_slope(tile, previous, is_blocking);
            }

            if self.check_next_row(tile.position, previous, is_blocking) {
                self.next_from(tile.into())
                    .scan(mark_visible, is_visible, is_blocking);
            }

            if is_blocking(&tile.position) || self.is_symmetric(tile.column as Int) {
                mark_visible(&tile.position);
            }

            previous = tile.position;
        }
        if previous != IVec2::ZERO && !is_blocking(&previous) {
            self.next().scan(mark_visible, is_visible, is_blocking);
        }
    }

    fn origin(&self) -> IVec2 {
        self.quadrant.origin
    }

    fn next_from(&self, slope: Fraction) -> QuadrantRow {
        let mut next = self.next();
        next.end_slope = slope;
        next
    }

    fn next(&self) -> QuadrantRow {
        let mut next = self.clone();
        next.depth += 1;
        next
    }

    fn is_symmetric(&self, column: Int) -> bool {
        let depth_fraction = Fraction::new(self.depth, 1u32);

        let start_slope = depth_fraction * self.start_slope;
        let start_slope_val = start_slope.int();

        let end_slope = depth_fraction * self.end_slope;
        let end_slope_val = end_slope.int();

        column + 1 >= start_slope_val && column - 1 <= end_slope_val
    }

    fn try_update_slope(
        &mut self,
        tile: QuadrantTile,
        previous_position: IVec2,
        is_blocking: &Predicate<IVec2>,
    ) {
        if is_blocking(&previous_position) && !is_blocking(&tile.position) {
            let slope: Fraction = tile.into();
            self.start_slope = slope;
        }
    }

    fn check_next_row(
        &self,
        position: IVec2,
        previous_position: IVec2,
        is_blocking: &Predicate<IVec2>,
    ) -> bool {
        !is_blocking(&previous_position) && is_blocking(&position)
    }

    fn tiles(&self, is_visible: &Predicate<IVec2>) -> Vec<QuadrantTile> {
        let min_col = self
            .round_ties_up(Fraction::new(self.depth, 1u32))
            .round()
            .int();
        let max_col = self
            .round_ties_down(Fraction::new(self.depth, 1u32))
            .round()
            .int();
        let mut tiles: Vec<QuadrantTile> = Vec::new();
        for column in min_col..=max_col {
            let local_quadrant_position = IVec2::new(self.depth as Int, column);
            let position = self.quadrant.transform(local_quadrant_position);
            let delta = IVec2::new(position.x - self.origin().x, position.y - self.origin().y);
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

#[cfg(test)]
mod tests {

    use std::cmp::Ordering;

    use bevy::math::IVec2;

    use crate::{
        test::{
            self,
            helpers::visibility::{from_ascii_expected, from_ascii_layout},
        },
        util::algorithms::field_of_view::FOV,
    };

    use super::symmetric_shadowcasting;

    #[test]
    fn symmetric_shadowcasting_tests() {
        let pos_sorter = |first: &IVec2, second: &IVec2| match first.y.cmp(&second.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => first.x.cmp(&second.x),
        };

        for case in test::visibility::cases() {
            let (origin, map) = from_ascii_layout(&case.layout);
            let mut expected = from_ascii_expected(&case.expected_visible);

            let fov = FOV::Quadratic(case.range, case.a, case.b);

            let mut visible_positions =
                symmetric_shadowcasting(&origin, &|pos| fov.sees(pos, case.cardinal), &|pos| {
                    map.is_blocking(pos)
                });

            visible_positions.sort_by(pos_sorter);
            expected.sort_by(pos_sorter);

            assert_eq!(
                expected, visible_positions,
                "Error in case: {:?}, range: {}, \n {}",
                case.cardinal, case.range, case.layout
            );
        }
    }
}
