use std::fs::File;
use std::io::{BufRead, BufReader};


// this is my first ever rust program
// it is awful! but working...
fn main() {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<i32> = buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
    
    // // part 1
    // let mut prev: i32 = lines[0];
    // let mut counter = 0;
    // for i in 1..lines.len() {
    //     let current = lines[i];

    //     if current > prev {
    //         counter = counter + 1;
    //     }
    //     prev = current;    
    // }
    // println!("{}", counter);

    //part 2
    let mut prev_sum_of_window = lines[0] + lines[1] + lines[2];
    let mut new_counter = 0;
    for n in 1..lines.len() - 2 {
        let sum_of_window = lines[n] + lines[n + 1] + lines[n + 2];
        if sum_of_window > prev_sum_of_window {
            new_counter = new_counter + 1;
        }
        prev_sum_of_window = sum_of_window;
    }
    println!("{}", new_counter);
}
