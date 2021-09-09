use bevy_ecs::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum InputStage {
    Poll,
    Handle,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum TurnGameStage {
    First,
    Decision,   // AI and player choose action
    PreUpdate,  // Prepare for update
    Update,     // Run update systems
    PostUpdate, // React to update
    Last,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderingStage {
    Draw,
}
