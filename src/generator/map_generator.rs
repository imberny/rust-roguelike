use std::cmp::{max, min};

use bevy_ecs::prelude::World;
use rltk::{RandomNumberGenerator, RGB};

use crate::{
    actors::{Action, Activity, ActorBundle, Player},
    ai::Monster,
    core::types::{GridPos, Int},
    game_world::{AreaGrid, TileType, Viewshed},
    rendering::Renderable,
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

    pub fn new_map_rooms_and_corridors(&self, world: &mut World) {
        let mut map = AreaGrid {
            tiles: vec![TileType::Wall; 80 * 50],
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
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

        add_monsters_to_rooms(world, &rooms);
        let (x, y) = rooms[0].center();
        create_player_at_pos(world, x, y);

        world.insert_resource(map);
    }
}

fn add_monsters_to_rooms(world: &mut World, rooms: &[Room]) {
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let name: String;
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                name = "Goblin".to_string();
                glyph = rltk::to_cp437('g');
            }
            _ => {
                name = "Orc".to_string();
                glyph = rltk::to_cp437('o')
            }
        }

        world
            .spawn()
            .insert(Monster {})
            .insert_bundle(ActorBundle {
                name: format!("{} #{}", &name, i),
                position: GridPos { x, y },
                viewshed: Viewshed::with_range(8),
                ..Default::default()
            })
            .insert(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            });
    }
}

fn create_player_at_pos(world: &mut World, player_x: Int, player_y: Int) {
    world
        .spawn()
        .insert(Player)
        .insert(Activity {
            action: Action::Wait,
            time_to_complete: 5,
        })
        .insert_bundle(ActorBundle {
            name: "Player".to_string(),
            position: GridPos {
                x: player_x,
                y: player_y,
            },
            viewshed: Viewshed::with_range(1),
            ..Default::default()
        })
        .insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        });
}
