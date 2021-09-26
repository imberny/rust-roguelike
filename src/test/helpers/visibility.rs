use ron::de;

use crate::{
    core::types::{GridPos, Index, Int},
    game_world::{AreaGrid, TileType},
    test::visibility::TestMapCases,
};

pub fn read_test_cases() -> TestMapCases {
    let current_dir = std::env::current_dir().unwrap();
    let path = format!(
        "{}/{}",
        current_dir.to_str().unwrap(),
        "src/test/data/maps.ron"
    );
    let maps = std::fs::File::open(&path).expect("Failed opening file");

    match de::from_reader(maps) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn from_ascii_layout(ascii_map: &str) -> (GridPos, AreaGrid) {
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
