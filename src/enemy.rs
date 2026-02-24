use crate::board::SeaMap;
use crate::ship::{Ship, ShipType};

pub struct Enemy {
    sea_map_enemy: SeaMap,
    ships: Vec<Ship>,
}

impl Enemy {
    pub fn new() -> Enemy {
        println!("Создаем противника");
        Enemy {
            sea_map_enemy: SeaMap::new(10, 10),
            ships: Self::create_fleet(),
        }
    }

    fn create_fleet() -> Vec<Ship> {
        let mut ships: Vec<Ship> = Vec::new();

        // Создаем 4 х Small,
        ships.push(Ship::new(ShipType::Small));
        ships.push(Ship::new(ShipType::Small));
        ships.push(Ship::new(ShipType::Small));
        ships.push(Ship::new(ShipType::Small));

        // Создаем 3 х Medium,
        ships.push(Ship::new(ShipType::Medium));
        ships.push(Ship::new(ShipType::Medium));
        ships.push(Ship::new(ShipType::Medium));

        // Создаем 2 х Large,
        ships.push(Ship::new(ShipType::Large));
        ships.push(Ship::new(ShipType::Large));

        // Создаем 1 х ExtraLarge,
        ships.push(Ship::new(ShipType::ExtraLarge));

        return ships;
    }
}
