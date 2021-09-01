use bevy_ecs::prelude::*;
use rltk::VirtualKeyCode;

use crate::{PlayerInput, constants::facings::*, types::Facing};

// Laptop      Numpad         Arrow keys + Control
// ---------------------------------------------------
// Y K U       7 8 9       Ctrl+Left   Up    Ctrl-Up
// H . L       4 5 6          Left     .      Right
// B J N       1 2 3       Ctrl+Down  Down  Ctrl+Right
pub fn input_to_facing(input: Res<PlayerInput>) -> Option<Facing> {
    let direction = match input.key {
        Some(key) => match key {
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => Some(NORTH_WEST),
            VirtualKeyCode::Numpad8 | VirtualKeyCode::K => Some(NORTH),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => Some(NORTH_EAST),
            VirtualKeyCode::Numpad6 | VirtualKeyCode::L => Some(EAST),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => Some(SOUTH_EAST),
            VirtualKeyCode::Numpad2 | VirtualKeyCode::J => Some(SOUTH),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => Some(SOUTH_WEST),
            VirtualKeyCode::Numpad4 | VirtualKeyCode::H => Some(WEST),

            // Shift + 7 => Home
            VirtualKeyCode::Home => {
                if input.is_strafing {
                    Some(NORTH_WEST)
                } else {
                    None
                }
            }

            // Shift + 9 => Page up
            VirtualKeyCode::PageUp => {
                if input.is_strafing {
                    Some(NORTH_EAST)
                } else {
                    None
                }
            }

            // Shift + 3 => Page down
            VirtualKeyCode::PageDown => {
                if input.is_strafing {
                    Some(SOUTH_EAST)
                } else {
                    None
                }
            }

            // Shift + 1 => End
            VirtualKeyCode::End => {
                if input.is_strafing {
                    Some(SOUTH_WEST)
                } else {
                    None
                }
            }

            VirtualKeyCode::Left => {
                if input.skew_move {
                    Some(SOUTH_WEST)
                } else {
                    Some(WEST)
                }
            }

            VirtualKeyCode::Up => {
                if input.skew_move {
                    Some(NORTH_WEST)
                } else {
                    Some(NORTH)
                }
            }

            VirtualKeyCode::Right => {
                if input.skew_move {
                    Some(NORTH_EAST)
                } else {
                    Some(EAST)
                }
            }

            VirtualKeyCode::Down => {
                if input.skew_move {
                    Some(SOUTH_EAST)
                } else {
                    Some(SOUTH)
                }
            }

            VirtualKeyCode::Period | VirtualKeyCode::Numpad5 => Some(Facing::new(0, 0)),

            _ => None,
        },
        None => None,
    };
    direction
}
