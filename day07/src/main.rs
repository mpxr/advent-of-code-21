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

    let mut best_pos_2 = isize::MAX;
    let mut best_cost_2 = isize::MAX;

    for n in *min_pos..=*max_pos {
        let mut cost = (positions[0] - n).abs();
        let mut cost_2 = sum_progression(cost);

        for pos in &positions[1..] {
            cost += (pos - n).abs();
            cost_2 += sum_progression((pos - n).abs());
        }

        if cost < best_cost {
            best_cost = cost;
            best_pos = n;
        }
        if cost_2 < best_cost_2 {
            best_cost_2 = cost_2;
            best_pos_2 = n;
        }
    }

    println!("PART 1. Best pos: {}, its cost: {}", best_pos, best_cost);
    println!(
        "PART 2. Best pos: {}, its cost: {}",
        best_pos_2, best_cost_2
    );
}

// part 2 step cost calculation
fn sum_progression(n: isize) -> isize {
    n * (n + 1) / 2
}
