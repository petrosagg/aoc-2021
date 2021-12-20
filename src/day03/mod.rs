pub fn part1() {
    const SIZE: usize = 12;
    let input = include_str!("input");
    let mut gamma_rate_ones = vec![0; SIZE];
    let mut gamma_rate_zeros = vec![0; SIZE];
    let mut epsilon_rate_ones = vec![0; SIZE];
    let mut epsilon_rate_zeros = vec![0; SIZE];

    for l in input.lines().filter(|l| !l.is_empty()) {
        for (position, c) in l.chars().enumerate() {
            match c {
                '0' => {
                    gamma_rate_zeros[position] += 1;
                    epsilon_rate_ones[position] += 1;
                }
                '1' => {
                    gamma_rate_ones[position] += 1;
                    epsilon_rate_zeros[position] += 1;
                }
                _ => panic!(),
            }
        }
    }

    let mut gamma_rate = 0;
    let mut gamma_bit = SIZE;
    for (ones, zeros) in gamma_rate_ones.into_iter().zip(gamma_rate_zeros.into_iter()) {
        if ones > zeros {
            gamma_rate += 1 << (gamma_bit - 1);
        }
        gamma_bit -= 1;
    }

    let mut epsilon_rate = 0;
    let mut epsilon_bit = SIZE;
    for (ones, zeros) in epsilon_rate_ones.into_iter().zip(epsilon_rate_zeros.into_iter()) {
        if ones > zeros {
            epsilon_rate += 1 << (epsilon_bit - 1);
        }
        epsilon_bit -= 1;
    }


    println!("result: {}", gamma_rate * epsilon_rate)
}

pub fn part2() {
    const SIZE: usize = 12;
    let input = include_str!("input");
    let mut oxygen_rating = 0;
    let mut scrubber_rating = 0;

    let report = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars().map(|c| match c {
                '0' => 0,
                '1' => 1,
                _ => panic!(),
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let mut oxygen_report = report.clone();
    let mut cur_bit = 0;
    while oxygen_report.len() > 1 {
        let ones = oxygen_report.iter().filter(|bits| bits[cur_bit] == 1).count();
        let zeros = oxygen_report.iter().filter(|bits| bits[cur_bit] == 0).count();
        let retained_bit = if ones >= zeros {
            1
        } else {
            0
        };
        oxygen_report.retain(|bits| bits[cur_bit] == retained_bit);
        cur_bit += 1;
    }

    let oxygen_rating_bits = oxygen_report.pop().unwrap();
    let mut oxygen_rating = 0;
    let mut cur_bit = SIZE;
    for bit in oxygen_rating_bits {
        if bit == 1 {
            oxygen_rating += 1 << (cur_bit - 1);
        }
        cur_bit -= 1;
    }

    let mut scrubber_report = report;
    let mut cur_bit = 0;
    while scrubber_report.len() > 1 {
        dbg!(&scrubber_report);
        let ones = scrubber_report.iter().filter(|bits| bits[cur_bit] == 1).count();
        let zeros = scrubber_report.iter().filter(|bits| bits[cur_bit] == 0).count();
        let retained_bit = if zeros <= ones {
            0
        } else {
            1
        };
        scrubber_report.retain(|bits| bits[cur_bit] == retained_bit);
        cur_bit += 1;
    }

    let scrubber_rating_bits = scrubber_report.pop().unwrap();
    let mut scrubber_rating = 0;
    let mut cur_bit = SIZE;
    for bit in scrubber_rating_bits {
        if bit == 1 {
            scrubber_rating += 1 << (cur_bit - 1);
        }
        cur_bit -= 1;
    }

    println!("result: {}", scrubber_rating * oxygen_rating);
}
