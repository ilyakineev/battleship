#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipType {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug)]
pub struct Ship {
    pub id: usize,
    pub ship_type: ShipType,
    pub positions: Vec<(u8, u8)>,
    hits: Vec<(u8, u8)>,
    pub is_horizontal: bool,
}

impl Ship {
    pub fn new(id: usize, ship_type: ShipType) -> Self {
        println!("Создаем корабль: {:?} с ID: {}", ship_type, id);
        Self {
            id,
            ship_type,
            positions: Vec::new(),
            hits: Vec::new(),
            is_horizontal: true,
        }
    }

    pub fn get_size(&self) -> u8 {
        match self.ship_type {
            ShipType::Small => 1,
            ShipType::Medium => 2,
            ShipType::Large => 3,
            ShipType::ExtraLarge => 4,
        }
    }

    pub fn add_hit(&mut self, pos: (u8, u8)) {
        self.hits.push(pos);
    }

    pub fn is_sunk(&self) -> bool {
        self.hits.len() as u8 >= self.get_size()
    }
}
