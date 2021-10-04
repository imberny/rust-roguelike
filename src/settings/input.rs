use bevy::prelude::*;
use std::collections::HashMap;

use crate::{actors::Action, core::types::Direction};

pub struct PlayerSettings {
    pub input_map: HashMap<KeyCode, Action>,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            input_map: HashMap::from([
                (KeyCode::Q, Action::Turn(Direction::ForwardLeft)),
                (KeyCode::W, Action::Move(Direction::Forward)),
                (KeyCode::E, Action::Turn(Direction::ForwardRight)),
                (KeyCode::D, Action::Move(Direction::Right)),
                // (KeyCode::C, Action::Move(Direction::BackRight)),
                (KeyCode::S, Action::Move(Direction::Back)),
                // (KeyCode::Z, Action::Move(Direction::BackLeft)),
                (KeyCode::A, Action::Move(Direction::Left)),
                (KeyCode::X, Action::Wait),
                (KeyCode::Period, Action::Wait),
                (KeyCode::Return, Action::InitiateAttack),
                (KeyCode::J, Action::InitiateAttack),
            ]),
        }
    }
}
