use std::{collections::HashSet, slice::Iter};

use fraction::{Fraction, ToPrimitive};
use rltk::Point;

use crate::game_world::{AreaGrid, TileType};

// type Cardinal = usize;

// const NORTH: Cardinal = 0;
// const EAST: Cardinal = 1;
// const SOUTH: Cardinal = 2;
// const WEST: Cardinal = 3;

#[derive(Debug, Clone, Copy)]
enum Cardinal {
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
struct Quadrant {
    pub cardinal: Cardinal,
    pub origin: Point,
}

impl Quadrant {
    fn new(cardinal: Cardinal, origin: Point) -> Self {
        Self { cardinal, origin }
    }

    fn transform(&self, point: Point) -> Point {
        match self.cardinal {
            Cardinal::North => Point::new(self.origin.x + point.y, self.origin.y - point.x),
            Cardinal::East => Point::new(self.origin.x + point.x, self.origin.y + point.y),
            Cardinal::South => Point::new(self.origin.x + point.y, self.origin.y + point.x),
            Cardinal::West => Point::new(self.origin.x - point.x, self.origin.y + point.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct QuadrantRow {
    quadrant: Quadrant,
    depth: u16,
    start_slope: Fraction,
    end_slope: Fraction,
}

impl QuadrantRow {
    fn new(quadrant: Quadrant, depth: u16, start_slope: Fraction, end_slope: Fraction) -> Self {
        Self {
            quadrant,
            depth,
            start_slope,
            end_slope,
        }
    }

    fn from_quadrant(quadrant: Quadrant) -> Self {
        Self {
            quadrant,
            depth: 1,
            start_slope: Fraction::new_neg(1u16, 1u32),
            end_slope: Fraction::new(1u16, 1u32),
        }
    }

    fn round_ties_up(&self, n: Fraction) -> Fraction {
        let sloped = self.start_slope * n;
        // let sum = sloped + Fraction::new(1u16, 2u32);
        // let floored = sum.ceil();
        // floored
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

    fn is_symmetric(&self, tile: &QuadrantTile) -> bool {
        let col = tile.column;
        let depth_fraction = Fraction::new(self.depth, 1u32);

        let start_slope = depth_fraction * self.start_slope;
        let start_slope_val = (start_slope.round()).to_i32().unwrap();

        let end_slope = depth_fraction * self.end_slope;
        let end_slope_val = (end_slope.round()).to_i32().unwrap();

        col as i32 + 1 >= start_slope_val && col as i32 - 1 <= end_slope_val
    }

    fn tiles(&self) -> Vec<QuadrantTile> {
        let min_col = self.round_ties_up(Fraction::new(self.depth, 1u32));
        let max_col = self.round_ties_down(Fraction::new(self.depth, 1u32));
        let mut tiles: Vec<QuadrantTile> = Vec::new();
        for column in min_col.round().to_i32().unwrap()..=max_col.round().to_i32().unwrap() {
            let relative_position = Point::new(self.depth as i32, column);
            let global_position = self.quadrant.transform(relative_position);
            tiles.push(QuadrantTile {
                row_depth: self.depth,
                column: column as u32,
                position: global_position,
            });
        }
        tiles
    }

    fn next(&self) -> QuadrantRow {
        QuadrantRow::new(
            self.quadrant,
            self.depth + 1,
            self.start_slope,
            self.end_slope,
        )
    }
}

struct QuadrantTile {
    row_depth: u16,
    column: u32,
    position: Point,
}

pub struct SymmetricShadowcaster<'a> {
    area_grid: &'a AreaGrid,
}

impl<'a> SymmetricShadowcaster<'a> {
    pub fn new(area_grid: &'a AreaGrid) -> Self {
        SymmetricShadowcaster { area_grid }
    }

    pub fn get_visible_positions(&self, origin: Point, range: usize) -> Vec<Point> {
        let mut visible_positions: HashSet<Point> = HashSet::new();
        visible_positions.insert(origin);

        for cardinal in Cardinal::iterator() {
            let first_row = QuadrantRow::from_quadrant(Quadrant::new(cardinal, origin));
            visible_positions.extend(self.scan_iterative(first_row));
        }

        visible_positions
            .into_iter()
            // .filter(|point| rltk::DistanceAlg::Chebyshev.distance2d(origin, *point) <= range as f32)
            .collect()
    }

    fn scan_iterative(&self, first_row: QuadrantRow) -> HashSet<Point> {
        let mut rows: Vec<QuadrantRow> = vec![first_row];
        let mut visible: HashSet<Point> = HashSet::new();

        while 0 < rows.len() {
            let mut row = rows.pop().unwrap();
            let mut prev_tile: Option<QuadrantTile> = None;
            for tile in row.tiles() {
                if !self.area_grid.is_point_in_bounds(tile.position) {
                    continue;
                }

                if let Some(next_row) = self.check_next_row(&prev_tile, &tile, &mut row) {
                    rows.push(next_row);
                }

                prev_tile =
                    self.try_reveal_tile(tile, &mut row, &mut visible, &mut rows, prev_tile);
            }
            if let Some(prev_tile) = prev_tile {
                if !self.area_grid.is_blocking(prev_tile.position) {
                    rows.push(row.next());
                }
            }
        }
        visible
    }

    fn try_reveal_tile(
        &self,
        tile: QuadrantTile,
        row: &mut QuadrantRow,
        visible: &mut HashSet<Point>,
        rows: &mut Vec<QuadrantRow>,
        prev_tile: Option<QuadrantTile>,
    ) -> Option<QuadrantTile> {
        if self.area_grid.is_blocking(tile.position) || row.is_symmetric(&tile) {
            visible.insert(tile.position);
        }

        Some(tile)
    }

    fn check_next_row(
        &self,
        prev_tile: &Option<QuadrantTile>,
        tile: &QuadrantTile,
        row: &mut QuadrantRow,
    ) -> Option<QuadrantRow> {
        if let Some(prev_tile) = prev_tile {
            if self.area_grid.is_blocking(prev_tile.position)
                && !self.area_grid.is_blocking(tile.position)
            {
                row.start_slope = self.slope(tile);
            }
            if !self.area_grid.is_blocking(prev_tile.position)
                && self.area_grid.is_blocking(tile.position)
            {
                let mut next_row = row.next();
                next_row.end_slope = self.slope(tile);
                return Some(next_row);
            }
        }
        None
    }

    fn slope(&self, tile: &QuadrantTile) -> Fraction {
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

#[cfg(test)]
mod tests {
    use fraction::{Fraction, ToPrimitive};
    use rltk::Point;

    use crate::game_world::AreaGrid;

    use super::SymmetricShadowcaster;

    fn square_room() -> AreaGrid {
        AreaGrid::from_ascii(
            r"######
#....#
#....#
######",
        )
    }

    fn cross() -> AreaGrid {
        AreaGrid::from_ascii(
            r"###########
#####.#####
#####.#####
#####.#####
#.........#
#####.#####
#####.#####
#####.#####
#####.#####
###########",
        )
    }

    #[test]
    fn from_ascii_map() {
        let map = square_room();
        assert_eq!(map.width, 6);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn given_a_range_of_one_the_origin_is_visible() {
        let map = square_room();

        let visible_positions =
            SymmetricShadowcaster { area_grid: &map }.get_visible_positions(Point::new(1, 3), 0);

        assert_eq!(visible_positions, [Point::new(1, 3)]);
    }

    #[test]
    fn given_a_rectangular_room_every_square_is_visible() {
        let map = square_room();

        let visible_positions =
            SymmetricShadowcaster { area_grid: &map }.get_visible_positions(Point::new(1, 2), 1);

        assert_eq!(map.tiles.len(), visible_positions.len());
        for (position, _tile_type) in map {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_corridor_crossing() {
        let map = cross();

        let expected_positions: Vec<Point> = vec![
            Point::new(1, 2),
            Point::new(0, 4),
            Point::new(1, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(6, 6),
            Point::new(1, 5),
            Point::new(2, 5),
            Point::new(4, 5),
            Point::new(3, 5),
            Point::new(2, 4),
            Point::new(3, 3),
            Point::new(1, 4),
            Point::new(3, 2),
            Point::new(0, 5),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(6, 5),
            Point::new(4, 4),
        ];

        let visible_positions =
            SymmetricShadowcaster { area_grid: &map }.get_visible_positions(Point::new(2, 3), 1);

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn fraction() {
        let one = Fraction::new(1u16, 1u32);
        let half = Fraction::new(1u16, 2u32);

        let sum = one + half;
        assert_eq!(Fraction::new(3u16, 2u32), sum);

        assert_eq!(one, sum.floor());
        assert_eq!(1, sum.floor().to_i32().unwrap());
        assert_eq!(
            1,
            (one + Fraction::new(1u16, 2u32)).floor().to_i32().unwrap()
        );

        let diff = one - half;
        assert_eq!(Fraction::new(1u16, 2u32), diff);
        assert_eq!(Fraction::new(0u16, 1u32), diff.floor());
        assert_eq!(0, diff.floor().to_i32().unwrap());

        let three = Fraction::new(3u16, 1u32);
        let quarter = Fraction::new(1u16, 4u32);
        assert_eq!(Fraction::new(3u16, 4u32), three * quarter);
        assert_eq!(1, (three * quarter).round().to_i32().unwrap());
    }
}
