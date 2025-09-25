use std::io;

fn process_word() {
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("failed to read line!");
    let trimmed_word = word.trim();

    if trimmed_word.len() <= 10 {
        println!("{}", trimmed_word);
    } else {
        let first_char = trimmed_word.chars().next().unwrap();
        let last_char = trimmed_word.chars().last().unwrap();
        let middle_length = trimmed_word.len() - 2;
        let abbreviation = format!("{}{}{}", first_char, middle_length, last_char);
        println!("{}", abbreviation);
    }
}

fn main() {
    let mut n_text = String::new();
    io::stdin()
        .read_line(&mut n_text)
        .expect("Failed to read line!");
    let n: i32 = n_text.trim().parse().expect("n must be a number!");

    for _ in 0..n {
        process_word();
    }
}
