mod map;
pub use map::{AreaGrid, TileType};

mod viewshed;
pub use viewshed::Viewshed;

pub mod systems;

use bevy::prelude::*;

use crate::{
    core::{
        types::{FontChar, GridPos},
        RenderingStage, TurnGameStage,
    },
    rendering::Renderable,
    AppState,
};

use self::systems::{apply_player_viewsheds, update_viewsheds};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(AppState::Running)
                .label(MapSystems::Viewshed)
                .with_system(update_viewsheds),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Running)
                .with_system(apply_player_viewsheds.after(MapSystems::Viewshed)),
        )
        .add_system_set(SystemSet::on_exit(AppState::Running).with_system(update_renderables));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MapSystems {
    Viewshed,
}

fn update_renderables(
    // mut map: ResMut<AreaGrid>,
    mut map_query: Query<&mut AreaGrid>,
    query: Query<(&GridPos, &Renderable)>,
) {
    let mut map = map_query.single_mut();
    map.renderables.drain();
    query.iter().for_each(|(pos, renderable)| {
        map.renderables.insert(pos.clone(), renderable.clone());
    });
}
