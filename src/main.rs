use game::run_game;

mod actor;
mod core;
mod game;
mod generator;
mod map;
mod rendering;
mod util;

fn main() -> rltk::BError {
    run_game()
}
