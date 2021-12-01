use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(file_name: String) -> Vec<i32> {
    let file = File::open(file_name).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
}

fn part_1 (lines: &[i32]) {
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
}

fn part_2(lines: &[i32]) {
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

fn main() {
    let lines: Vec<i32> = read_file("src/input.txt".to_string());

    part_1(&lines);

    part_2(&lines);

}
