use bevy::prelude::Color;

#[inline]
#[must_use]
pub fn greyscale(color: &Color) -> Color {
    let linear = (color.r() * 0.2126) + (color.g() * 0.7152) + (color.b() * 0.0722);
    Color::rgb(linear, linear, linear)
}
