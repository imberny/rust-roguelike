use bevy::{math::IVec2, prelude::Component};

use crate::core::types::Int;

#[derive(Debug, Default, Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<IVec2>,
    pub range: Int,
    pub dirty: bool,
}

impl Viewshed {
    pub fn with_range(range: Int) -> Self {
        Self {
            range,
            dirty: true,
            ..Default::default()
        }
    }
}
