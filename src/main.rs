use std::io;

fn main() {
    println!("Тестовый ввод");

    println!("Введите число");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

}
