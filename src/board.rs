use std::cell::RefCell;
use std::rc::Rc;

pub struct SeaMap {
    x: u8,                                 // Размер по Х
    y: u8,                                 // Размер по Y
    sea_map: Vec<Vec<Rc<RefCell<Field>>>>, // Вектор двумерный с полями
}

pub struct Field {
    status_field: StatusField,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatusField {
    // Статус поля
    Empty, // Пустое
    Ship,  // Корабль
    Hit,   // Подбитый корабль
    Miss,  // Промох
}

impl SeaMap {
    pub fn new(width: u8, height: u8) -> Self {
        println!("Создаем игровое поле");
        let mut map_data: Vec<Vec<Rc<RefCell<Field>>>> = Vec::with_capacity(height as usize);
        for _row_index in 0..height {
            let mut current_row: Vec<Rc<RefCell<Field>>> = Vec::with_capacity(width as usize);
            for _col_index in 0..width {
                // Оборачиваем каждое поле в Rc и RefCell
                current_row.push(Rc::new(RefCell::new(Field {
                    status_field: StatusField::Empty,
                })));
            }
            map_data.push(current_row);
        }

        Self {
            x: width,
            y: height,
            sea_map: map_data,
        }
    }

    // Отрисовать игровую карту
    pub fn draw() {}

    // обновить игровую карту после выстрела
}
