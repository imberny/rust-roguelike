use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use rltk::Point;

use crate::{
    actor::{Action, Actor, Viewshed},
    map::{Map, TileType},
    types::{Facing, Position},
};

fn do_move(pos: &mut Point, viewshed: &mut Viewshed, movement_delta: &Facing, map: &Res<Map>) {
    let destination_idx = map.xy_idx(pos.x + movement_delta.x, pos.y + movement_delta.y);
    let destination_tile = map.tiles[destination_idx];
    if destination_tile != TileType::Wall {
        pos.x = min(79, max(0, pos.x + movement_delta.x));
        pos.y = min(49, max(0, pos.y + movement_delta.y));

        viewshed.dirty = true;
    }
}

pub fn process_move_actions(
    map: Res<Map>,
    mut actors: Query<(&mut Position, &mut Viewshed, &mut Actor)>,
) {
    for (mut pos, mut viewshed, mut actor) in actors.iter_mut() {
        actor.action = match &actor.action {
            Action::None => Action::None,
            Action::Move(direction) => {
                do_move(&mut pos, &mut viewshed, &direction, &map);
                Action::None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{action::Action, Actor, Viewshed},
        map::Map,
        types::Position,
    };

    use super::process_move_actions;

    #[test]
    fn none_action() {
        let mut world = World::new();
        world.insert_resource(Map::new_map_rooms_and_corridors());
        let entity = world
            .spawn()
            .insert_bundle((
                Position::zero(),
                Viewshed::default(),
                Actor {
                    action: Action::None,
                    ..Default::default()
                },
            ))
            .id();
        let mut stage = SystemStage::single(process_move_actions.system());

        // run process_move_action
        stage.run(&mut world);

        // check action is none
        let actor = world.get::<Actor>(entity).unwrap();
        assert_eq!(Action::None, actor.action);
    }
}
