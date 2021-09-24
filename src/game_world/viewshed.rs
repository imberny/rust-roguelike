use crate::core::types::GridPos;

#[derive(Debug, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<GridPos>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn with_range(range: i32) -> Self {
        Self {
            range,
            dirty: true,
            ..Default::default()
        }
    }
}
