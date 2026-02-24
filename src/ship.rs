use crate::board::Field;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Ship {
    ship_type: ShipType,
    position: Vec<Weak<RefCell<Field>>>,
    is_destroyed: bool,
}

#[derive(Debug)]
pub enum ShipType {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl Ship {
    pub fn new(ship_type: ShipType) -> Self {
        println!("Создаем корабль: {:?}", ship_type);
        Self {
            ship_type: ship_type,
            position: Vec::new(),
            is_destroyed: false,
        }
    }
}
