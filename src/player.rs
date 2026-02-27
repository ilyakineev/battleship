use crate::board::SeaMap;
use crate::ship::{Ship, ShipType};

pub struct Player {
    pub sea_map_player: SeaMap,
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            sea_map_player: SeaMap::new(10, 10),
            ships: Vec::new(),
        }
    }

    pub fn setup_fleet(&mut self) {
        self.ships = Self::create_fleet();
        self.sea_map_player
            .place_ships_automatically(&mut self.ships);
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
        self.sea_map_player.draw(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldStatus;

    #[test]
    fn test_automatic_placement_rules() {
        let mut player = Player::new();
        player.setup_fleet();

        let mut total_ship_cells = 0;
        let mut ship_cell_coords = Vec::new();
        let (width, height) = player.sea_map_player.get_dimensions();

        for ship in &player.ships {
            assert!(
                !ship.positions.is_empty(),
                "Корабль {:?} (ID: {}) не был размещен.",
                ship.ship_type,
                ship.id
            );
        }

        for y in 0..height {
            for x in 0..width {
                if let FieldStatus::Occupied(_) =
                    player.sea_map_player.fields[y as usize][x as usize].status
                {
                    total_ship_cells += 1;
                    ship_cell_coords.push((x, y));
                }
            }
        }

        let expected_cells = player
            .ships
            .iter()
            .map(|s| s.get_size() as u32)
            .sum::<u32>();
        assert_eq!(
            total_ship_cells, expected_cells,
            "Неверное количество ячеек кораблей на доске."
        );

        for &(x, y) in &ship_cell_coords {
            let original_id = if let FieldStatus::Occupied(id) =
                player.sea_map_player.fields[y as usize][x as usize].status
            {
                id
            } else {
                continue;
            };

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let check_x_i = x as i16 + dx;
                    let check_y_i = y as i16 + dy;

                    if check_x_i >= 0
                        && check_x_i < width as i16
                        && check_y_i >= 0
                        && check_y_i < height as i16
                    {
                        let check_x = check_x_i as u8;
                        let check_y = check_y_i as u8;

                        if let FieldStatus::Occupied(adjacent_id) =
                            player.sea_map_player.fields[check_y as usize][check_x as usize].status
                        {
                            if original_id != adjacent_id {
                                panic!(
                                    "Ошибка смежности: ячейка корабля в ({}, {}) (корабль {}) находится рядом с другой ячейкой корабля в ({}, {}) (корабль {})",
                                    x, y, original_id, check_x, check_y, adjacent_id
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
