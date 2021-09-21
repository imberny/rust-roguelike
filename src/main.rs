use game::run_game;

mod actor;
mod ai;
mod core;
mod game;
mod game_world;
mod generator;
mod rendering;
mod util;

fn main() -> rltk::BError {
    run_game()
}
