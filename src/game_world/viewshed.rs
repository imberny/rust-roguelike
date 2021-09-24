use crate::core::types::{GridPos, Int};

#[derive(Debug, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<GridPos>,
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
