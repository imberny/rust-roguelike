use std::collections::HashSet;

use fraction::Fraction;

use super::quadrant::{Cardinal, Quadrant, QuadrantRow, QuadrantTile};
use crate::{
    core::types::{GridPos, GridPosPredicate, Int},
    util::math::RealToInt,
};

struct RowProvider {
    cardinal: Cardinal,
    origin: GridPos,
}

impl IntoIterator for RowProvider {
    type Item = QuadrantRow;

    type IntoIter = RowProviderIterator;

    fn into_iter(self) -> Self::IntoIter {
        let first_row = QuadrantRow::from_quadrant(Quadrant::new(self.cardinal, self.origin));
        RowProviderIterator { row: first_row }
    }
}

struct RowProviderIterator {
    row: QuadrantRow,
}

impl Iterator for RowProviderIterator {
    type Item = QuadrantRow;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.row.next())
    }
}

pub fn symmetric_shadowcasting(
    origin: GridPos,
    is_visible: &GridPosPredicate,
    is_blocking: &GridPosPredicate,
) -> Vec<GridPos> {
    let mut visible_positions: HashSet<GridPos> = HashSet::new();
    visible_positions.insert(origin);

    for cardinal in Cardinal::iterator() {
        // let row_provider = RowProvider { cardinal, origin };
        let first_row = QuadrantRow::from_quadrant(Quadrant::new(cardinal, origin));
        visible_positions.extend(scan_iterative(first_row, origin, is_visible, is_blocking));
    }

    visible_positions.into_iter().collect()
}

fn scan_iterative(
    first_row: QuadrantRow,
    // row_provider: RowProvider,
    origin: GridPos,
    is_visible: &GridPosPredicate,
    is_blocking: &GridPosPredicate,
) -> HashSet<GridPos> {
    let mut rows: Vec<QuadrantRow> = vec![first_row];

    let mut visible_positions: HashSet<GridPos> = HashSet::new();

    while !rows.is_empty() {
        let mut row = rows.pop().unwrap();
        let mut prev_tile: Option<QuadrantTile> = None;
        for tile in tiles(&row, is_visible, origin) {
            if let Some(next_row) = check_next_row(&prev_tile, &tile, &mut row, is_blocking) {
                rows.push(next_row);
            }

            prev_tile = try_reveal_tile(tile, &row, is_blocking, &mut visible_positions);
        }
        if let Some(prev_tile) = prev_tile {
            if !is_blocking(prev_tile.position) {
                rows.push(row.next());
            }
        }
    }
    visible_positions
}

fn tiles(row: &QuadrantRow, is_visible: &GridPosPredicate, from: GridPos) -> Vec<QuadrantTile> {
    let min_col = row
        .round_ties_up(Fraction::new(row.depth, 1u32))
        .round()
        .int();
    let max_col = row
        .round_ties_down(Fraction::new(row.depth, 1u32))
        .round()
        .int();
    let mut tiles: Vec<QuadrantTile> = Vec::new();
    for column in min_col..=max_col {
        let local_quadrant_position = GridPos::new(row.depth as Int, column);
        let position = row.quadrant.transform(local_quadrant_position);
        let delta = GridPos::new(position.x - from.x, position.y - from.y);
        if is_visible(delta) {
            tiles.push(QuadrantTile {
                row_depth: row.depth,
                column: column as u32,
                position,
            });
        }
    }
    tiles
}

fn try_reveal_tile(
    tile: QuadrantTile,
    row: &QuadrantRow,
    is_blocking: &GridPosPredicate,
    visible_positions: &mut HashSet<GridPos>,
) -> Option<QuadrantTile> {
    if is_blocking(tile.position) || row.is_symmetric(&tile) {
        visible_positions.insert(tile.position);
    }

    Some(tile)
}

fn check_next_row(
    prev_tile: &Option<QuadrantTile>,
    tile: &QuadrantTile,
    row: &mut QuadrantRow,
    is_blocking: &GridPosPredicate,
) -> Option<QuadrantRow> {
    if let Some(prev_tile) = prev_tile {
        if is_blocking(prev_tile.position) && !is_blocking(tile.position) {
            row.start_slope = slope_from(tile);
        }
        if !is_blocking(prev_tile.position) && is_blocking(tile.position) {
            let mut next_row = row.next();
            next_row.end_slope = slope_from(tile);
            return Some(next_row);
        }
    }
    None
}

