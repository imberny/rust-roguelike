use serde::Deserialize;

use crate::{
    core::types::{Cardinal, GridPos, Index, Int, Real},
    game_world::{AreaGrid, TileType},
    rendering::Renderable,
    util::helpers::deserialize,
};

pub fn cases() -> impl Iterator<Item = TestMap> {
    let cases: TestMapCases = deserialize("src/test/data/maps.ron");
    cases.cases.into_iter()
}

#[derive(Debug, Deserialize)]
pub struct TestMap {
    pub range: Int,
    pub a: Real,
    pub b: Real,
    pub cardinal: Cardinal,
    pub layout: String,
    pub expected_visible: String,
}

#[derive(Debug, Deserialize)]
struct TestMapCases {
    pub cases: Vec<TestMap>,
}

pub fn from_ascii_layout(ascii_map: &str) -> (GridPos, AreaGrid) {
    let mut origin = GridPos::zero();
    let mut tiles: Vec<TileType> = vec![];
    let width = ascii_map.trim_start().find('\n').unwrap() as Int;

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
                    origin.x = x as Int;
                    origin.y = y as Int;
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
        revealed: vec![false; (width * height) as Index],
        visible: vec![false; (width * height) as Index],
        ..Default::default()
    };
    (origin, map)
}

pub fn from_ascii_expected(ascii_map: &str) -> Vec<GridPos> {
    let mut visible_positions: Vec<GridPos> = vec![];

    let width = ascii_map.find('\n').unwrap() as Int;
    let rows = ascii_map.split('\n');
    let mut y = 0;
    for row in rows {
        assert_eq!(width, row.trim_start().len() as Int);
        for (x, char) in row.trim_start().char_indices() {
            if char == 'y' {
                visible_positions.push(GridPos::new(x as Int, y as Int))
            }
        }
        y += 1
    }

    visible_positions
}
