use bevy::prelude::*;
use rltk::{Rltk, VirtualKeyCode};
use std::collections::HashMap;

use crate::{
    actors::{input::PlayerInput, Action},
    core::types::{Cardinal, Direction, Index},
};

struct PlayerSettings {
    input_map: HashMap<VirtualKeyCode, Action>,
}

impl PlayerSettings {
    pub fn new() -> Self {
        Self {
            input_map: HashMap::from([
                (VirtualKeyCode::Q, Action::Turn(Direction::ForwardLeft)),
                (VirtualKeyCode::W, Action::Move(Direction::Forward)),
                (VirtualKeyCode::E, Action::Turn(Direction::ForwardRight)),
                (VirtualKeyCode::D, Action::Move(Direction::Right)),
                (VirtualKeyCode::C, Action::Move(Direction::BackRight)),
                (VirtualKeyCode::S, Action::Move(Direction::Back)),
                (VirtualKeyCode::Z, Action::Move(Direction::BackLeft)),
                (VirtualKeyCode::A, Action::Move(Direction::Left)),
                (VirtualKeyCode::X, Action::Wait),
                (VirtualKeyCode::Return, Action::InitiateAttack),
                (VirtualKeyCode::J, Action::InitiateAttack),
            ]),
        }
    }
}

const LEFT_MOUSE_BUTTON: Index = 0;

pub fn poll_input(world: &mut World, ctx: &Rltk) {
    let mut input_resource = world.get_resource_mut::<PlayerInput>().unwrap();
    update_player_input(&mut input_resource, ctx);
}

fn update_player_input(input: &mut PlayerInput, ctx: &Rltk) {
    let rltk_input = rltk::INPUT.lock();
    input.left_click = rltk_input.is_mouse_button_pressed(LEFT_MOUSE_BUTTON);
    input.is_strafing = rltk_input.is_key_pressed(VirtualKeyCode::LShift)
        || rltk_input.is_key_pressed(VirtualKeyCode::RShift);
    input.skew_move = rltk_input.is_key_pressed(VirtualKeyCode::LControl)
        || rltk_input.is_key_pressed(VirtualKeyCode::RControl);
    input.alt = rltk_input.is_key_pressed(VirtualKeyCode::LAlt)
        || rltk_input.is_key_pressed(VirtualKeyCode::RAlt);

    input.cursor_pos = ctx.mouse_pos;
    input.action = if let Some(key) = ctx.key {
        input_to_action(key, input.is_strafing, input.skew_move)
    } else {
        Action::None
    };
}

// Laptop      Numpad         Arrow keys + Control
// ---------------------------------------------------
// Q W E       7 8 9       Ctrl+Left   Up    Ctrl-Up
// S X D       4 5 6          Left     .      Right
// Z S C       1 2 3       Ctrl+Down  Down  Ctrl+Right
pub fn input_to_action(key: VirtualKeyCode, _is_strafing: bool, _skew_move: bool) -> Action {
    let player_settings = PlayerSettings::new();

    if player_settings.input_map.contains_key(&key) {
        return player_settings.input_map[&key];
    }

    // player can't change those
    match key {
        VirtualKeyCode::Numpad7 => Action::Face(Cardinal::NorthWest),
        VirtualKeyCode::Numpad8 => Action::Face(Cardinal::North),
        VirtualKeyCode::Numpad9 => Action::Face(Cardinal::NorthEast),
        VirtualKeyCode::Numpad6 => Action::Face(Cardinal::East),
        VirtualKeyCode::Numpad3 => Action::Face(Cardinal::SouthEast),
        VirtualKeyCode::Numpad2 => Action::Face(Cardinal::South),
        VirtualKeyCode::Numpad1 => Action::Face(Cardinal::SouthWest),
        VirtualKeyCode::Numpad4 => Action::Face(Cardinal::West),

        //     // Shift + 7 => Home
        //     VirtualKeyCode::Home => {
        //         if is_strafing {
        //             Action::Move(NORTH_WEST)
        //         } else {
        //             Action::None
        //         }
        //     }

        //     // Shift + 9 => Page up
        //     VirtualKeyCode::PageUp => {
        //         if is_strafing {
        //             Action::Move(NORTH_EAST)
        //         } else {
        //             Action::None
        //         }
        //     }

        //     // Shift + 3 => Page down
        //     VirtualKeyCode::PageDown => {
        //         if is_strafing {
        //             Action::Move(SOUTH_EAST)
        //         } else {
        //             Action::None
        //         }
        //     }

        //     // Shift + 1 => End
        //     VirtualKeyCode::End => {
        //         if is_strafing {
        //             Action::Move(SOUTH_WEST)
        //         } else {
        //             Action::None
        //         }
        //     }

        //     VirtualKeyCode::Left => {
        //         if skew_move {
        //             Action::Move(SOUTH_WEST)
        //         } else {
        //             Action::Move(WEST)
        //         }
        //     }

        //     VirtualKeyCode::Up => {
        //         if skew_move {
        //             Action::Move(NORTH_WEST)
        //         } else {
        //             Action::Move(NORTH)
        //         }
        //     }

        //     VirtualKeyCode::Right => {
        //         if skew_move {
        //             Action::Move(NORTH_EAST)
        //         } else {
        //             Action::Move(EAST)
        //         }
        //     }

        //     VirtualKeyCode::Down => {
        //         if skew_move {
        //             Action::Move(SOUTH_EAST)
        //         } else {
        //             Action::Move(SOUTH)
        //         }
        //     }
        VirtualKeyCode::Numpad5 => Action::Wait,

        _ => Action::None,
    }
}
