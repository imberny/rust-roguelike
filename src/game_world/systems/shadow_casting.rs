use std::collections::HashSet;

use fraction::Fraction;
use rltk::Point;

use crate::game_world::{
    field_of_view::FieldOfView,
    quadrant::{Cardinal, Quadrant, QuadrantRow, QuadrantTile},
    AreaGrid,
};

pub struct SymmetricShadowcaster<'a> {
    area_grid: &'a AreaGrid,
    fov: FieldOfView,
}

impl<'a> SymmetricShadowcaster<'a> {
    pub fn new(area_grid: &'a AreaGrid, fov: FieldOfView) -> Self {
        SymmetricShadowcaster { area_grid, fov }
    }

    pub fn visible_positions(&self, origin: Point) -> Vec<Point> {
        let mut visible_positions: HashSet<Point> = HashSet::new();
        visible_positions.insert(origin);

        for cardinal in Cardinal::iterator() {
            let first_row = QuadrantRow::from_quadrant(Quadrant::new(cardinal, origin), self.fov);
            visible_positions.extend(self.scan_iterative(first_row));
        }

        visible_positions
            .into_iter()
            // .filter(|point| rltk::DistanceAlg::Chebyshev.distance2d(origin, *point) <= range as f32)
            .collect()
    }

    fn scan_iterative(&self, first_row: QuadrantRow) -> HashSet<Point> {
        let mut rows: Vec<QuadrantRow> = vec![first_row];

        let mut visible_positions: HashSet<Point> = HashSet::new();

        while 0 < rows.len() {
            let mut row = rows.pop().unwrap();
            let mut prev_tile: Option<QuadrantTile> = None;
            for tile in row.tiles(self.fov) {
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
        visible_positions: &mut HashSet<Point>,
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
    use rltk::Point;

    use crate::game_world::{field_of_view::FieldOfView, AreaGrid};

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
    fn given_a_rectangular_room_every_square_is_visible() {
        let map = square_room();
        let fov = FieldOfView::infinite();

        let visible_positions =
            SymmetricShadowcaster::new(&map, fov).visible_positions(Point::new(1, 2));

        assert_eq!(map.tiles.len(), visible_positions.len());
        for (position, _tile_type) in map {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_corridor_crossing() {
        let map = cross();
        let fov = FieldOfView::infinite();

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
            SymmetricShadowcaster::new(&map, fov).visible_positions(Point::new(2, 3));

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }

    #[test]
    fn given_a_fov_of_zero_only_the_origin_is_visible() {
        let map = square_room();
        let fov = FieldOfView::new_omni(0);

        let visible_positions =
            SymmetricShadowcaster::new(&map, fov).visible_positions(Point::new(1, 3));

        assert_eq!(visible_positions, [Point::new(1, 3)]);
    }

    #[test]
    fn limited_fov() {
        let fov = FieldOfView::new_omni(2);

        let map = square_room();

        let expected_positions: Vec<Point> = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 1),
            Point::new(2, 2),
        ];

        let visible_positions =
            SymmetricShadowcaster::new(&map, fov).visible_positions(Point::new(1, 1));

        assert_eq!(expected_positions.len(), visible_positions.len());
        for position in expected_positions {
            assert!(visible_positions.contains(&position));
        }
    }
}
