use itertools::Itertools;
use std::str::FromStr;
use std::cell::Cell;

pub fn part1() {
    let input = include_str!("input");

    let mut coords = vec![];
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let coord = line.split(',').map(|n| usize::from_str(n).unwrap()).collect_vec();
        max_x = std::cmp::max(max_x, coord[0]);
        max_y = std::cmp::max(max_y, coord[1]);
        coords.push((coord[0], coord[1]));
    }

    let mut grid = vec![vec![Cell::new(false); max_x + 1]; max_y + 1];

    for (x, y) in coords {
        grid[y][x].set(true);
    }

    let folds = vec![
        (false, 655),
        (true, 447),
        (false, 327),
        (true, 223),
        (false, 163),
        (true, 111),
        (false, 81),
        (true, 55),
        (false, 40),
        (true, 27),
        (true, 13),
        (true, 6),
    ];

    for (fold, (is_horizontal, fold_line)) in folds.into_iter().enumerate() {
        for (y, row) in grid.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if is_horizontal && y > fold_line && point.get() {
                    grid[fold_line - (y - fold_line)][x].set(true);
                }
                if !is_horizontal && x > fold_line && point.get() {
                    grid[y][fold_line - (x - fold_line)].set(true);
                }
            }
        }
        if is_horizontal {
            grid.truncate(fold_line);
        } else {
            for row in grid.iter_mut() {
                row.truncate(fold_line);
            }
        }
        if fold == 0 {
            let answer = grid.iter().flatten().filter(|p| p.get()).count();
            dbg!(answer);
        }
    }

    for row in grid {
        for point in row {
            if point.get() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}
