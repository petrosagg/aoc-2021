use std::cell::Cell;
use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut grid = vec![];
    for line in input {
        let octupuses = line.chars().map(|c| (Cell::new(c.to_digit(10).unwrap()), Cell::new(false))).collect_vec();
        grid.push(octupuses);
    }

    let max_x = grid[0].len();
    let max_y = grid.len();
    let mut flashes = 0;
    for step in 1.. {
        let mut round_flashes = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let mut stack = vec![(x, y)];
                while let Some((x, y)) = stack.pop() {
                    let (energy, flashed) = &grid[y][x];
                    if !flashed.get() {
                        energy.set(energy.get() + 1);
                    }
                    if energy.get() > 9 {
                        flashes += 1;
                        round_flashes += 1;
                        flashed.set(true);
                        energy.set(0);

                        if x > 0 {
                            stack.push((x - 1, y));
                            if y > 0 {
                                stack.push((x - 1, y - 1));
                            }
                            if y < max_y - 1 {
                                stack.push((x - 1, y + 1));
                            }
                        }
                        if x < max_x - 1 {
                            stack.push((x + 1, y));
                            if y > 0 {
                                stack.push((x + 1, y - 1));
                            }
                            if y < max_y - 1 {
                                stack.push((x + 1, y + 1));
                            }
                        }
                        if y > 0 {
                            stack.push((x, y - 1));
                        }
                        if y < max_y - 1 {
                            stack.push((x, y + 1));
                        }
                    }
                }
            }
        }

        if round_flashes == 100 {
            println!("Part 2: {}", step);
            break;
        }
        if step == 100 {
            dbg!(flashes);
        }

        for row in grid.iter() {
            for (_, flashed) in row {
                flashed.set(false);
            }
        }
    }
}
