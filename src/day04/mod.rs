use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1() {
    let input = include_str!("input");

    let mut lines = input.lines();
    let random_numbers = lines.next().unwrap().split(',').map(|n| u64::from_str(n).unwrap()).collect_vec();
    lines.next().unwrap();

    let mut boards = vec![];
    let mut board = vec![];
    for line in lines {
        if line.is_empty() {
            boards.push(board);
            board = vec![];
        }
        board.extend(line.split_whitespace().map(|n| (u64::from_str(n).unwrap(), false)));
    }
    boards.push(board);

    let mut seen = HashSet::new();
    let mut winning_boards = vec![];
    for number in random_numbers {
        println!("{}", number);
        for (board_id, board) in boards.iter_mut().enumerate() {
            for (board_number, marked) in board.iter_mut() {
                if *board_number == number {
                    *marked = true;
                }
            }

            if board.chunks(5).any(|row| row.iter().all(|(_, marked)| *marked)) {
                if seen.insert(board_id) {
                    let unmarked_sum: u64 = board.iter().filter(|(_, marked)| !*marked).map(|(n, _)| n).sum();
                    let score = unmarked_sum * number;
                    winning_boards.push((board_id, score));
                }
            }
            for column in 0..5 {
                if board.iter().skip(column).step_by(5).all(|(_, marked)| *marked) {
                    if seen.insert(board_id) {
                        let unmarked_sum: u64 = board.iter().filter(|(_, marked)| !*marked).map(|(n, _)| n).sum();
                        let score = unmarked_sum * number;
                        winning_boards.push((board_id, score));
                    }
                }
            }
        }
    }

    dbg!(winning_boards.first().unwrap());
    dbg!(winning_boards.last().unwrap());
}

