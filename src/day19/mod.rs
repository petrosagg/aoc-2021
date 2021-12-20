use std::collections::HashSet;
use std::str::FromStr;

/// Rotates a coordinate using the selected orientation
#[inline]
fn rotate_coord((x, y, z): (isize, isize, isize), orientation: usize) -> (isize, isize, isize) {
    [
        // Assume we're looking down x+
        ( x,  y,  z),
        ( x,  z, -y),
        ( x, -y, -z),
        ( x, -z,  y),
        // Rotate 90 degrees around z, now we're looking down y+
        ( y,  z,  x),
        ( y,  x, -z),
        ( y, -z, -x),
        ( y, -x,  z),
        // Rotate 90 degrees around z, now we're looking down -x
        (-x,  z,  y),
        (-x,  y, -z),
        (-x, -z, -y),
        (-x, -y,  z),
        // Rotate 90 degrees around z, now we're looking down -y
        (-y,  x,  z),
        (-y,  z, -x),
        (-y, -x, -z),
        (-y, -z,  x),
        // Rotate 90 degrees around y, now we're looking down z+
        ( z,  y, -x),
        ( z, -x, -y),
        ( z, -y,  x),
        ( z,  x,  y),
        // Rotate 180 degrees around x, now we're looking down z-
        (-z, -y, -x),
        (-z, -x,  y),
        (-z,  y,  x),
        (-z,  x, -y),
    ][orientation]
}

fn add(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize, isize) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn sub(a: (isize, isize, isize), b: (isize, isize, isize)) -> (isize, isize, isize) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub fn part1() {
    let scanners_raw = include_str!("input").split("\n\n");

    let mut scanners = vec![];
    for scanner_raw in scanners_raw {
        let mut observations = vec![];
        for line in scanner_raw.lines().filter(|l| !l.is_empty()).skip(1) {
            let mut parts = line.split(',').map(|s| isize::from_str(s).unwrap());
            let coords = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
            observations.push(coords);
        }
        scanners.push(observations);
    }

    // Here we hold the beacons we know exist
    let mut map = HashSet::new();
    map.extend(scanners.remove(0));

    let mut scanner_coords = vec![(0, 0, 0)];
    while !scanners.is_empty() {
        let mut scanner_id = None;
        // Try to find the next scanner that has some orientation that has 12 common points
        'search: for (i, scanner) in scanners.iter().enumerate() {
            // We will try to align the observations with all the known beacons
            for beacon in &map {
                for orientation in 0..24 {
                    for observation in scanner {
                        // Let's assume observation == beacon
                        let offset = sub(*beacon, rotate_coord(*observation, orientation));

                        // Now let's see how much overlap we have
                        let mut overlap = 0;
                        for observation in scanner {
                            let observation = add(rotate_coord(*observation, orientation), offset);
                            if map.contains(&observation) {
                                overlap += 1;
                            }
                            if overlap >= 12 {
                                // We have found an new piece of the map
                                map.extend(scanner.iter().map(|observation| {
                                    add(rotate_coord(*observation, orientation), offset)
                                }));
                                scanner_id = Some(i);
                                scanner_coords.push(offset);
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
        scanners.remove(scanner_id.unwrap());
    }
    dbg!(map.len());
    let mut max_distance = 0;
    for (i, a) in scanner_coords.iter().copied().enumerate() {
        for (j, b) in scanner_coords.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            max_distance = std::cmp::max(max_distance, (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs());
        }
    }
    dbg!(max_distance);
}
