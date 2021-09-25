use std::collections::HashSet;

use fraction::Fraction;

use crate::{
    core::types::{GridPos, Int},
    game_world::quadrant::{Cardinal, Quadrant, QuadrantRow, QuadrantTile},
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
    is_visible: &dyn Fn(GridPos) -> bool,
    is_blocking: &dyn Fn(GridPos) -> bool,
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
    is_visible: &dyn Fn(GridPos) -> bool,
    is_blocking: &dyn Fn(GridPos) -> bool,
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

fn tiles(
    row: &QuadrantRow,
    is_visible: &dyn Fn(GridPos) -> bool,
    from: GridPos,
) -> Vec<QuadrantTile> {
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
    is_blocking: &dyn Fn(GridPos) -> bool,
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
    is_blocking: &dyn Fn(GridPos) -> bool,
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

    use crate::{
        core::{
            constants::SOUTH,
            types::{GridPos, Int},
        },
        game_world::{field_of_view::*, AreaGrid, TileType},
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
        let fov = new_infinite();

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
        let fov = new_infinite();

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
        let fov = new_omni(0);

        let visible_positions =
            symmetric_shadowcasting(GridPos::new(1, 3), &|pos| fov.sees(pos), &|pos| {
                map.is_blocking(pos)
            });

        assert_eq!(visible_positions, [GridPos::new(1, 3)]);
    }

    #[test]
    fn curve_fov() {
        let fov = new_quadratic(2, SOUTH, 0.5, 1.5);

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

        let rows = ascii_map.split('\n');
        for row in rows {
            map.height += 1;
            for tile in row.chars() {
                match tile {
                    '.' => map.tiles.push(TileType::Floor),
                    '#' => map.tiles.push(TileType::Wall),
                    _ => panic!("Unrecognized map tile: {:?}", tile),
                }
            }
        }
        map.width = map.tiles.len() as Int / map.height;

        map
    }

    fn square_room() -> AreaGrid {
        from_ascii(
            r"######
#....#
#....#
######",
        )
    }

    fn cross() -> AreaGrid {
        from_ascii(
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
}
