use std::cmp::{max, min};

use bevy::prelude::*;
use rltk::RandomNumberGenerator;

use crate::{
    actors::{Action, Activity, ActorBundle, Player, WeaponBundle},
    ai::Monster,
    core::{
        types::{GridPos, Index, Int},
        MainPointOfView,
    },
    world::{AreaGrid, Renderable, TileType, Viewshed, WorldMap},
};

use super::types::Room;

struct TempMap {
    tiles: Vec<TileType>,
    dimensions: IVec2,
}

impl TempMap {
    pub fn xy_idx(&self, x: Int, y: Int) -> Index {
        ((y * self.dimensions.y) + x) as Index
    }

    pub fn idx_xy(&self, idx: Index) -> (Int, Int) {
        (
            idx as Int % self.dimensions.y,
            idx as Int / self.dimensions.y,
        )
    }
}

pub struct MapGenerator {}

impl MapGenerator {
    fn apply_room_to_map(&self, map: &mut TempMap, room: &Room) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut TempMap, x1: Int, x2: Int, y: Int) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut TempMap, y1: Int, y2: Int, x: Int) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }

    pub fn generate_new_map(&self) -> (TempMap, Vec<Room>) {
        let mut map = TempMap {
            tiles: vec![TileType::Wall; 80 * 50],
            dimensions: IVec2::new(50, 80),
        };
        let mut rooms: Vec<Room> = Vec::new();

        const MAX_ROOMS: Int = 30;
        const MIN_SIZE: Int = 6;
        const MAX_SIZE: Int = 10;

        let mut rng = RandomNumberGenerator::new();

        // for _ in 0..MAX_ROOMS {
        //     let w = rng.range(MIN_SIZE, MAX_SIZE);
        //     let h = rng.range(MIN_SIZE, MAX_SIZE);
        //     let x = rng.roll_dice(1, 80 - w - 1) - 1;
        //     let y = rng.roll_dice(1, 50 - h - 1) - 1;
        //     let new_room = Room::new(x, y, w, h);
        //     if !rooms
        //         .iter()
        //         .any(|other_room| new_room.intersect(other_room))
        //     {
        //         self.apply_room_to_map(&mut map, &new_room);
        //         if !rooms.is_empty() {
        //             let (new_x, new_y) = new_room.center();
        //             let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

        //             if 1 == rng.range(0, 2) {
        //                 self.apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
        //                 self.apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
        //             } else {
        //                 self.apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
        //                 self.apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
        //             }
        //         }

        //         rooms.push(new_room);
        //     }
        // }
        let new_room = Room::new(2, 2, 76, 46);
        self.apply_room_to_map(&mut map, &new_room);
        rooms.push(new_room);
        (map, rooms)
    }
}

pub fn generate_map_system(mut commands: Commands, mut world_map: ResMut<WorldMap>) {
    let (new_map, rooms) = MapGenerator {}.generate_new_map();
    world_map.insert_offset(
        &IVec2::ZERO,
        AreaGrid::from_tiles(&IVec2::new(50, 80), new_map.tiles),
    );

    let mut rng = rltk::RandomNumberGenerator::new();
    for (_i, room) in rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let roll = rng.roll_dice(1, 2);
        let glyph = match roll {
            1 => 'g',
            _ => 'o',
        };

        commands
            .spawn()
            .insert(Monster {})
            .insert_bundle(ActorBundle {
                position: GridPos(IVec2::new(x, y)),
                viewshed: Viewshed::with_range(8),
                renderable: Renderable {
                    glyph,
                    fg: Color::RED,
                    bg: Color::BLACK,
                },
                ..Default::default()
            })
            .with_children(|actor| {
                actor.spawn_bundle(WeaponBundle {
                    position: GridPos(IVec2::new(x, y - 1)),
                    ..Default::default()
                });
            });
    }

    let (player_x, player_y) = rooms[0].center();
    commands
        .spawn()
        .insert(Player)
        .insert(MainPointOfView)
        .insert(Activity {
            action: Action::Wait,
            time_to_complete: 0,
        })
        .insert_bundle(ActorBundle {
            position: GridPos(IVec2::new(player_x, player_y)),
            viewshed: Viewshed::with_range(1),
            renderable: Renderable {
                glyph: '@',
                fg: Color::CYAN,
                bg: Color::BLACK,
            },
            ..Default::default()
        })
        .with_children(|actor| {
            actor.spawn_bundle(WeaponBundle {
                position: GridPos(IVec2::new(player_x, player_y - 1)),
                ..Default::default()
            });
        });
}
