use std::collections::HashSet;

use fraction::{Fraction, ToPrimitive};
use rltk::Point;

use crate::map::{Map, TileType};

type Cardinal = usize;

const NORTH: Cardinal = 0;
const EAST: Cardinal = 1;
const SOUTH: Cardinal = 2;
const WEST: Cardinal = 3;

#[derive(Debug, Clone, Copy)]
struct Quadrant {
    pub cardinal: Cardinal,
    pub origin: Point,
}

impl Quadrant {
    fn new(cardinal: Cardinal, origin: Point) -> Self {
        Self { cardinal, origin }
    }

    fn transform(&self, position: Point) -> Point {
        let row = position.x;
        let col = position.y;
        if self.cardinal == NORTH {
            Point::new(self.origin.x + col, self.origin.y - row)
        } else if self.cardinal == SOUTH {
            Point::new(self.origin.x + col, self.origin.y + row)
        } else if self.cardinal == EAST {
            Point::new(self.origin.x + row, self.origin.y + col)
        } else {
            Point::new(self.origin.x - row, self.origin.y + col)
        }
    }
}

#[derive(Clone, Copy)]
struct Row {
    quadrant: Quadrant,
    depth: u16,
    start_slope: Fraction,
    end_slope: Fraction,
}

impl Row {
    fn new(quadrant: Quadrant, depth: u16, start_slope: Fraction, end_slope: Fraction) -> Self {
        Self {
            quadrant,
            depth,
            start_slope,
            end_slope,
        }
    }

