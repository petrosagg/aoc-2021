pub fn part1() {
    let lines = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut grid = vec![];
    for line in lines {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let max_y = grid.len();
    let max_x = grid[0].len();

    let mut changes = vec![];
    for step in 1.. {
        for (y, row) in grid.iter().enumerate() {
            for (x, &position) in row.iter().enumerate() {
                if position == '>' && grid[y][(x + 1) % max_x] == '.' {
                    changes.push((x, y));
                }
            }
        }
        let moved_right = !changes.is_empty();
        for (x, y) in changes.drain(..) {
            grid[y][(x + 1) % max_x] = '>';
            grid[y][x] = '.';
        }

        for (y, row) in grid.iter().enumerate() {
            for (x, &position) in row.iter().enumerate() {
                if position == 'v' && grid[(y + 1) % max_y][x] == '.' {
                    changes.push((x, y));
                }
            }
        }

        if !moved_right && changes.is_empty() {
            dbg!(step);
            break;
        }
        for (x, y) in changes.drain(..) {
            grid[(y + 1) % max_y][x] = 'v';
            grid[y][x] = '.';
        }
    }
}
