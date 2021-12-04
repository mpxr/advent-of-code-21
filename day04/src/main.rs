use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let mut boards: Vec<[[i32; 5]; 5]> = Vec::new();
    let mut board = [[0i32; 5]; 5];
    let mut i = 0;
    for line in &lines[2..] {
        if line.chars().count() == 0 {
            boards.push(board);
            board = [[0i32; 5]; 5];
            i = 0;
            continue;
        }

        let columns = line.split_whitespace();
        for (j, col) in columns.enumerate() {
            board[i][j] = col.parse::<i32>().unwrap();
        }
        i += 1;
    }

    let input = lines[0].split(',');
    'outer: for value in input {
        let i_value = value.parse::<i32>().unwrap();
        for board in boards.iter_mut() {
            mark_board(board, i_value);
            let board_value: i32 = check_board(board);
            if board_value > 0 {
                println!(
                    "Sum: {}, Number called: {}, Result: {}",
                    board_value,
                    i_value,
                    board_value * i_value
                );
                break 'outer;
            }
        }
    }
}

fn mark_board(board: &mut [[i32; 5]; 5], num: i32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] == num {
                board[i][j] = -1;
            }
        }
    }
}

fn check_board(board: &mut [[i32; 5]; 5]) -> i32 {
    let mut hor_complete = false;
    let mut vert_complete = false;


    for i in 0..5 {
        let row_sum: i32 = board[i].iter().sum();
        if row_sum == -5 {
            hor_complete = true;
            break;
        }
    }
    for i in 0..5 {
        let mut col_sum: i32 = 0;
        for j in 0..5 {
            col_sum += board[j][i];
        }
        if col_sum == -5 {
            vert_complete = true;
            break;
        }
    }

    let mut sum = 0;
    if hor_complete || vert_complete {
        for i in 0..5 {
            for j in 0..5 {
                if board[i][j] > -1 {
                    sum += board[i][j];
                }
            }
        }
    }

    return sum;
}
