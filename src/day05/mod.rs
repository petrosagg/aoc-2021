use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1() {
    let input = include_str!("input");

    let mut lines = vec![];
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut parts = line.split(" -> ").map(|part| {
            let mut coords = part.split(',').map(|n| usize::from_str(n).unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            (x, y)
        });

        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        lines.push((from, to));
    }

    let max_x = lines.iter().flat_map(|(from, to)| [from.0, to.0]).max().unwrap();
    let max_y = lines.iter().flat_map(|(from, to)| [from.1, to.1]).max().unwrap();
    let mut grid = vec![0; (max_x + 1) * (max_y + 1)];
    for (from, to) in lines {
        if from.0 == to.0 {
            let range = if from.1 < to.1 {
                from.1..=to.1
            } else {
                to.1..=from.1
            };
            let x = from.0;
            for y in range {
                grid[y * max_y + x] += 1;
            }
        } else if from.1 == to.1 {
            let range = if from.0 < to.0 {
                from.0..=to.0
            } else {
                to.0..=from.0
            };
            let y = from.1;
            for x in range {
                grid[y * max_y + x] += 1;
            }
        } else {
            let range_x: Box<dyn Iterator<Item=usize>> = if from.0 < to.0 {
                Box::new(from.0..=to.0)
            } else {
                Box::new((to.0..=from.0).rev())
            };
            let range_y: Box<dyn Iterator<Item=usize>> = if from.1 < to.1 {
                Box::new(from.1..=to.1)
            } else {
                Box::new((to.1..=from.1).rev())
            };

            for (x, y) in range_x.zip(range_y) {
                grid[y * max_y + x] += 1;
            }
        }
    }

    for row in grid.chunks(max_x) {
        for x in row {
            if *x == 0 {
                print!(".");
            } else {
                print!("{}", x);
            }
        }
        println!();
    }

    let overlap = grid.iter().filter(|n| **n >= 2).count();
    dbg!(overlap);
}
