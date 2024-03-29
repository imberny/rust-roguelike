use bevy::math::IVec2;
use serde::Deserialize;

use crate::{
    core::types::{Cardinal, Index, Int, Real},
    util::helpers::deserialize,
    world::{AreaGrid, TileType},
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

pub fn from_ascii_layout(ascii_map: &str) -> (IVec2, AreaGrid) {
    let mut origin = IVec2::ZERO;
    let mut tiles: Vec<TileType> = vec![];
    let width = ascii_map.trim_start().find('\n').unwrap() as Int;

    let row_count = ascii_map.split('\n').count();

    let rows = ascii_map.split('\n');
    let mut y = 0;
    for row in rows {
        assert_eq!(width, row.trim_start().len() as Int);
        for (index, c) in row.trim_start().char_indices() {
            match c {
                '.' => tiles.push(TileType::Floor),
                '#' => tiles.push(TileType::Wall),
                '@' => {
                    tiles.push(TileType::Floor);
                    // origin.x = (index as Int) - (width / 2);
                    origin.x = index as Int;
                    origin.y = y as Int;
                }

                _ => panic!("Unrecognized map tile: {:?}", c),
            }
        }
        y += 1;
    }
    let height = row_count as Int;

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

pub fn from_ascii_expected(ascii_map: &str) -> Vec<IVec2> {
    let mut visible_positions: Vec<IVec2> = vec![];

    let width = ascii_map.find('\n').unwrap() as Int;
    let rows = ascii_map.split('\n');
    let mut y = 0;
    for row in rows {
        assert_eq!(width, row.trim_start().len() as Int);
        for (index, char) in row.trim_start().char_indices() {
            if char == '0' {
                visible_positions.push(IVec2::new(index as Int, y as Int))
            }
        }
        y += 1;
    }

    visible_positions
}

pub fn from_ascii_axis_positions(ascii: &str) -> Vec<IVec2> {
    let mut positions: Vec<IVec2> = vec![];

    let row_count = ascii.split('\n').count();

    let width = ascii.trim_start().find('\n').unwrap() as Int;
    let rows = ascii.split('\n');
    let mut y = -(row_count as Int / 2);
    for row in rows {
        assert_eq!(width, row.trim_start().len() as Int);
        for (index, char) in row.trim_start().char_indices() {
            if char == '0' {
                let x = (index as Int) - (width / 2);
                positions.push(IVec2::new(x, y))
            }
        }
        y += 1;
    }
    positions
}
