use bevy_ecs::prelude::World;
use rltk::RGB;

use crate::{
    actor::{ai::Monster, player::Player, Action, Activity, ActorBundle, Viewshed},
    core::types::Position,
    map::Map,
    rendering::Renderable,
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
            .insert_bundle(ActorBundle {
                name: format!("{} #{}", &name, i),
                position: Position { x, y },
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

fn create_player_at_pos(world: &mut World, player_x: i32, player_y: i32) {
    world
        .spawn()
        .insert(Player)
        .insert(Activity {
            action: Action::Wait,
            time_to_complete: 5,
        })
        .insert_bundle(ActorBundle {
            name: "Player".to_string(),
            position: Position {
                x: player_x,
                y: player_y,
            },
            viewshed: Viewshed::with_range(8),
            ..Default::default()
        })
        .insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        });
}

pub fn build_map(world: &mut World) {
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    add_monsters_to_rooms(world, &map);
    world.insert_resource(map);

    create_player_at_pos(world, player_x, player_y);
}
