use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Win {
    sum: i32,
    number_called: i32,
}

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

    let mut first_win: Win = Win {
        sum: 0 - 1,
        number_called: -1,
    };
    let mut last_win: Win = Win {
        sum: -1,
        number_called: -1,
    };

    let input = lines[0].split(',');

    let mut won_boards: Vec<i32> = Vec::new();

    for value in input {
        let i_value = value.parse::<i32>().unwrap();

        for (board_index, board) in boards.iter_mut().enumerate() {
            mark_board(board, i_value);
            let board_value: i32 = check_board(board);
            if board_value > 0 {
                if first_win.sum == -1 {
                    first_win = Win {
                        sum: board_value,
                        number_called: i_value,
                    };
                }

                if !won_boards.contains(&(board_index as i32)) {
                    last_win = Win {
                        sum: board_value,
                        number_called: i_value,
                    };
                    won_boards.push(board_index as i32);
                }
            }
        }
    }

    println!(
        "PART 1. Win: {:?}, Result: {}",
        first_win,
        first_win.sum * first_win.number_called
    );
    println!(
        "PART 2. Win: {:?}, Result: {}",
        last_win,
        last_win.sum * last_win.number_called
    );
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
