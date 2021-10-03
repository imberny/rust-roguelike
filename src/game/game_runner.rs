use bevy::prelude::*;
use rltk::{GameState, Rltk};

use crate::actors::input::systems::poll_input;

// pub struct GameRunner {
//     pub world: World,
//     pub input: Schedule,
//     pub game_logic: Schedule,
//     pub rendering: Schedule,
// }

// impl GameState for GameRunner {
//     fn tick(&mut self, ctx: &mut Rltk) {
//         // update PlayerInput resource
//         poll_input(&mut self.world, ctx);

//         self.input.run(&mut self.world);
//         self.game_logic.run(&mut self.world);
//         self.rendering.run(&mut self.world);

//         // Print
//         render(&mut self.world, ctx);
//     }
// }

// pub fn run_game() -> rltk::RltkError {
//     let gs = init_game();

//     rltk::main_loop(context, gs)
// }
