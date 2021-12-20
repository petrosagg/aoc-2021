use itertools::Itertools;

// Adds a 2 pixel border around the grid with the given infinity pixel
fn enlarge(grid: Vec<Vec<bool>>, infinity: bool) -> Vec<Vec<bool>> {
    let new_max_x = grid[0].len() + 4;
    let mut new_grid = vec![vec![infinity; new_max_x]; 2];
    for row in grid {
        let mut new_row = vec![infinity; 2];
        new_row.extend(row);
        new_row.extend([infinity, infinity]);
        new_grid.push(new_row);
    }
    new_grid.extend(vec![vec![infinity; new_max_x]; 2]);
    new_grid
}

pub fn part1() {
    let mut lines = include_str!("example").lines().filter(|l| !l.is_empty());

    let enhancement_algo = lines.next().unwrap().chars().map(|c| c == '#').collect_vec();
    let mut grid = vec![];
    for line in lines {
        grid.push(line.chars().map(|c| c == '#').collect_vec());
    }

    let mut infinity = false;
    for passes in 1..=50 {
        grid = enlarge(grid, infinity);
        let mut new_grid = vec![];
        for (a, b, c) in grid.iter().tuple_windows() {
            let mut new_row = vec![];
            let matrix_iter = a.iter().tuple_windows::<(_, _, _)>()
                .zip(b.iter().tuple_windows::<(_, _, _)>())
                .zip(c.iter().tuple_windows::<(_, _ ,_)>());

            for ((x,y),z) in matrix_iter {
                let bits = [x.0, x.1, x.2, y.0, y.1, y.2, z.0, z.1, z.2];
                let mut index = 0;
                for (position, bit) in bits.into_iter().rev().enumerate() {
                    index += (*bit as usize) << position;
                }
                new_row.push(enhancement_algo[index]);
            }
            new_grid.push(new_row);
        }
        grid = new_grid;
        if infinity {
            infinity = enhancement_algo[511];
        } else {
            infinity = enhancement_algo[0];
        }

        if passes == 2 || passes == 50 {
            println!("After {} passes: {}", passes, grid.iter().flatten().filter(|x| **x).count());
        }
    }
}
