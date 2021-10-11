use bevy::prelude::*;

use crate::{AppState, SystemLabels};

use super::{
    systems::{draw, load_char_tiles, pre_draw},
    DrawEvent,
};

pub struct TileRendererPlugin;

impl Plugin for TileRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DrawEvent>()
            .add_startup_system(load_char_tiles.after(SystemLabels::Generation))
            .add_system_set(
                SystemSet::on_update(AppState::Paused)
                    .before(SystemLabels::Rendering)
                    .with_system(pre_draw),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Paused)
                    .label(SystemLabels::Rendering)
                    .with_system(draw),
            );
    }
}
