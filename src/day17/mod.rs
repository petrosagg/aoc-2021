use std::collections::HashSet;

pub fn part1() {
    let target_area = ((57,116), (-198,-148));
    // let target_area = ((20,30),(-10,-5));

    let mut best_vel_x = 0;
    // Find first x velocity that just about enters the area
    for vel_x in 0..100 {
        // We can do a max of vel_x steps before reaching speed 0
        let distance_traveled = (0..=vel_x).sum();

        if target_area.0.0 <= distance_traveled && distance_traveled <= target_area.0.1 {
            best_vel_x = vel_x;
            break;
        }
    }

    let mut best_vel_y = 1000;
    let mut total_max_y = 0;
    // Now let's see how high we can go without over shooting
    'outer: for vel_y in (0..200i32).rev() {
        let mut distance_traveled = 0;
        let mut cur_vel = vel_y;
        let mut max_y = 0;

        // Keep simulating until we overshoot the target area
        for _ in 0.. {
            distance_traveled += cur_vel;
            cur_vel -= 1;

            max_y = std::cmp::max(max_y, distance_traveled);

            if target_area.1.0 <= distance_traveled && distance_traveled <= target_area.1.1 {
                best_vel_y = vel_y;
                total_max_y = max_y;
                break 'outer;
            }
            if distance_traveled < target_area.1.0 {
                break;
            }
        }
    }
    dbg!(best_vel_x);
    dbg!(best_vel_y);
    dbg!(total_max_y);
}

pub fn part2() {
    let target_area = ((57,116), (-198,-148));
    //let target_area = ((20,30),(-10,-5));

    let mut coords = HashSet::new();
    // Find first x velocity that just about enters the area
    for vel_x in 0..200 {
        let mut distance_traveled_x = 0;

        let mut cur_vel_x = vel_x;
        // For any step that gets to the target X coordinate, see how many vel_y we can find
        for step in 1..=1000 {
            distance_traveled_x += cur_vel_x;
            cur_vel_x = std::cmp::max(cur_vel_x - 1, 0);

            if target_area.0.0 <= distance_traveled_x && distance_traveled_x <= target_area.0.1 {
                // In here our X coordinate is in the target X range after `step` steps
                for vel_y in -200..200 {
                    // Calculate the Y coordinate after `step` steps and see if we're in
                    // the area

                    let mut cur_vel = vel_y;
                    let mut distance_traveled_y = 0;
                    for _ in 0..step {
                        distance_traveled_y += cur_vel;
                        cur_vel -= 1;
                    }
                    if target_area.1.0 <= distance_traveled_y && distance_traveled_y <= target_area.1.1 {
                        coords.insert((vel_x, vel_y));
                    }
                }
            }

        }
    }
    dbg!(coords.len());
}
