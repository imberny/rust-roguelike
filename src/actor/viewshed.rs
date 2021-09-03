use crate::types::Position;

#[derive(Debug, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
    pub dirty: bool,
}
