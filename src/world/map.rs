use bevy::{math::IVec2, prelude::Component};

use std::collections::HashMap;

use rltk::{Algorithm2D, BaseMap, Point};

use crate::core::types::{Index, Int};

use super::Renderable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Wall
    }
}

// pub struct Tile {
//     pub kind: TileType,
//     pub content: Vec<Entity>,
// }

#[derive(Debug, Clone, Component)]
pub struct AreaGrid {
    pub tiles: Vec<TileType>,
    pub renderables: HashMap<IVec2, Renderable>,
    pub width: Int,
    pub height: Int,
    pub revealed: Vec<bool>,
    pub visible: Vec<bool>,
}

impl Default for AreaGrid {
    fn default() -> Self {
        Self {
            tiles: Default::default(),
            width: Default::default(),
            height: Default::default(),
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
            renderables: HashMap::default(),
        }
    }
}

impl AreaGrid {
    pub fn xy_idx(&self, x: Int, y: Int) -> Index {
        ((y * self.width) + x) as Index
    }

    pub fn idx_xy(&self, idx: Index) -> (Int, Int) {
        (idx as Int % self.width, idx as Int / self.width)
    }

    pub fn is_in_bounds(&self, x: Int, y: Int) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_point_in_bounds(&self, point: &IVec2) -> bool {
        self.is_in_bounds(point.x, point.y)
    }

    pub fn at(&self, position: &IVec2) -> TileType {
        let idx = self.xy_idx(position.x, position.y);
        self.tiles[idx]
    }

    pub fn index_to_point(&self, index: Index) -> IVec2 {
        IVec2::new(index as Int % self.width, index as Int / self.width)
    }

    pub fn is_blocking(&self, position: &IVec2) -> bool {
        if !self.is_point_in_bounds(position) {
            return false;
        }
        match self.at(position) {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

impl IntoIterator for AreaGrid {
    type Item = (IVec2, TileType);

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
    index: Index,
}

impl Iterator for MapIterator {
    type Item = (IVec2, TileType);

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
    fn is_opaque(&self, idx: Index) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for AreaGrid {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
