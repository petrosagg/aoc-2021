use itertools::Itertools;
use std::cell::Cell;

pub fn part1() {
    let input = include_str!("input");

    let mut map = vec![];
    for line in input.lines().filter(|l| !l.is_empty()) {
        let line = line.chars().map(|c| (c.to_digit(10).unwrap(), Cell::new(false))).collect_vec();
        map.push(line);
    }

    let mut basins = vec![];
    let max_x = map[0].len();
    let max_y = map.len();
    let mut risk_level = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, (height, _)) in row.iter().enumerate() {
            // Find low point
            let mut low_point = true;

            if x > 0 && map[y][x-1].0 <= *height {
                low_point = false;
            }
            if y > 0 && map[y-1][x].0 <= *height {
                low_point = false;
            }
            if x < max_x - 1 && map[y][x + 1].0 <= *height {
                low_point = false;
            }

            if y < max_y - 1 && map[y + 1][x].0 <= *height {
                low_point = false;
            }

            if low_point {
                risk_level += height + 1;

                // Explore basin
                let mut basin_size = 0;
                let mut stack = vec![(x, y)];
                while let Some((x, y)) = stack.pop() {
                    if map[y][x].1.replace(true) {
                        continue;
                    }
                    basin_size += 1;
                    if x > 0 && map[y][x-1].0 < 9 {
                        stack.push((x-1, y));
                    }
                    if y > 0 && map[y-1][x].0 < 9 {
                        stack.push((x, y-1));
                    }
                    if x < max_x - 1 && map[y][x + 1].0 < 9 {
                        stack.push((x + 1, y));
                    }
                    if y < max_y - 1 && map[y + 1][x].0 < 9 {
                        stack.push((x, y + 1));
                    }

                }
                basins.push(basin_size);
            }
        }
    }
    basins.sort_unstable();
    dbg!(risk_level);
    dbg!(basins.into_iter().rev().take(3).product::<u64>());
}
