use bevy::{
    math::{IVec2, Vec2},
    prelude::Component,
};

#[derive(Debug, Clone, Copy, Component)]
pub struct GridPos(pub IVec2);

#[derive(Debug, Clone, Copy, Component)]
pub struct RealPos(pub Vec2);

pub type Predicate<'a, T> = dyn Fn(&T) -> bool + 'a;
