use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file_name: String) -> Vec<i32> {
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    return buf
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
}

fn day_1() {
    let lines: Vec<i32> = read_file("src/day_1_input.txt".to_string());
    let mut prev: i32 = lines[0];
    let mut counter = 0;
    for i in 1..lines.len() {
        let current = lines[i];

        if current > prev {
            counter = counter + 1;
        }
        prev = current;
    }
    println!("Part 1: {}", counter);

    let mut prev_sum_of_window = lines[0] + lines[1] + lines[2];
    let mut new_counter = 0;
    for n in 1..lines.len() - 2 {
        let sum_of_window = lines[n] + lines[n + 1] + lines[n + 2];
        if sum_of_window > prev_sum_of_window {
            new_counter = new_counter + 1;
        }
        prev_sum_of_window = sum_of_window;
    }
    println!("Part 2: {}", new_counter);
}

fn day_2() {
    let file = File::open("src/day_2_input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    for i in 0..lines.len() {
        let split: Vec<&str> = lines[i].split(" ").collect();
        let command = split[0];
        let units = split[1].parse::<i32>().unwrap();

        if command == "forward" {
            horizontal_position = horizontal_position + units;
        } else if command == "down" {
            depth = depth + units;
        } else if command == "up" {
            depth = depth - units;
        }
    }
    println!(
        "Horizontal pos: {}, Depth: {}, Sum: {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );
}

fn main() {
    // day_1();
    day_2();
}
