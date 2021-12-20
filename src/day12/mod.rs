use std::cell::Cell;
use std::collections::{HashMap, HashSet};

fn search_paths(name: &str, double_taken: bool, caves: &HashMap<String, (bool, Cell<u8>)>, edges: &HashMap<String, HashSet<String>>) -> u64 {
    // Increment visit count
    let visit_count = &caves.get(name).unwrap().1;
    visit_count.set(visit_count.get() + 1);

    // Recurse into visiting the paths from here
    let mut paths = 0;
    for name in edges.get(name).into_iter().flatten() {
        let (is_big, visited) = caves.get(name).unwrap();
        if name == "end" || name == "start" {
            if name == "end" {
                paths += 1;
            }
        } else if !is_big {
            if !double_taken && visited.get() > 0 {
                paths += search_paths(name, true, caves, edges);
            } else if visited.get() == 0 {
                paths += search_paths(name, double_taken, caves, edges);
            }
        } else if *is_big {
            paths += search_paths(name, double_taken, caves, edges);
        }
    }
    // Decrement visit count
    visit_count.set(visit_count.get() - 1);
    paths
}

pub fn part1() {
    let input = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut caves = HashMap::new();
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input {
        let mut parts = line.split('-');

        let from  = parts.next().unwrap().to_owned();
        let to  = parts.next().unwrap().to_owned();

        let is_big = from == from.to_ascii_uppercase();
        caves.insert(from.clone(), (is_big, Cell::new(0)));

        let is_big = to == to.to_ascii_uppercase();
        caves.insert(to.clone(), (is_big, Cell::new(0)));

        edges.entry(from.clone()).or_default().insert(to.clone());
        edges.entry(to).or_default().insert(from);
    }

    let paths = search_paths("start", false, &caves, &edges);
    dbg!(paths);
}
