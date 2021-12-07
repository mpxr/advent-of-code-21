use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("no such file");
    let positions: Vec<isize> = contents
        .trim()
        .split(',')
        .map(|v| v.parse::<isize>().expect("eeeh"))
        .collect();

    let min_pos = positions.iter().min().unwrap();
    let max_pos = positions.iter().max().unwrap();

    let mut best_pos = isize::MAX;
    let mut best_cost = isize::MAX;
    for n in *min_pos..=*max_pos {
        let mut cost = (positions[0] - n).abs();
        for pos in &positions[1..] {
            cost += (pos - n).abs();
        }
        if cost < best_cost {
            best_cost = cost;
            best_pos = n;
        }
    }

    println!("PART 1. Best pos: {}, its cost: {}", best_pos, best_cost);
}
