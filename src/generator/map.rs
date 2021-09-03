use bevy_ecs::prelude::World;
use rltk::RGB;

use crate::{
    actor::{ai::Monster, Actor, Name, Viewshed},
    map::Map,
    player::Player,
    rendering::Renderable,
    types::Position,
};

fn add_monsters_to_rooms(world: &mut World, map: &Map) {
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
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
            .insert(Actor::default())
            .insert(Name {
                name: format!("{} #{}", &name, i),
            })
            .insert(Position { x, y })
            .insert(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            });
    }
}

fn create_player_at_pos(world: &mut World, player_x: i32, player_y: i32) {
    world
        .spawn()
        .insert(Player)
        .insert(Actor::default())
        .insert(Name {
            name: "Player".to_string(),
        })
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        });
}

pub fn build_map(world: &mut World) {
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    add_monsters_to_rooms(world, &map);
    world.insert_resource(map);

    create_player_at_pos(world, player_x, player_y);
}
