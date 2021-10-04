use bevy::math::{IVec2, Vec2};

pub type GridPos = IVec2;
pub type RealPos = Vec2;

pub type GridPosPredicate<'a> = dyn Fn(&GridPos) -> bool + 'a;