    pub fn round_ties_up(&self, n: Fraction) -> Fraction {
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

    pub fn round_ties_down(&self, n: Fraction) -> Fraction {
        let sloped = n * self.end_slope;
        let sum = sloped - Fraction::new(1u16, 2u32);
        if sum.is_sign_negative() {
            sum.floor()
        } else {
            sum.ceil()
        }
    }

    fn tiles(&self) -> Vec<Point> {
        let min_col = self.round_ties_up(Fraction::new(self.depth, 1u32));
        let max_col = self.round_ties_down(Fraction::new(self.depth, 1u32));
        let mut tiles: Vec<Point> = Vec::new();
        for col in min_col.round().to_i32().unwrap()..=max_col.round().to_i32().unwrap() {
            tiles.push(Point::new(self.depth as i32, col));
        }
        tiles
    }

    fn next(&self) -> Row {
        Row::new(
            self.quadrant,
            self.depth + 1,
            self.start_slope,
            self.end_slope,
        )
    }
}

fn is_out_of_bounds(map: &Map, position: Point) -> bool {
    position.x < 0 || position.y < 0 || position.x >= map.width || position.y >= map.height
}

fn scan(map: &Map, quadrant: &Quadrant, row: &mut Row) -> HashSet<Point> {
    let mut previous_tile: Option<Point> = None;
    let mut visible_tiles: HashSet<Point> = HashSet::new();

    for tile in row.tiles() {
        let position = quadrant.transform(tile);
        if !map.is_point_in_bounds(position) {
            continue;
        }
        if is_wall(map, position) || is_symmetric(&row, tile) {
            visible_tiles.insert(position);
        }
        if let Some(previous_tile) = previous_tile {
            let previous_position = quadrant.transform(previous_tile);
            if is_wall(map, previous_position) && is_floor(map, position) {
                row.start_slope = slope(tile);
            }

            if is_floor(map, previous_position) && is_wall(map, position) {
                let mut next_row = row.next();
                next_row.end_slope = slope(tile);
                visible_tiles.extend(scan(map, quadrant, &mut next_row));
            }
        }
        previous_tile = Some(position);
    }
    if let Some(previous_tile) = previous_tile {
        let previous_position = quadrant.transform(previous_tile);
        if is_floor(map, previous_position) {
            visible_tiles.extend(scan(map, quadrant, &mut row.next()));
        }
    }

    visible_tiles
}

fn scan_iterative(map: &Map, row: &mut Row) -> HashSet<Point> {
    let mut rows = vec![row.clone()];
    let mut visible: HashSet<Point> = HashSet::new();
    while 0 < rows.len() {
        let mut row = rows.pop().unwrap();
        let mut prev_tile: Option<Point> = None;
        for tile in row.tiles() {
            let position = row.quadrant.transform(tile);
            if is_wall(map, position) || is_symmetric(&row, tile) {
                visible.insert(position);
            }
            if let Some(prev_tile) = prev_tile {
                let prev_pos = row.quadrant.transform(prev_tile);
                if is_wall(map, prev_pos) && is_floor(map, position) {
                    row.start_slope = slope(tile);
                }
                if is_floor(map, prev_pos) && is_wall(map, position) {
                    let mut next_row = row.next();
                    next_row.end_slope = slope(tile);
                    rows.push(next_row);
                }
            }

            prev_tile = Some(tile);
        }
        if let Some(prev_tile) = prev_tile {
            let prev_pos = row.quadrant.transform(prev_tile);
            if is_floor(map, prev_pos) {
                rows.push(row.next());
            }
        }
    }
    visible
}

fn slope(tile: Point) -> Fraction {
    let row_depth = tile.x as u32;
    let col = tile.y as i16;
    let num = 2i16 * col - 1i16;
    if 0 < num {
        Fraction::new(num as u16, 2u32 * row_depth)
    } else {
        Fraction::new_neg(num.abs() as u16, 2u32 * row_depth)
    }
}

fn is_symmetric(row: &Row, position: Point) -> bool {
    let col = position.y;
    let depth_fraction = Fraction::new(row.depth as u16, 1u32);

    let start_slope = depth_fraction * row.start_slope;
    let start_sign = start_slope.signum();
    // let start_slope_val = (start_sign * start_slope.abs().round()).to_i32().unwrap();
    let start_slope_val = (start_slope.round()).to_i32().unwrap();

    let end_slope = depth_fraction * row.end_slope;
    let end_sign = end_slope.signum();
    // let end_slope_val = (end_sign * end_slope.abs().round()).to_i32().unwrap();
    let end_slope_val = (end_slope.round()).to_i32().unwrap();

    col + 1 >= start_slope_val && col - 1 <= end_slope_val
}

fn is_wall(map: &Map, tile: Point) -> bool {
    match map.at(tile) {
        TileType::Wall => true,
        TileType::Floor => false,
    }
}

fn is_floor(map: &Map, tile: Point) -> bool {
    match map.at(tile) {
        TileType::Wall => false,
        TileType::Floor => true,
    }
}

pub fn symmetric_shadowcasting(map: &Map, origin: Point, range: usize) -> Vec<Point> {
    let mut visible_positions: HashSet<Point> = HashSet::new();
    visible_positions.insert(origin);

    for cardinal in NORTH..=WEST {
        let quadrant = Quadrant::new(cardinal, origin);
        let mut first_row = Row::new(
            quadrant,
            1,
            Fraction::new_neg(1u16, 1u32),
            Fraction::new(1u16, 1u32),
        );
        // visible_positions.extend(scan(&map, &quadrant, &mut first_row));
        visible_positions.extend(scan_iterative(&map, &mut first_row));
    }

    visible_positions
        .into_iter()
        // .filter(|point| rltk::DistanceAlg::Chebyshev.distance2d(origin, *point) <= range as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use fraction::{Fraction, ToPrimitive};
    use rltk::Point;

    use crate::map::Map;

    use super::symmetric_shadowcasting;

    fn small_map() -> Map {
        Map::from_ascii(
            r"######
#....#
#....#
######",
        )
    }

    #[test]
    fn from_ascii_map() {
        let map = small_map();
        assert_eq!(map.width, 6);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn given_a_range_of_one_the_origin_is_visible() {
        let map = small_map();

        let visible_positions = symmetric_shadowcasting(&map, Point::new(1, 3), 0);

        assert_eq!(visible_positions, [Point::new(1, 3)]);
    }

    #[test]
    fn given_a_range_of_two() {
        let map = small_map();

        let visible_positions = symmetric_shadowcasting(&map, Point::new(1, 2), 1);

        let expected_positions: Vec<Point> = vec![
            Point::new(0, 2),
            Point::new(0, 3),
            Point::new(0, 4),
            Point::new(1, 2),
            Point::new(1, 3),
            Point::new(1, 4),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
        ];

        // assert_eq!(expected_positions, visible_positions);
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
