use bevy::prelude::*;

use crate::{AppState, SystemLabels};

use super::tile_renderer::{draw, load_char_tiles};

pub struct TileRendererPlugin;

impl Plugin for TileRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(load_char_tiles.after(SystemLabels::Generation))
            .add_system_set(SystemSet::on_update(AppState::Paused).with_system(draw.system()));
    }
}
