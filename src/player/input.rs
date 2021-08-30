use bevy_ecs::prelude::*;
use rltk::{Point, VirtualKeyCode};

use crate::PlayerInput;

pub type Direction = Point;

// Laptop      Numpad         Arrow keys + Control
// ---------------------------------------------------
// Y K U       7 8 9       Ctrl+Left   Up    Ctrl-Up
// H . L       4 5 6          Left     .      Right
// B J N       1 2 3       Ctrl+Down  Down  Ctrl+Right
pub fn direction_from(input: Res<PlayerInput>) -> Option<Direction> {
    let direction = match input.key {
        Some(key) => match key {
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => Some(Point::new(-1, -1)),

            VirtualKeyCode::Numpad8 | VirtualKeyCode::K => Some(Point::new(0, -1)),

            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => Some(Point::new(1, -1)),

            VirtualKeyCode::Numpad6 | VirtualKeyCode::L => Some(Point::new(1, 0)),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => Some(Point::new(1, 1)),

            VirtualKeyCode::Numpad2 | VirtualKeyCode::J => Some(Point::new(0, 1)),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => Some(Point::new(-1, 1)),

            VirtualKeyCode::Numpad4 | VirtualKeyCode::H => Some(Point::new(-1, 0)),

            VirtualKeyCode::Left => {
                if input.control {
                    Some(Point::new(-1, 1))
                } else {
                    Some(Point::new(-1, 0))
                }
            }

            VirtualKeyCode::Up => {
                if input.control {
                    Some(Point::new(-1, -1))
                } else {
                    Some(Point::new(0, -1))
                }
            }

            VirtualKeyCode::Right => {
                if input.control {
                    Some(Point::new(1, -1))
                } else {
                    Some(Point::new(1, 0))
                }
            }

            VirtualKeyCode::Down => {
                if input.control {
                    Some(Point::new(1, 1))
                } else {
                    Some(Point::new(0, 1))
                }
            }

            VirtualKeyCode::Period | VirtualKeyCode::Numpad5 => Some(Point::new(0, 0)),

            _ => None,
        },
        None => None,
    };
    direction
}