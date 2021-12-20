use std::cell::Cell;
use std::collections::{HashMap, BTreeSet};

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd)]
struct Node {
    // This will be sorted based on field order, so put the distance first
    distance: Cell<usize>,
    coords: (usize, usize),
    risk: usize,
}

fn neighbors((x, y): (usize, usize), (max_x, max_y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if x > 0 {
        result.push((x - 1, y));
    }
    if x < max_x - 1 {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y < max_y - 1 {
        result.push((x, y + 1));
    }
    result
}

pub fn part1() {
    let input = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut max_x = 0;
    let mut max_y = 0;
    let mut nodes = HashMap::new();
    for (y, line) in input.enumerate() {
        nodes.extend(
            line.chars().enumerate().map(|(x, risk)| {
                max_x = std::cmp::max(max_x, x);
                max_y = std::cmp::max(max_y, y);
                (
                    (x, y),
                    Node {
                        coords: (x, y),
                        risk: risk.to_digit(10).unwrap() as usize,
                        distance: Cell::new(usize::MAX),
                    }
                )
            })
        );
    }
    max_x += 1;
    max_y += 1;

    let mut big_map = HashMap::new();
    for (coords, node) in nodes {
        for i in 0..5 {
            for j in 0..5 {
                let new_coords = (coords.0 + max_x * i, coords.1 + max_y * j);
                let mut new_node = node.clone();
                new_node.coords = new_coords;
                new_node.risk = (node.risk - 1 + i + j) % 9 + 1;
                big_map.insert(new_coords, new_node);
            }
        }
    }
    let nodes = big_map;
    max_x *= 5;
    max_y *= 5;

    nodes.get(&(0, 0)).unwrap().distance.set(0);

    let mut unvisited_nodes = BTreeSet::from_iter(nodes.values());
    loop {
        let first = unvisited_nodes.iter().next().cloned().cloned().unwrap();
        let cur_node = unvisited_nodes.take(&first).unwrap();
        for coord in neighbors(cur_node.coords, (max_x, max_y)) {
            if let Some(neighbor) = nodes.get(&coord) {
                if let Some(neighbor) = unvisited_nodes.take(neighbor) {
                    let tentantive_distance = neighbor.risk + cur_node.distance.get();
                    if tentantive_distance < neighbor.distance.get() {
                        neighbor.distance.set(tentantive_distance);
                    }
                    unvisited_nodes.insert(neighbor);
                }
            }
        }
        if cur_node.coords == ((max_x / 5) - 1, (max_y / 5) - 1) {
            dbg!(cur_node.distance.get());
        }
        if cur_node.coords == (max_x - 1, max_y - 1) {
            dbg!(cur_node.distance.get());
            break;
        }
    }
}
