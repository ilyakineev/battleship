mod board;
mod enemy;
mod game;
mod player;
mod ship;
mod utils;

use crate::game::Game;
use crate::utils::ask_yes_no;
use crate::utils::close_game;

fn main() {
    loop {
        let mut game: Game = Game::new();
        game.start_game();

        // Запрос повторени игры или выход
        println!("Хотети повторить игру?");
        if !ask_yes_no() {
            close_game();
            return;
        }
    }
}
