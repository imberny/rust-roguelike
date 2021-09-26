use game::run_game;

mod actors;
mod ai;
mod core;
mod game;
mod game_world;
mod generator;
mod rendering;
mod util;

#[cfg(test)]
mod test;

fn main() -> rltk::BError {
    run_game()
}
