use std::io::{self, BufRead};

fn main() {
    // Вектор для хранения уникальных строк
    let mut unique_lines = Vec::new();

    // Чтение строк из стандартного ввода
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(line) => {
                // Проверка наличия строки в векторе
                if !unique_lines.contains(&line) {
                    unique_lines.push(line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Вывод уникальных строк
    for line in unique_lines {
        println!("{}", line);
    }
}
