use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    for i in 0..lines.len() {
        let mut iter = lines[i].split_whitespace();
        let command = iter.next().unwrap();
        let x = iter.next().unwrap().parse::<i32>().unwrap();

        if command == "forward" {
            horizontal_position = horizontal_position + x;
        } else if command == "down" {
            depth = depth + x;
        } else if command == "up" {
            depth = depth - x;
        }
    }
    println!(
        "PART 1. Horizontal pos: {}, Depth: {}, Sum: {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );

    let mut fixed_horizontal_pos = 0;
    let mut fixed_depth = 0;
    let mut aim = 0;

    for i in 0..lines.len() {
        let mut iter = lines[i].split_whitespace();
        let command = iter.next().unwrap();
        let x = iter.next().unwrap().parse::<i32>().unwrap();

        if command == "down" {
            aim = aim + x;
        } else if command == "up" {
            aim = aim - x;
        } else if command == "forward" {
            fixed_horizontal_pos = fixed_horizontal_pos + x;
            fixed_depth = fixed_depth + aim * x;
        }
    }

    println!(
        "PART 2. Horizontal pos: {}, Depth: {}, Sum: {}",
        fixed_horizontal_pos,
        fixed_depth,
        fixed_horizontal_pos * fixed_depth
    );
}
