use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("no such file");
    let mut fish: Vec<i16> = contents
        .trim()
        .split(',')
        .map(|v| v.parse::<i16>().expect("eeeh"))
        .collect();

    // lol don't try to run this with 256 days. you tricked me!
    for _ in 0..80 {
        for iter in 0..fish.len() {
            if fish[iter] == 0 {
                fish[iter] = 6;
                fish.push(8);
            } else {
                fish[iter] -= 1;
            }
        }
    }
    println!("PART 1. Number of lanterfish: {}", fish.len());
}