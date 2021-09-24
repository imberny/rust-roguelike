use rltk::{Algorithm2D, BaseMap, Point};

use crate::core::types::{GridPos, Int};

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
    pub width: Int,
    pub height: Int,
    pub revealed: Vec<bool>,
    pub visible: Vec<bool>,
}

impl AreaGrid {
    pub fn xy_idx(&self, x: Int, y: Int) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (Int, Int) {
        (idx as Int % self.width, idx as Int / self.width)
    }

    pub fn is_in_bounds(&self, x: Int, y: Int) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_point_in_bounds(&self, point: GridPos) -> bool {
        self.is_in_bounds(point.x, point.y)
    }

    pub fn at(&self, position: GridPos) -> TileType {
        let idx = self.xy_idx(position.x, position.y);
        self.tiles[idx]
    }

    pub fn index_to_point(&self, index: usize) -> GridPos {
        GridPos::new(index as Int % self.width, index as Int / self.width)
    }

    pub fn is_blocking(&self, position: GridPos) -> bool {
        match self.at(position) {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

impl IntoIterator for AreaGrid {
    type Item = (GridPos, TileType);

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
    type Item = (GridPos, TileType);

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
