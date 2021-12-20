fn check_syntax(input: &mut impl Iterator<Item=char>, expected: Option<char>) -> Result<(), u64> {
    while let Some(next) = input.next() {
        match next {
            '(' => check_syntax(input, Some(')'))?,
            '[' => check_syntax(input, Some(']'))?,
            '{' => check_syntax(input, Some('}'))?,
            '<' => check_syntax(input, Some('>'))?,
            c if Some(c) == expected => return Ok(()),
            ')' => return Err(3),
            ']' => return Err(57),
            '}' => return Err(1197),
            '>' => return Err(25137),
            _ => panic!(),
        }
    }
    Ok(())
}

fn autocomplete(input: &mut impl Iterator<Item=char>, expected: Option<char>, out: &mut Vec<char>) {
    while let Some(next) = input.next() {
        match next {
            '(' => autocomplete(input, Some(')'), out),
            '[' => autocomplete(input, Some(']'), out),
            '{' => autocomplete(input, Some('}'), out),
            '<' => autocomplete(input, Some('>'), out),
            c if Some(c) == expected => return,
            _ => panic!(),
        }
    }
    // We're expecting a character but there is none left
    if let Some(c) = expected {
        out.push(c);
    }
}

pub fn part1() {
    let input = include_str!("input").lines().filter(|l| !l.is_empty());

    let mut incomplete_lines = vec![];
    let mut total_score = 0;
    for line in input {
        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            match check_syntax(&mut chars, None) {
                Ok(()) => incomplete_lines.push(line.to_owned()),
                Err(score) => {
                    total_score += score;
                    break;
                }
            }
        }
    }
    dbg!(total_score);

    let mut autocomplete_scores = vec![];
    for line in incomplete_lines {
        let mut rest = vec![];
        autocomplete(&mut line.chars(), None, &mut rest);
        let score = rest.into_iter().fold(0u64, |acc, c| {
            let points = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            };
            acc * 5 + points
        });
        autocomplete_scores.push(score);
    }
    autocomplete_scores.sort_unstable();
    dbg!(autocomplete_scores[autocomplete_scores.len() / 2]);
}
