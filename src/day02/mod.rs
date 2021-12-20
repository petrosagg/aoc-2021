pub fn part1() {
    let input = include_str!("example");
    let mut depth = 0;
    let mut position = 0;
    for l in input.lines().filter(|l| !l.is_empty()) {
        let mut parts = l.split(' ');
        let direction = parts.next().unwrap();
        let magnitude: i64 = parts.next().unwrap().parse().unwrap();

        match direction {
            "up" => depth -= magnitude,
            "down" => depth += magnitude,
            "forward" => position += magnitude,
            s => panic!("unknown direction {}", s),
        }
    }

    println!("result: {}", depth * position);
}

pub fn part2() {
    let input = include_str!("input");
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for l in input.lines().filter(|l| !l.is_empty()) {
        let mut parts = l.split(' ');
        let direction = parts.next().unwrap();
        let magnitude: i64 = parts.next().unwrap().parse().unwrap();

        match direction {
            "up" => aim -= magnitude,
            "down" => aim += magnitude,
            "forward" => {
                position += magnitude;
                depth += magnitude * aim;
            }
            s => panic!("unknown direction {}", s),
        }
    }

    println!("result: {}", depth * position);
}
