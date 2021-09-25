use std::collections::HashSet;

use fraction::Fraction;

use crate::{
    core::types::GridPos,
    game_world::{
        quadrant::{Cardinal, Quadrant, QuadrantRow, QuadrantTile},
        AreaGrid,
    },
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

pub struct SymmetricShadowcaster<'a> {
    area_grid: &'a AreaGrid,
}

impl<'a> SymmetricShadowcaster<'a> {
    pub fn new(area_grid: &'a AreaGrid) -> Self {
        SymmetricShadowcaster { area_grid }
    }

    pub fn visible_positions<F>(&self, origin: GridPos, is_visible: F) -> Vec<GridPos>
    where
        F: Fn(&GridPos) -> bool,
    {
        let mut visible_positions: HashSet<GridPos> = HashSet::new();
        visible_positions.insert(origin);

        for cardinal in Cardinal::iterator() {
            // let row_provider = RowProvider { cardinal, origin };
            let first_row = QuadrantRow::from_quadrant(Quadrant::new(cardinal, origin));
            visible_positions.extend(self.scan_iterative(first_row, origin, &is_visible));
        }

        visible_positions
            .into_iter()
            // .filter(|point| rltk::DistanceAlg::Chebyshev.distance2d(origin, *point) <= range as Real)
            .collect()
    }

    fn scan_iterative<F>(
        &self,
        first_row: QuadrantRow,
        // row_provider: RowProvider,
        origin: GridPos,
        is_visible: &F,
    ) -> HashSet<GridPos>
    where
        F: Fn(&GridPos) -> bool,
    {
        let mut rows: Vec<QuadrantRow> = vec![first_row];

        let mut visible_positions: HashSet<GridPos> = HashSet::new();

        while !rows.is_empty() {
            let mut row = rows.pop().unwrap();
            let mut prev_tile: Option<QuadrantTile> = None;
            for tile in row.tiles(is_visible, origin) {
                if !self.area_grid.is_point_in_bounds(tile.position) {
                    continue;
                }

                if let Some(next_row) = self.check_next_row(&prev_tile, &tile, &mut row) {
                    rows.push(next_row);
                }

                prev_tile = self.try_reveal_tile(tile, &row, &mut visible_positions);
            }
            if let Some(prev_tile) = prev_tile {
                if !self.area_grid.is_blocking(prev_tile.position) {
                    rows.push(row.next());
                }
            }
        }
        visible_positions
    }

    fn try_reveal_tile(
        &self,
        tile: QuadrantTile,
        row: &QuadrantRow,
        visible_positions: &mut HashSet<GridPos>,
    ) -> Option<QuadrantTile> {
        if self.area_grid.is_blocking(tile.position) || row.is_symmetric(&tile) {
            visible_positions.insert(tile.position);
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
                row.start_slope = self.slope_from(tile);
            }
            if !self.area_grid.is_blocking(prev_tile.position)
                && self.area_grid.is_blocking(tile.position)
            {
                let mut next_row = row.next();
                next_row.end_slope = self.slope_from(tile);
                return Some(next_row);
            }
        }
        None
    }

    fn slope_from(&self, tile: &QuadrantTile) -> Fraction {
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

    use crate::{
        core::{
            constants::SOUTH,
            types::{GridPos, Int},
        },
        game_world::{field_of_view::*, AreaGrid, TileType},
    };

    use super::SymmetricShadowcaster;

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

        let visible_positions = SymmetricShadowcaster::new(&map)
            .visible_positions(GridPos::new(1, 2), |pos| fov.sees(pos));

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

        let visible_positions = SymmetricShadowcaster::new(&map)
            .visible_positions(GridPos::new(2, 3), |pos| fov.sees(pos));

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_fov_of_zero_only_the_origin_is_visible() {
        let map = square_room();
        let fov = new_omni(0);

        let visible_positions = SymmetricShadowcaster::new(&map)
            .visible_positions(GridPos::new(1, 3), |pos| fov.sees(pos));

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

        let visible_positions = SymmetricShadowcaster::new(&map)
            .visible_positions(GridPos::new(1, 1), |pos| fov.sees(pos));

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
