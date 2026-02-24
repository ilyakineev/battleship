use std::io;

pub fn write() -> u8 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Не удалось прочитать строку");
    let number: u8 = number
        .trim()
        .parse()
        .expect("Ошибка преобразования в число");
    number
}

pub fn ask_yes_no() -> bool {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать строку");

    let choice = user_input.trim().to_lowercase();

    choice == "да"
}

pub fn close_game() {
    println!("Нажмите Enter чтобы выйти...");
    let mut exit = String::new();
    let _ = std::io::stdin().read_line(&mut exit);
}
