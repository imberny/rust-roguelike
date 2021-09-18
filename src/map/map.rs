use bevy_ecs::prelude::Entity;
use rltk::Point;
use rltk::{Algorithm2D, BaseMap, RandomNumberGenerator};
use std::cmp::{max, min};

use super::rect::Rect;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// pub struct Tile {
//     pub kind: TileType,
//     pub content: Vec<Entity>,
// }

type Room = Rect;

pub struct Map {
    pub tiles: Vec<TileType>,
    // pub contents: Vec<Vec>, // items, actors, ...
    pub rooms: Vec<Room>,
    pub width: i32,
    pub height: i32,
    pub revealed: Vec<bool>,
    pub visible: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_point_in_bounds(&self, point: Point) -> bool {
        self.is_in_bounds(point.x, point.y)
    }

    pub fn at(&self, position: Point) -> TileType {
        let idx = self.xy_idx(position.x, position.y);
        self.tiles[idx]
    }

    pub fn index_to_point(&self, index: usize) -> Point {
        Point::new(index as i32 % self.width, index as i32 / self.width)
    }

    pub fn from_ascii(ascii_map: &str) -> Self {
        let mut map = Self {
            tiles: Vec::new(),
            rooms: Vec::new(),
            width: 0,
            height: 0,
            revealed: Vec::new(),
            visible: Vec::new(),
        };

        let mut rows = ascii_map.split('\n');
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

impl IntoIterator for Map {
    type Item = (Point, TileType);

    type IntoIter = MapIterator;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator {
            map: self,
            index: 0,
        }
    }
}

// struct Tile {
//     pub origin: Point,
//     pub kind: TileType,
// }

pub struct MapIterator {
    map: Map,
    index: usize,
}

impl Iterator for MapIterator {
    type Item = (Point, TileType);

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

pub struct MapGenerator {}

impl MapGenerator {
    fn apply_room_to_map(&self, map: &mut Map, room: &Room) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut Map, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut Map, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    pub fn new_map_rooms_and_corridors(&self) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Room::new(x, y, w, h);
            if !map
                .rooms
                .iter()
                .any(|other_room| new_room.intersect(&other_room))
            {
                self.apply_room_to_map(&mut map, &new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if 1 == rng.range(0, 2) {
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::MapGenerator;

    #[test]
    fn iterate() {
        let map = MapGenerator {}.new_map_rooms_and_corridors();

        for (point, _tile_type) in map.into_iter() {
            println!("{:?}", point);
        }
    }
}
