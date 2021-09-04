use crate::types::Position;

#[derive(Debug, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>,
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
