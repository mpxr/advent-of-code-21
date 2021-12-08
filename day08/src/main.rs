use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("no such file");
    let lines = contents.split("\n");
    let mut counter = 0;

    for line in lines {
        counter += line
            .splitn(2, " | ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter(|o| o.chars().count() >= 2 && o.chars().count() <= 4 || o.chars().count() == 7)
            .collect::<Vec<&str>>()
            .len();
    }
    println!("{}", counter);
}
