use std::io;

pub fn read_coordinates() -> Option<(u8, u8)> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    let input = input.trim().to_uppercase();
    if input.len() < 2 {
        return None;
    }

    let col = input.chars().next()? as u8;
    let row_str: String = input.chars().skip(1).collect();

    let row: u8 = row_str.parse().ok()?;

    if col < b'A' || col > b'J' || row < 1 || row > 10 {
        return None;
    }

    Some((col - b'A', row - 1))
}

pub fn close_game() {
    println!("\nНажмите Enter чтобы выйти...");
    let mut exit = String::new();
    let _ = std::io::stdin().read_line(&mut exit);
}
