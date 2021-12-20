use std::str::FromStr;

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input");
    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| u64::from_str(l).unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("result: {}", result);
}

pub fn part2() {
    let input = include_str!("input");
    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| u64::from_str(l).unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("result: {}", result);
}
