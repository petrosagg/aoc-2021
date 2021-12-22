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
        let coords = [coords[0], coords[1], coords[2]];
        program.push((is_on, coords));
    }

    // Holds all the on areas. Initially everything is off
    let mut state: Vec<[(isize, isize); 3]> = vec![];
    let mut temp_state = vec![];
    for (is_on, [new_x, new_y, new_z]) in program {
        for [old_x, old_y, old_z] in state.drain(..) {
            // Slice the area we have in our state based on the sides of the new area
            let top = [
                old_x,
                old_y,
                (min(max(new_z.1, old_z.0), old_z.1), old_z.1),
            ];
            let bottom = [
                old_x,
                old_y,
                (old_z.0, max(min(new_z.0, old_z.1), old_z.0)),
            ];
            let front = [
                old_x,
                (min(max(new_y.1, old_y.0), old_y.1), old_y.1),
                (bottom[2].1, top[2].0)
            ];
            let back = [
                old_x,
                (old_y.0, max(min(new_y.0, old_y.1), old_y.0)),
                (bottom[2].1, top[2].0)
            ];
            let left = [
                (min(max(new_x.1, old_x.0), old_x.1), old_x.1),
                (back[1].1, front[1].0),
                (bottom[2].1, top[2].0)
            ];
            let right = [
                (old_x.0, max(min(new_x.0, old_x.1), old_x.0)),
                (back[1].1, front[1].0),
                (bottom[2].1, top[2].0)
            ];
            for face in [top, bottom, front, back, left, right] {
                if !is_zero(&face) {
                    temp_state.push(face);
                }
            }
        };
        if is_on {
            temp_state.push([new_x, new_y, new_z]);
        }
        std::mem::swap(&mut state, &mut temp_state);
    }

    let mut total = 0;
    for area in state {
        total += area.iter().fold(1, |acc, (lo, hi)| acc * (hi - lo));
    }
    dbg!(total);
}
