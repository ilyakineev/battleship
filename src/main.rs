mod board;
mod enemy;
mod field;
mod game;
mod player;
mod ship;
mod utils;

fn main() {
    let mut game = game::Game::new();
    game.start();
    utils::close_game();
}
