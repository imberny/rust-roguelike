use crate::components::{Player, Position, Viewshed};
use crate::Map;
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, ent, player, mut viewshed, pos) = data;
        for (ent, viewshed, pos) in (&ent, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| map.is_in_bounds(p.x, p.y));

                let p: Option<&Player> = player.get(ent);
                if let Some(p) = p {
                    for t in map.visible.iter_mut() {
                        *t = false
                    }
                    for vis in &viewshed.visible_tiles {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed[idx] = true;
                        map.visible[idx] = true;
                    }
                }
            }
        }
    }
}
