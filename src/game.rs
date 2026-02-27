use crate::enemy::Enemy;
use crate::field::ShotStatus;
use crate::player::Player;
use crate::utils;
use rand::Rng;
use std::thread;
use std::time::Duration;

pub struct Game {
    status_game: StatusGame,
    player: Player,
    enemy: Enemy,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum StatusGame {
    PlayerStep,
    EnemyStep,
    EndGame(Winner),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Winner {
    Player,
    Enemy,
}

impl Game {
    pub fn new() -> Game {
        println!("Создаем игру...");
        let mut player = Player::new();
        player.setup_fleet();

        let mut enemy = Enemy::new();
        enemy.setup_fleet();

        Game {
            status_game: StatusGame::PlayerStep,
            player,
            enemy,
        }
    }

    pub fn start(&mut self) {
        println!("--- Начало игры Морской бой! ---");

        loop {
            self.draw_boards();

            match self.status_game {
                StatusGame::PlayerStep => {
                    self.handle_player_turn();
                    if self.check_win_condition() {
                        break;
                    }
                    self.status_game = StatusGame::EnemyStep;
                }
                StatusGame::EnemyStep => {
                    self.handle_enemy_turn();
                    if self.check_win_condition() {
                        break;
                    }
                    self.status_game = StatusGame::PlayerStep;
                }
                StatusGame::EndGame(_) => {
                    break;
                }
            }
        }
        self.draw_boards();
        self.display_winner();
    }

    fn handle_player_turn(&mut self) {
        println!("Ваш ход. Введите координаты (например, 'A1' или 'h10'):");
        loop {
            if let Some((x, y)) = utils::read_coordinates() {
                match self.enemy.sea_map_enemy.take_shot(x, y) {
                    Some((ShotStatus::Hit, Some(ship_id))) => {
                        println!("Попадание!");
                        self.enemy.ships[ship_id].add_hit((x, y));
                        if self.enemy.ships[ship_id].is_sunk() {
                            println!("Корабль противника потоплен!");
                        }
                        self.draw_boards();
                        if self.check_win_condition() {
                            return;
                        }
                        println!("Вы попали! Ходите еще раз:");
                    }
                    Some((ShotStatus::Miss, _)) => {
                        println!("Промах!");
                        break;
                    }
                    None => {
                        println!(
                            "Вы уже стреляли в эту клетку или ввели неверные координаты. Попробуйте снова."
                        );
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                println!("Неверный формат ввода. Попробуйте снова (например, 'A1').");
            }
        }
    }

    fn handle_enemy_turn(&mut self) {
        println!("Ход противника...");
        thread::sleep(Duration::from_secs(3));

        loop {
            let mut rng = rand::thread_rng();
            let (width, height) = self.player.sea_map_player.get_dimensions();
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if self.player.sea_map_player.fields[y as usize][x as usize]
                .shot
                .is_none()
            {
                match self.player.sea_map_player.take_shot(x, y) {
                    Some((ShotStatus::Hit, Some(ship_id))) => {
                        println!(
                            "Противник попал по вашему кораблю в {}{}",
                            (b'A' + x) as char,
                            y + 1
                        );
                        self.player.ships[ship_id].add_hit((x, y));
                        if self.player.ships[ship_id].is_sunk() {
                            println!("Ваш корабль потоплен!");
                        }
                        if self.check_win_condition() {
                            return;
                        }
                    }
                    Some((ShotStatus::Miss, _)) => {
                        println!("Противник промахнулся.");
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
    }

    fn check_win_condition(&mut self) -> bool {
        if self.enemy.ships.iter().all(|s| s.is_sunk()) {
            self.status_game = StatusGame::EndGame(Winner::Player);
            return true;
        }
        if self.player.ships.iter().all(|s| s.is_sunk()) {
            self.status_game = StatusGame::EndGame(Winner::Enemy);
            return true;
        }
        false
    }

    fn draw_boards(&self) {
        println!("========================================");
        println!("          ПОЛЕ ПРОТИВНИКА");
        self.enemy.draw_sea_map();
        println!("             ВАШЕ ПОЛЕ");
        self.player.draw_sea_map();
        println!("========================================");
    }

    fn display_winner(&self) {
        if let StatusGame::EndGame(winner) = self.status_game {
            match winner {
                Winner::Player => println!("--- ПОЗДРАВЛЯЕМ! ВЫ ПОБЕДИЛИ! ---"),
                Winner::Enemy => println!("--- К СОЖАЛЕНИЮ, ВЫ ПРОИГРАЛИ. ---"),
            }
        }
    }
}
