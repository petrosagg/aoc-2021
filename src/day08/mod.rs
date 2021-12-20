use std::collections::HashSet;

use itertools::Itertools;

fn char_to_segment(c: char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!(),
    }
}

pub fn part1() {
    let input = include_str!("input");

    let mut problem = vec![];
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut parts = line.split(" | ");
        let input = parts.next().unwrap();
        let output = parts.next().unwrap();

        let input = input.split(' ').map(|s| {
            s.chars().map(char_to_segment).collect_vec()
        }).collect_vec();
        let output = output.split(' ').map(|s| {
            s.chars().map(char_to_segment).collect_vec()
        }).collect_vec();

        problem.push((input, output));
    }

    let mut easy_digits = 0;
    for (_, output) in &problem {
        for entry in output {
            if matches!(entry.len(), 2 | 3 | 4 | 7) {
                easy_digits += 1;
            }
        }
    }
    dbg!(easy_digits);

    let digit_to_segments: [&[usize]; 10] = [
        /* 0 */ &[0, 1, 2, 4, 5, 6],
        /* 1 */ &[2, 5],
        /* 2 */ &[0, 2, 3, 4, 6],
        /* 3 */ &[0, 2, 3, 5, 6],
        /* 4 */ &[1, 2, 3, 5],
        /* 5 */ &[0, 1, 3, 5, 6],
        /* 6 */ &[0, 1, 3, 4, 5, 6],
        /* 7 */ &[0, 2, 5],
        /* 8 */ &[0, 1, 2, 3, 4, 5, 6],
        /* 9 */ &[0, 1, 2, 3, 5, 6],
    ];

    // For a given number of signals, what are the possible segments?
    let mut signals_to_segments = vec![];
    for n in 0..8 {
        let mut union = HashSet::new();
        let mut intersection = HashSet::new();
        intersection.extend(0usize..7);
        for &segments in digit_to_segments.iter() {
            if segments.len() == n {
                union.extend(segments.iter().copied());
                intersection.retain(|n| segments.contains(n));
            }
        }
        signals_to_segments.push((union, intersection));
    }

    let mut total_sum = 0;
    for (input, output) in problem.into_iter() {
        // For each signal, records the currently possible segments
        let mut scratch: Vec<HashSet<usize>> = vec![HashSet::from_iter(0..7); 7];

        for observation in input.iter() {
            // println!("Processing observation: {:?}", observation);
            let (union, intersection) = &signals_to_segments[observation.len()];

            for &signal in observation {
                scratch[signal].retain(|segment| union.contains(segment));
            }
            for signal in (0usize..7).filter(|n| !observation.contains(n)) {
                scratch[signal].retain(|segment| !intersection.contains(segment));
            }
        }

        for _ in 0..7 {
            let mut scratch2 = scratch.clone();
            for (i, segment) in scratch.into_iter().enumerate().filter(|(_, s)| s.len() == 1) {
                for (j, segments) in scratch2.iter_mut().enumerate() {
                    if i != j {
                        segments.retain(|n| n != segment.iter().next().unwrap());
                    }
                }
            }
            scratch = scratch2;
        }

        let signal_to_segment = scratch.into_iter().flatten().collect_vec();
        assert_eq!(signal_to_segment.len(), 7);
        let mut number = 0;
        for (position, digit) in output.into_iter().rev().enumerate() {
            let mut segments = vec![];
            for signal in digit {
                segments.push(signal_to_segment[signal]);
            }
            segments.sort_unstable();

            let d = digit_to_segments.iter().position(|&x| x == &*segments).unwrap();
            number += d * 10usize.pow(position as u32);
        }
        total_sum += number;
    }
    dbg!(total_sum);
}
