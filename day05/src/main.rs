use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    let mut vents = HashMap::new();

    for line in lines {
        let mut segment = line.splitn(2, " -> ");
        let mut coord_1 = segment.next().unwrap().splitn(2, ',');
        let mut coord_2 = segment.next().unwrap().splitn(2, ',');

        let x1: usize = coord_1.next().unwrap().parse().unwrap();
        let y1: usize = coord_1.next().unwrap().parse().unwrap();

        let x2: usize = coord_2.next().unwrap().parse().unwrap();
        let y2: usize = coord_2.next().unwrap().parse().unwrap();

        if x1 == x2 {
            for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                let key = format!("x={},y={}", x1, i);
                if vents.contains_key(&key) {
                    let curr_value = vents.get(&key).unwrap();
                    let new_value = curr_value + 1;
                    vents.insert(key, new_value);
                } else {
                    vents.insert(key, 1);
                }
            }
        }
        else if y1 == y2 {
            for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                let key = format!("x={},y={}", i, y1);
                if vents.contains_key(&key) {
                    let curr_value = vents.get(&key).unwrap();
                    let new_value = curr_value + 1;
                    vents.insert(key, new_value);
                } else {
                    vents.insert(key, 1);
                }
            }
        }
    }

    let mut overlapping_lines = 0;
    for (point, danger) in &vents {
        if danger > &1 {
            overlapping_lines +=1 ;
            // println!("Point = {}, Danger = {}", point, danger);
        }
    }

    println!("PART 1. Overlapping lines = {}", overlapping_lines);
}
