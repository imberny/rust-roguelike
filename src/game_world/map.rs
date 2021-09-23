use rltk::{Algorithm2D, BaseMap, Point};

use crate::core::types::Position;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// pub struct Tile {
//     pub kind: TileType,
//     pub content: Vec<Entity>,
// }

pub struct AreaGrid {
    pub tiles: Vec<TileType>,
    // pub contents: Vec<Vec>, // items, actors, ...
    pub width: i32,
    pub height: i32,
    pub revealed: Vec<bool>,
    pub visible: Vec<bool>,
}

impl AreaGrid {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_point_in_bounds(&self, point: Position) -> bool {
        self.is_in_bounds(point.x, point.y)
    }

    pub fn at(&self, position: Position) -> TileType {
        let idx = self.xy_idx(position.x, position.y);
        self.tiles[idx]
    }

    pub fn index_to_point(&self, index: usize) -> Position {
        Position::new(index as i32 % self.width, index as i32 / self.width)
    }

    pub fn is_blocking(&self, position: Position) -> bool {
        match self.at(position) {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }

    pub fn from_ascii(ascii_map: &str) -> Self {
        let mut map = Self {
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
        map.width = (map.tiles.len() / (map.height as usize)) as i32;

        map
    }
}

impl IntoIterator for AreaGrid {
    type Item = (Position, TileType);

    type IntoIter = MapIterator;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator {
            map: self,
            index: 0,
        }
    }
}

pub struct MapIterator {
    map: AreaGrid,
    index: usize,
}

impl Iterator for MapIterator {
    type Item = (Position, TileType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.map.tiles.len() == self.index {
            return None;
        }

        let result = (
            self.map.index_to_point(self.index),
            self.map.tiles[self.index],
        );
        self.index += 1;
        Some(result)
    }
}

impl BaseMap for AreaGrid {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for AreaGrid {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
