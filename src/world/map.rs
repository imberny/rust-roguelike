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

pub struct TileHandle<'a> {
    map: &'a AreaGrid,
    index: usize,
}

impl<'a> TileHandle<'a> {
    fn new(grid: &'a AreaGrid, index: usize) -> Self {
        Self { map: grid, index }
    }

    pub fn which(&self) -> TileType {
        self.map.tiles[self.index]
    }

    pub fn is_visible(&self) -> bool {
        self.map.visible[self.index]
    }

    pub fn is_revealed(&self) -> bool {
        self.map.revealed[self.index]
    }
}

pub struct TileHandleMut<'a> {
    map: &'a mut AreaGrid,
    index: usize,
}

impl<'a> TileHandleMut<'a> {
    fn new(grid: &'a mut AreaGrid, index: usize) -> Self {
        Self { map: grid, index }
    }

    pub fn which(&self) -> TileType {
        self.map.tiles[self.index]
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        self.map.visible[self.index] = is_visible;
    }

    pub fn set_revealed(&mut self, is_revealed: bool) {
        self.map.revealed[self.index] = is_revealed;
    }

    pub fn set_renderable(&mut self, renderable: Renderable) {
        let pos = self.map.index_to_point(self.index);
        self.map.renderables.insert(pos, renderable);
    }

    pub fn is_visible(&mut self) -> bool {
        self.map.visible[self.index]
    }

    pub fn is_revealed(&mut self) -> bool {
        self.map.revealed[self.index]
    }
}

impl AreaGrid {
    pub fn new(dimensions: &IVec2) -> Self {
        let tile_count = (dimensions.x * dimensions.y) as usize;
        Self {
            tiles: vec![TileType::Wall; tile_count],
            width: dimensions.y,
            height: dimensions.x,
            revealed: vec![false; tile_count],
            visible: vec![false; tile_count],
            renderables: HashMap::default(),
        }
    }

    pub fn from_tiles(dimensions: &IVec2, tiles: Vec<TileType>) -> Self {
        let tile_count = tiles.len();
        Self {
            tiles,
            width: dimensions.y,
            height: dimensions.x,
            revealed: vec![false; tile_count],
            visible: vec![false; tile_count],
            renderables: HashMap::default(),
        }
    }

    pub fn xy_idx(&self, x: Int, y: Int) -> Index {
        ((y * self.width) + x) as Index
    }

    pub fn tile_at(&self, position: &IVec2) -> Option<TileHandle> {
        if self.is_point_in_bounds(position) {
            let idx = self.xy_idx(position.x, position.y);
            Some(TileHandle::new(self, idx))
        } else {
            None
        }
    }

    pub fn tile_at_mut(&mut self, position: &IVec2) -> Option<TileHandleMut> {
        if self.is_point_in_bounds(position) {
            let idx = self.xy_idx(position.x, position.y);
            Some(TileHandleMut::new(self, idx))
        } else {
            None
        }
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

    pub fn clear_renderables(&mut self) {
        self.renderables.drain();
    }

    fn index_to_point(&self, index: Index) -> IVec2 {
        IVec2::new(index as Int % self.width, index as Int / self.width)
    }

    fn is_point_in_bounds(&self, point: &IVec2) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    fn at(&self, position: &IVec2) -> TileType {
        let idx = self.xy_idx(position.x, position.y);
        self.tiles[idx]
    }
}

impl IntoIterator for AreaGrid {
    type Item = IVec2;

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
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.map.tiles.len() == self.index {
            return None;
        }

        self.index += 1;
        Some(self.map.index_to_point(self.index - 1))
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
