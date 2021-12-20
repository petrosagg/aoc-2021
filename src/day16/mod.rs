use bitvec::prelude::*;
use itertools::Itertools;

fn bits_to_number(bits: &[bool]) -> usize {
    let mut number = 0;
    for (position, &bit) in bits.iter().rev().enumerate() {
        number += (bit as usize) << position;
    };
    number
}

// Reads a packet and returns the number of bytes read
fn parse_packet(input: &[bool]) -> (&[bool], usize, usize) {
    let (version, input) = input.split_at(3);
    let mut total_version = bits_to_number(version);

    let (type_id, mut input) = input.split_at(3);

    match bits_to_number(type_id) {
        // Literal
        4 => {
            let mut number_bits: Vec<bool> = vec![];
            loop {
                let (group, rest) = input.split_at(5);
                input = rest;
                number_bits.extend(&group[1..]);
                if !group[0] {
                    break;
                }
            }
            let number = bits_to_number(&number_bits);
            (input, total_version, number)
        }
        // Operator
        type_id => {
            let mut arguments = vec![];
            let (length_type, rest) = input.split_at(1);
            input = rest;

            match bits_to_number(length_type) {
                0 => {
                    let (length, rest) = input.split_at(15);
                    input = rest;
                    let expected_consumed = input.len() - bits_to_number(length);
                    while input.len() > expected_consumed {
                        let result = parse_packet(input);
                        input = result.0;
                        total_version += result.1;
                        arguments.push(result.2);
                    }
                },
                1 => {
                    let (subpackets, rest) = input.split_at(11);
                    input = rest;
                    for _ in 0..bits_to_number(subpackets) {
                        let result = parse_packet(input);
                        input = result.0;
                        total_version += result.1;
                        arguments.push(result.2);
                    }
                },
                _ => panic!(),
            }
            let result = match type_id {
                0 => arguments.into_iter().sum(),
                1 => arguments.into_iter().product(),
                2 => arguments.into_iter().min().unwrap(),
                3 => arguments.into_iter().max().unwrap(),
                5 => (arguments[0] > arguments[1]) as usize,
                6 => (arguments[0] < arguments[1]) as usize,
                7 => (arguments[0] == arguments[1]) as usize,
                _ => unreachable!(),
            };
            (input, total_version, result)
        }
    }
}

pub fn part1() {
    let input = include_str!("input").trim();

    let input = hex::decode(input).unwrap();
    let data_bits = input.view_bits::<Msb0>();

    let bits = data_bits.iter().map(|x| *x).collect_vec();

    let (_, total_version, result) = parse_packet(&bits);
    dbg!(total_version);
    dbg!(result);
}
