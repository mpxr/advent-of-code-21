use std::fs::File;
use std::io::{BufRead, BufReader};


// this is my first ever rust program
// it is awful! but working...
fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    
    // part 1
    let mut prev: i32 = 0;
    let mut counter = 0;
    for (index, line) in reader.lines().enumerate() {
        let meter = line.unwrap().parse::<i32>().unwrap();

        if index > 0 {
            if meter > prev {
                counter = counter + 1;
            }
        }
        prev = meter;    
    }
    println!("{}", counter);
}
