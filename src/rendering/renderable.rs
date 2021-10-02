use rltk::{FontCharType, RGB};

#[derive(Debug, Default, Clone, Copy)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
