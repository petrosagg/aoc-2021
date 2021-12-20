use itertools::Itertools;
use std::collections::HashMap;

pub fn part1() {
    let mut input = include_str!("input").lines().filter(|l| !l.is_empty());

    let template = input.next().unwrap();

    let mut rules = vec![];
    for line in input {
        let mut parts = line.split(" -> ");
        let (left, right) = parts.next().map(|rule| {
            let mut chars = rule.chars();
            (chars.next().unwrap(), chars.next().unwrap())
        }).unwrap();

        let result = parts.next().unwrap().chars().next().unwrap();

        rules.push(((left, right), result));
    }

    let mut letter_counts = HashMap::new();
    let mut pair_counts: HashMap<(char, char), i64> = HashMap::new();

    for letter in template.chars() {
        *letter_counts.entry(letter).or_insert(0) += 1;
    }
    for pair in template.chars().tuple_windows() {
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    let mut letter_diffs = HashMap::new();
    let mut pair_diffs = HashMap::new();
    for step in 1..=40 {
        for ((left, right), insertion) in rules.iter().copied() {
            if let Some(count) = pair_counts.get(&(left, right)) {
                *letter_diffs.entry(insertion).or_insert(0) += count;
                *pair_diffs.entry((left, right)).or_insert(0) -= count;
                *pair_diffs.entry((left, insertion)).or_insert(0) += count;
                *pair_diffs.entry((insertion, right)).or_insert(0) += count;
            }
        }
        for (pair, diff) in pair_diffs.drain() {
            *pair_counts.entry(pair).or_insert(0) += diff;
        }
        for (letter, diff) in letter_diffs.drain() {
            *letter_counts.entry(letter).or_insert(0) += diff;
        }

        if step == 10 || step == 40 {
            let mut counts = letter_counts.values().collect_vec();
            counts.sort_unstable();
            println!("step {}: {}", step, counts.pop().unwrap() - counts[0]);
        }
    }
}
