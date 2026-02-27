use crate::field::{Field, FieldStatus, ShotStatus};
use crate::ship::Ship;
use rand::{Rng, thread_rng};

pub struct SeaMap {
    width: u8,
    height: u8,
    pub fields: Vec<Vec<Field>>,
}

impl SeaMap {
    pub fn new(width: u8, height: u8) -> Self {
        let fields = vec![vec![Field::new(); width as usize]; height as usize];
        Self {
            width,
            height,
            fields,
        }
    }

    pub fn place_ships_automatically(&mut self, ships: &mut [Ship]) {
        let mut rng = thread_rng();
        for ship in ships.iter_mut() {
            loop {
                let is_horizontal = rng.gen_bool(0.5);
                let (x, y) = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));

                if self.can_place_ship(x, y, ship.get_size(), is_horizontal) {
                    self.perform_ship_placement(ship, x, y, is_horizontal);
                    break;
                }
            }
        }
    }

    fn can_place_ship(&self, x: u8, y: u8, ship_size: u8, is_horizontal: bool) -> bool {
        if (is_horizontal && x + ship_size > self.width)
            || (!is_horizontal && y + ship_size > self.height)
        {
            return false;
        }

        let x_min = x.saturating_sub(1);
        let y_min = y.saturating_sub(1);
        let (x_max, y_max) = if is_horizontal {
            (
                (x + ship_size).min(self.width - 1),
                (y + 1).min(self.height - 1),
            )
        } else {
            (
                (x + 1).min(self.width - 1),
                (y + ship_size).min(self.height - 1),
            )
        };

        for check_y in y_min..=y_max {
            for check_x in x_min..=x_max {
                if self.fields[check_y as usize][check_x as usize].status != FieldStatus::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn perform_ship_placement(&mut self, ship: &mut Ship, x: u8, y: u8, is_horizontal: bool) {
        ship.is_horizontal = is_horizontal;
        ship.positions.clear();

        for i in 0..ship.get_size() {
            let (pos_x, pos_y) = if is_horizontal {
                (x + i, y)
            } else {
                (x, y + i)
            };
            self.fields[pos_y as usize][pos_x as usize].status = FieldStatus::Occupied(ship.id);
            ship.positions.push((pos_x, pos_y));
        }
    }

    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    pub fn take_shot(&mut self, x: u8, y: u8) -> Option<(ShotStatus, Option<usize>)> {
        if x >= self.width || y >= self.height || self.fields[y as usize][x as usize].shot.is_some()
        {
            return None;
        }

        let field = &mut self.fields[y as usize][x as usize];
        match field.status {
            FieldStatus::Empty => {
                field.shot = Some(ShotStatus::Miss);
                Some((ShotStatus::Miss, None))
            }
            FieldStatus::Occupied(ship_id) => {
                field.shot = Some(ShotStatus::Hit);
                Some((ShotStatus::Hit, Some(ship_id)))
            }
        }
    }

    pub fn draw(&self, is_enemy_board: bool) {
        print!("   ");
        for i in 0..self.width {
            print!("{} ", (b'A' + i) as char);
        }
        println!();

        for (i, row) in self.fields.iter().enumerate() {
            print!("{:2} ", i + 1);
            for field in row {
                let symbol = match field.shot {
                    Some(ShotStatus::Hit) => "X",
                    Some(ShotStatus::Miss) => "o",
                    None => match field.status {
                        FieldStatus::Empty => "~",
                        FieldStatus::Occupied(_) => {
                            if is_enemy_board {
                                "~"
                            } else {
                                "■"
                            }
                        }
                    },
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ship::{Ship, ShipType};

    #[test]
    fn test_new_seamap() {
        let map = SeaMap::new(10, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.fields.len(), 10);
        assert_eq!(map.fields[0].len(), 10);
        for row in map.fields {
            for field in row {
                assert_eq!(field.status, FieldStatus::Empty);
                assert!(field.shot.is_none());
            }
        }
    }

    #[test]
    fn test_take_shot() {
        let mut map = SeaMap::new(10, 10);
        let mut ship = Ship::new(0, ShipType::ExtraLarge);
        map.perform_ship_placement(&mut ship, 1, 1, true);

        let (status, ship_id) = map.take_shot(0, 0).unwrap();
        assert_eq!(status, ShotStatus::Miss);
        assert!(ship_id.is_none());
        assert_eq!(map.fields[0][0].shot, Some(ShotStatus::Miss));

        let (status, ship_id) = map.take_shot(2, 1).unwrap();
        assert_eq!(status, ShotStatus::Hit);
        assert_eq!(ship_id, Some(ship.id));
        assert_eq!(map.fields[1][2].shot, Some(ShotStatus::Hit));

        assert!(map.take_shot(2, 1).is_none());
        assert!(map.take_shot(10, 10).is_none());
    }

    #[test]
    fn test_can_place_ship_valid() {
        let map = SeaMap::new(10, 10);
        assert!(map.can_place_ship(0, 0, 4, true));
        assert!(map.can_place_ship(5, 5, 3, false));
    }

    #[test]
    fn test_can_place_ship_invalid_bounds() {
        let map = SeaMap::new(10, 10);
        assert!(!map.can_place_ship(7, 0, 4, true));
        assert!(!map.can_place_ship(0, 8, 3, false));
    }

    #[test]
    fn test_can_place_ship_invalid_adjacency() {
        let mut map = SeaMap::new(10, 10);
        let mut ship = Ship::new(0, ShipType::Large);
        map.perform_ship_placement(&mut ship, 2, 2, true);

        assert!(!map.can_place_ship(2, 3, 2, true));
        assert!(!map.can_place_ship(1, 1, 2, true));
        assert!(!map.can_place_ship(3, 2, 2, true));
    }

    #[test]
    fn test_place_ships_automatically() {
        let mut map = SeaMap::new(10, 10);
        let mut ships = vec![
            Ship::new(0, ShipType::ExtraLarge), // 4
            Ship::new(1, ShipType::Large),      // 3
            Ship::new(2, ShipType::Medium),     // 2
            Ship::new(3, ShipType::Medium),     // 2
            Ship::new(4, ShipType::Small),      // 1
        ];
        map.place_ships_automatically(&mut ships);

        let expected_occupied_fields: usize = ships.iter().map(|s| s.get_size() as usize).sum();
        let mut actual_occupied_fields = 0;
        for row in &map.fields {
            for field in row {
                if let FieldStatus::Occupied(_) = field.status {
                    actual_occupied_fields += 1;
                }
            }
        }
        assert_eq!(actual_occupied_fields, expected_occupied_fields);
    }
}