fn slope_from(tile: &QuadrantTile) -> Fraction {
    let row_depth = tile.row_depth;
    let column = tile.column as i16;
    let num = 2i16 * column - 1i16;
    if 0 < num {
        Fraction::new(num as u16, 2u16 * row_depth)
    } else {
        Fraction::new_neg(num.abs() as u16, 2u16 * row_depth)
    }
}

#[cfg(test)]
mod tests {

    use std::cmp::Ordering;

    use ron::de::from_reader;
    use serde::Deserialize;

    use crate::{
        core::types::{Cardinal, GridPos, Int, Real},
        game_world::{AreaGrid, TileType},
        util::algorithms::field_of_view::{self, FieldOfView},
    };

    use super::symmetric_shadowcasting;

    fn from_ascii_layout(ascii_map: &str) -> (GridPos, AreaGrid) {
        let mut origin = GridPos::zero();
        let mut tiles: Vec<TileType> = vec![];
        let width = ascii_map.find('\n').unwrap() as Int;

        let rows = ascii_map.split('\n');
        let mut y = 0;
        for row in rows {
            assert_eq!(width, row.trim_start().len() as Int);
            for (x, tile) in row.trim_start().char_indices() {
                match tile {
                    '.' => tiles.push(TileType::Floor),
                    '#' => tiles.push(TileType::Wall),
                    '@' => {
                        tiles.push(TileType::Floor);
                        origin.x = x as i32;
                        origin.y = y as i32;
                    }

                    _ => panic!("Unrecognized map tile: {:?}", tile),
                }
            }
            y += 1;
        }
        let height = y;

        let map = AreaGrid {
            tiles,
            width,
            height,
            revealed: vec![false; (width * height) as usize],
            visible: vec![false; (width * height) as usize],
        };
        (origin, map)
    }

    fn from_ascii_expected(ascii_map: &str) -> Vec<GridPos> {
        let mut visible_positions: Vec<GridPos> = vec![];

        let width = ascii_map.find('\n').unwrap() as Int;
        let rows = ascii_map.split('\n');
        let mut y = 0;
        for row in rows {
            assert_eq!(width, row.trim_start().len() as Int);
            for (x, char) in row.trim_start().char_indices() {
                if char == 'y' {
                    visible_positions.push(GridPos::new(x as i32, y as i32))
                }
            }
            y += 1
        }

        visible_positions
    }

    #[derive(Debug, Deserialize)]
    struct TestMap {
        range: Int,
        a: Real,
        b: Real,
        cardinal: Cardinal,
        layout: String,
        expected_visible: String,
    }

    #[derive(Debug, Deserialize)]
    struct TestMapCases {
        cases: Vec<TestMap>,
    }

    fn read_test_cases() -> TestMapCases {
        let current_dir = std::env::current_dir().unwrap();
        let path = format!(
            "{}/{}",
            current_dir.to_str().unwrap(),
            "src/test/data/maps.ron"
        );
        let maps = std::fs::File::open(&path).expect("Failed opening file");

        match from_reader(maps) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        }
    }

    #[test]
    fn symmetric_shadowcasting_tests() {
        let test_cases = read_test_cases();
        let pos_sorter = |first: &GridPos, second: &GridPos| match first.y.cmp(&second.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => first.x.cmp(&second.x),
        };

        for case in test_cases.cases {
            let (origin, map) = from_ascii_layout(&case.layout);
            let expected = from_ascii_expected(&case.expected_visible);

            let fov =
                field_of_view::quadratic_fov(case.range, case.cardinal.into(), case.a, case.b);

            let mut visible_positions =
                symmetric_shadowcasting(origin, &|pos| fov.sees(pos), &|pos| map.is_blocking(pos));

            visible_positions.sort_by(pos_sorter);

            assert_eq!(expected, visible_positions);
        }
    }
}
