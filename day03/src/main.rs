use std::fs::File;
use std::io::{BufRead, BufReader};

fn reduce(ls: &mut [String], pos: usize, criteria: String) -> String {
    if ls.len() == 1 {
        return ls.first().unwrap().to_string();
    }

    let mut ones: Vec<String> = Vec::new();
    let mut zeros: Vec<String> = Vec::new();
    for i in 0..ls.len() {
        if ls[i].chars().nth(pos).unwrap() == '1' {
            ones.push(ls[i].clone());
        } else {
            zeros.push(ls[i].clone());
        }
    }

    let mut winners;
    if criteria == "most_common" {
        winners = if ones.len() >= zeros.len() {
            ones
        } else {
            zeros
        };
    } else {
        winners = if ones.len() < zeros.len() {
            ones
        } else {
            zeros
        };
    }
    return reduce(&mut winners, pos + 1, criteria);
}

fn main() {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

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
        "PART 1. Gamma rate: {}, Epsilon rate: {}, Power consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let oxygen_gen_bin = reduce(&mut lines, 0, "most_common".to_string());
    let co2_scrubber_bin = reduce(&mut lines, 0, "least_common".to_string());

    let oxygen_gen = isize::from_str_radix(&oxygen_gen_bin, 2).unwrap();
    let co2_scrubber = isize::from_str_radix(&co2_scrubber_bin, 2).unwrap();

    println!(
        "PART 2. Oxygen generator: {}, CO2 scrubber: {}, Life support: {}",
        oxygen_gen, co2_scrubber, oxygen_gen * co2_scrubber
    );
}
