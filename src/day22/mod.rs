use std::cmp::{min,max};
use itertools::Itertools;

fn is_zero(area: &[(isize, isize)]) -> bool {
    area.iter().any(|(lo, hi)| lo == hi)
}

pub fn part1() {
    let lines = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut program = vec![];
    for line in lines {
        let mut parts = line.split(' ').collect_vec();
        let is_on = parts[0] == "on";
        let parts = parts[1].split(',').collect_vec();
        let mut coords = vec![];
        for part in parts {
            let part = &part[2..];
            let mut parts = part.split("..");
            let low: isize = parts.next().unwrap().parse().unwrap();
            let high: isize = parts.next().unwrap().parse::<isize>().unwrap() + 1;
            coords.push((low, high));
        }
        program.push((is_on, coords));
    }

    // Holds all the on areas. Initially everything is off
    let mut state: Vec<Vec<(isize, isize)>> = vec![];
    let mut temp_state = vec![];
    for (is_on, new_shape) in program {
        for cur_shape in state.drain(..) {
            // Carve out the area we have in our state based on the new area
            let mut tmp = cur_shape.clone();
            // There are two sides per dimention
            for i in 0..new_shape.len() {
                let mut side_a = tmp.clone();
                side_a[i] = (min(max(new_shape[i].1, cur_shape[i].0), cur_shape[i].1), cur_shape[i].1);

                let mut side_b = tmp.clone();
                side_b[i] = (cur_shape[i].0, max(min(new_shape[i].0, cur_shape[i].1), cur_shape[i].0));

                tmp[i] = (side_b[i].1, side_a[i].0);
                if !is_zero(&side_a) {
                    temp_state.push(side_a);
                }
                if !is_zero(&side_b) {
                    temp_state.push(side_b);
                }
            }
        };
        if is_on {
            temp_state.push(new_shape);
        }
        std::mem::swap(&mut state, &mut temp_state);
    }

    let mut total = 0;
    for area in state {
        total += area.iter().fold(1, |acc, (lo, hi)| acc * (hi - lo));
    }
    dbg!(total);
}
