use crate::enemy::Enemy;
use crate::player::Player;

pub struct Game {
    status_game: StatusGame,
    player: Player,
    enemy: Enemy,
}

impl Game {
    pub fn new() -> Game {
        println!("Создаем игру");
        Game {
            status_game: StatusGame::NewGame,
            player: Player::new(),
            enemy: Enemy::new(),
        }
    }

    pub fn start_game(&mut self) {
        println!("Запуск игры Морской бой.");

        // Основной цикл игры:
        self.set_status(StatusGame::PositionPlayerShips);
        self.set_status(StatusGame::PositionEnemyShips);

        println!("Тут мы раставляем корабли");

        self.set_status(StatusGame::PlayerStep);
        while (self.get_status() == StatusGame::PlayerStep)
            || (self.get_status() == StatusGame::EnemyStep)
        {
            // --> Ход игрока
            println!("Тут по очереди ходит игрок и противник");
            // --> Ход противника
            println!("После каждого хода проверяем условие победы");
            // --> Проверка условия выйграша
            //
            self.set_status(StatusGame::Victory);
        }

        // Поздравление и конец игры
        if self.get_status() == StatusGame::Victory {
            println!("Поздравляем с победой!!!");
        } else if self.get_status() == StatusGame::Defeat {
            println!("К сожалению вы проиграли");
        }

        self.set_status(StatusGame::EndGame);
    }

    pub fn get_status(&self) -> StatusGame {
        self.status_game
    }

    pub fn set_status(&mut self, new_status: StatusGame) {
        self.status_game = new_status;
    }
}

// Этап игры
#[derive(PartialEq, Copy, Clone)]
pub enum StatusGame {
    NewGame,             // Ожидание новой игры
    PositionPlayerShips, // Расстановка кораблей игрока
    PositionEnemyShips,  // Расстановка кораблей противника
    PlayerStep,          // Ход игрока
    EnemyStep,           // Ход противника
    Victory,             // Победа
    Defeat,              // Поражение
    EndGame,
}
