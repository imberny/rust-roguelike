use std::cmp::{max, min};

use bevy::prelude::*;
use rltk::RandomNumberGenerator;

use crate::{
    actors::{Action, Activity, ActorBundle, Player},
    ai::Monster,
    core::types::{FontChar, GridPos, Int},
    util::helpers::cp437,
    world::{AreaGrid, Renderable, TileType, Viewshed},
};

use super::types::Room;

pub struct MapGenerator {}

impl MapGenerator {
    fn apply_room_to_map(&self, map: &mut AreaGrid, room: &Room) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut AreaGrid, x1: Int, x2: Int, y: Int) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut AreaGrid, y1: Int, y2: Int, x: Int) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    pub fn generate_new_map(&self) -> (AreaGrid, Vec<Room>) {
        let mut map = AreaGrid {
            tiles: vec![TileType::Wall; 80 * 50],
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
            ..Default::default()
        };
        let mut rooms: Vec<Room> = Vec::new();

        const MAX_ROOMS: Int = 30;
        const MIN_SIZE: Int = 6;
        const MAX_SIZE: Int = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Room::new(x, y, w, h);
            if !rooms
                .iter()
                .any(|other_room| new_room.intersect(other_room))
            {
                self.apply_room_to_map(&mut map, &new_room);
                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    if 1 == rng.range(0, 2) {
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }
        (map, rooms)
    }
}

pub fn generate_map_system(mut commands: Commands, mut map_query: Query<&mut AreaGrid>) {
    let mut map = map_query.single_mut();

    let (new_map, rooms) = MapGenerator {}.generate_new_map();
    map.tiles = new_map.tiles;
    map.width = new_map.width;
    map.height = new_map.height;

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let name: String;
        let glyph: FontChar;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                name = "Goblin".to_string();
                glyph = cp437('g');
            }
            _ => {
                name = "Orc".to_string();
                glyph = cp437('o')
            }
        }

        commands
            .spawn()
            .insert(Monster {})
            .insert_bundle(ActorBundle {
                name: format!("{} #{}", &name, i),
                position: GridPos::new(x, y),
                viewshed: Viewshed::with_range(8),
                ..Default::default()
            })
            .insert(Renderable {
                glyph,
                fg: Color::RED,
                bg: Color::BLACK,
            });
    }

    let (player_x, player_y) = rooms[0].center();
    commands
        .spawn()
        .insert(Player)
        .insert(Activity {
            action: Action::Wait,
            time_to_complete: 0,
        })
        .insert_bundle(ActorBundle {
            name: "Player".to_string(),
            position: GridPos::new(player_x, player_y),
            viewshed: Viewshed::with_range(1),
            ..Default::default()
        })
        .insert(Renderable {
            glyph: cp437('@'),
            fg: Color::CYAN,
            bg: Color::BLACK,
        });
}
