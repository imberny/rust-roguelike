use rltk::{Point, RGB};


pub type Position = Point;

pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Monster {}

#[derive(Debug)]
pub struct Name {
    pub name: String,
}

pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}
