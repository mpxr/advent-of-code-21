use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let length = lines[0].len();
    let mut dir = vec![0; length];

    for i in 0..lines.len() {
        for (idx, ch) in lines[i].chars().enumerate() {
            if ch == '1' {
                dir[idx] += 1;
            } else {
                dir[idx] -= 1;
            }
        }
    }

    let mut gamma_bin: String = "".to_string();
    let mut epsilon_bin: String = "".to_string();
    for ch in &dir {
        if *ch > 0 {
            gamma_bin.push_str("1");
            epsilon_bin.push_str("0");
        } else if *ch < 0 {
            gamma_bin.push_str("0");
            epsilon_bin.push_str("1");
        }
    }

    let gamma = isize::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_bin, 2).unwrap();
    
    println!(
        "PART 1. Gamma rate: {}, Epsilon rate: {}, Power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
