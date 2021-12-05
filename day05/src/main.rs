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

        let x_diff = (x1 as i32 - x2 as i32).abs() as usize;
        let y_diff = (y1 as i32 - y2 as i32).abs() as usize;

        if x1 == x2 {
            // consider horizontal lines
            for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                let key = format!("x={},y={}", x1, i);
                let counter = vents.entry(key).or_insert(0);
                *counter += 1;
            }
        } else if y1 == y2 {
            // consider vertical lines
            for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                let key = format!("x={},y={}", i, y1);
                let counter = vents.entry(key).or_insert(0);
                *counter += 1;
            }
        } else if x_diff == y_diff {
            // consider diagonal lines
            // comment out this if-else to get the result of part 1
            for i in 0..=x_diff {
                let p1: usize;
                let q1: usize;
                if x1 <= x2 {
                    p1 = x1 + i;
                    q1 = if y1 > y2 { y1 - i } else { y1 + i };
                } else {
                    p1 = x2 + i;
                    q1 = if y2 < y1 { y2 + i } else { y2 - i };
                }

                let key = format!("x={},y={}", p1, q1);
                let counter = vents.entry(key).or_insert(0);
                *counter += 1;
            }
        }
    }

    let overlapping_lines = vents.values().filter(|v| **v >= 2).count();

    println!("PART (1) 2. Overlapping lines = {}", overlapping_lines);
}
