use bevy_ecs::prelude::*;
use fraction::Fraction;
use rltk::{field_of_view, Point};

use crate::{actor::Viewshed, core::types::Position, map::Map};

use super::shadow_casting::symmetric_shadowcasting;

enum Cardinal {
    North,
    East,
    South,
    West,
}

struct Quadrant {
    pub cardinal: Cardinal,
    pub origin: Point,
}

impl Quadrant {
    pub fn new(cardinal: Cardinal, origin: Point) -> Self {
        Self { cardinal, origin }
    }
}

struct Row {
    pub depth: i32,
    pub start_slope: Fraction,
    pub end_slope: Fraction,
}

impl Row {
    fn new(depth: i32, start_slope: Fraction, end_slope: Fraction) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }
}

// https://www.albertford.com/shadowcasting/
// fn symmetric_shadowcasting(map: &Map, start: Point, range: i32) -> Vec<Point> {
//     let mut visible_tiles: Vec<Point> = Vec::new();
//     visible_tiles.push(start);

//     let cardinal_directions = [
//         Cardinal::North,
//         Cardinal::East,
//         Cardinal::South,
//         Cardinal::West,
//     ];
//     for cardinal in cardinal_directions {
//         let quadrant = Quadrant::new(cardinal, start);

//         let first_row = Row::new(1, Fraction::new_neg(1u16, 1u32), Fraction::new(1u16, 1u32));
//         scan_iterative(first_row);
//     }

//     visible_tiles
// }

pub fn update_viewsheds(map: ResMut<Map>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles =
                symmetric_shadowcasting(&map, Point::new(pos.x, pos.y), viewshed.range as usize);
            // viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            // viewshed
            // .visible_tiles
            // .retain(|p| map.is_in_bounds(p.x, p.y));
        }
    }
}
