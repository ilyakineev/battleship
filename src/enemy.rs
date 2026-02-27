use crate::board::SeaMap;
use crate::ship::{Ship, ShipType};

pub struct Enemy {
    pub sea_map_enemy: SeaMap,
    pub ships: Vec<Ship>,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            sea_map_enemy: SeaMap::new(10, 10),
            ships: Vec::new(),
        }
    }

    pub fn setup_fleet(&mut self) {
        self.ships = Self::create_fleet();
        self.sea_map_enemy.place_ships_automatically(&mut self.ships);
    }

    fn create_fleet() -> Vec<Ship> {
        let mut ships = Vec::new();
        let ship_types = [
            ShipType::ExtraLarge,
            ShipType::Large,
            ShipType::Large,
            ShipType::Medium,
            ShipType::Medium,
            ShipType::Medium,
            ShipType::Small,
            ShipType::Small,
            ShipType::Small,
            ShipType::Small,
        ];

        for (i, &ship_type) in ship_types.iter().enumerate() {
            ships.push(Ship::new(i, ship_type));
        }
        ships
    }

    pub fn draw_sea_map(&self) {
        self.sea_map_enemy.draw(true);
    }
}
