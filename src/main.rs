use game::run_game;

mod actor;
mod constants;
mod game;
mod generator;
mod initialization;
mod map;
mod player;
mod rendering;
mod types;
mod util;

fn main() -> rltk::BError {
    run_game()
}
