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
        core::types::{Cardinal, GridPos, Int},
        game_world::{AreaGrid, TileType},
        util::algorithms::field_of_view::{self, FieldOfView},
    };

    use super::symmetric_shadowcasting;

    #[test]
    fn from_ascii_map() {
        let map = square_room();
        assert_eq!(map.width, 6);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn given_a_rectangular_room_every_square_is_visible() {
        let map = square_room();
        let fov = field_of_view::infinite_fov();

        let visible_positions =
            symmetric_shadowcasting(GridPos::new(1, 2), &|pos1| fov.sees(pos1), &|pos2| {
                map.is_blocking(pos2)
            });

        assert_eq!(map.tiles.len(), visible_positions.len());
        for (position, _tile_type) in map {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_corridor_crossing() {
        let map = cross();
        let fov = field_of_view::infinite_fov();

        let expected_positions: Vec<GridPos> = vec![
            GridPos::new(1, 2),
            GridPos::new(0, 4),
            GridPos::new(1, 3),
            GridPos::new(3, 4),
            GridPos::new(5, 5),
            GridPos::new(6, 6),
            GridPos::new(1, 5),
            GridPos::new(2, 5),
            GridPos::new(4, 5),
            GridPos::new(3, 5),
            GridPos::new(2, 4),
            GridPos::new(3, 3),
            GridPos::new(1, 4),
            GridPos::new(3, 2),
            GridPos::new(0, 5),
            GridPos::new(2, 2),
            GridPos::new(2, 3),
            GridPos::new(6, 5),
            GridPos::new(4, 4),
        ];

        let visible_positions =
            symmetric_shadowcasting(GridPos::new(2, 3), &|pos| fov.sees(pos), &|pos| {
                map.is_blocking(pos)
            });

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_fov_of_zero_only_the_origin_is_visible() {
        let map = square_room();
        let fov = field_of_view::omnidirectional_fov(0);

        let visible_positions =
            symmetric_shadowcasting(GridPos::new(1, 3), &|pos| fov.sees(pos), &|pos| {
                map.is_blocking(pos)
            });

        assert_eq!(visible_positions, [GridPos::new(1, 3)]);
    }

    #[test]
    fn curve_fov() {
        let fov = field_of_view::quadratic_fov(2, Cardinal::South.into(), 0.5, -1.5);

        let map = square_room();

        let expected_positions: Vec<GridPos> = vec![
            GridPos::new(0, 0),
            GridPos::new(0, 1),
            GridPos::new(1, 0),
            GridPos::new(1, 1),
            GridPos::new(1, 2),
            GridPos::new(2, 1),
            GridPos::new(2, 2),
        ];

        let visible_positions =
            symmetric_shadowcasting(GridPos::new(1, 1), &|pos| fov.sees(pos), &|pos| {
                map.is_blocking(pos)
            });

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }

    fn from_ascii(ascii_map: &str) -> AreaGrid {
        let mut map = AreaGrid {
            tiles: Vec::new(),
            width: 0,
            height: 0,
            revealed: Vec::new(),
            visible: Vec::new(),
        };

        map.width = ascii_map.find('\n').unwrap() as Int;
        let rows = ascii_map.split('\n');
        for row in rows {
            assert_eq!(map.width, row.len() as Int);
            map.height += 1;
            for tile in row.chars() {
                match tile {
                    '.' => map.tiles.push(TileType::Floor),
                    '#' => map.tiles.push(TileType::Wall),
                    _ => panic!("Unrecognized map tile: {:?}", tile),
                }
            }
        }

        map
    }

    fn from_ascii_layout(ascii_map: &str) -> (GridPos, AreaGrid) {
        let mut map = AreaGrid {
            tiles: Vec::new(),
            width: 0,
            height: 0,
            revealed: Vec::new(),
            visible: Vec::new(),
        };
        let mut origin = GridPos::zero();

        map.width = ascii_map.find('\n').unwrap() as Int;
        let rows = ascii_map.split('\n');
        let mut y = 0;
        for row in rows {
            assert_eq!(map.width, row.len() as Int);
            for (x, tile) in row.char_indices() {
                match tile {
                    '.' => map.tiles.push(TileType::Floor),
                    '#' => map.tiles.push(TileType::Wall),
                    '@' => {
                        map.tiles.push(TileType::Floor);
                        origin.x = x as i32;
                        origin.y = y as i32;
                    }

                    _ => panic!("Unrecognized map tile: {:?}", tile),
                }
            }
            y += 1;
        }
        map.height = y;

        (origin, map)
    }

    fn from_ascii_expected(ascii_map: &str) -> Vec<GridPos> {
        let mut visible_positions: Vec<GridPos> = vec![];

        let width = ascii_map.find('\n').unwrap() as Int;
        let rows = ascii_map.split('\n');
        let mut y = 0;
        for row in rows {
            assert_eq!(width, row.len() as Int);
            for (x, char) in row.char_indices() {
                if char == 'y' {
                    visible_positions.push(GridPos::new(x as i32, y as i32))
                }
            }
            y += 1
        }

        visible_positions
    }

    fn square_room() -> AreaGrid {
        from_ascii(
            r"######
#....#
#....#
######",
        )
    }

    #[derive(Debug, Deserialize)]
    struct TestMap {
        range: Int,
        layout: String,
        expected_visible: String,
    }

    #[derive(Debug, Deserialize)]
    struct TestMapCases {
        cases: Vec<TestMap>,
    }

    fn cross() -> AreaGrid {
        let current_dir = std::env::current_dir().unwrap();
        let path = format!(
            "{}/{}",
            current_dir.to_str().unwrap(),
            "src/test/data/maps.ron"
        );
        let maps = std::fs::File::open(&path).expect("Failed opening file");

        let test_cases: TestMapCases = match from_reader(maps) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        from_ascii(&test_cases.cases[0].layout)
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
    fn map_test() {
        let test_cases = read_test_cases();
        let pos_sorter = |first: &GridPos, second: &GridPos| match first.y.cmp(&second.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => first.x.cmp(&second.x),
        };

        for case in test_cases.cases {
            let (origin, map) = from_ascii_layout(&case.layout);
            let expected = from_ascii_expected(&case.expected_visible);

            let fov = field_of_view::quadratic_fov(case.range, Cardinal::East.into(), 0.5, -1.5);

            let mut visible_positions =
                symmetric_shadowcasting(origin, &|pos| fov.sees(pos), &|pos| map.is_blocking(pos));

            visible_positions.sort_by(pos_sorter);

            assert_eq!(expected, visible_positions);
        }
    }
}
