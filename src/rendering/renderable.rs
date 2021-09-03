use rltk::{FontCharType, RGB};

#[derive(Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
