use itertools::Itertools;
use std::collections::HashMap;

pub fn part1() {
    let mut positions = vec![7, 8];
    let mut scores = vec![0, 0];
    let mut die = (1..=100).cycle();
    let mut times_rolled = 0;
    'game: loop {
        for (player, (position, score)) in positions.iter_mut().zip(scores.iter_mut()).enumerate() {
            times_rolled += 3;
            let total_die = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
            *position = ((*position - 1 + total_die) % 10) + 1;
            *score += *position;

            println!("Player {} rolls ({}): position {}, score {}", player, total_die, &position, &score);
            if *score >= 1000 {
                dbg!(&positions, &scores);
                dbg!(scores[(player + 1) % 2] * times_rolled);
                break 'game;
            }
        }
    }

}

pub fn part2() {
    let mut universes_won = [0u64, 0];

    // For every game state store how many universes it exists in
    let mut games = HashMap::new();
    games.insert(([7u64, 8], [0, 0]), 1);

    let mut temp_games = HashMap::new();
    for player in (0usize..2).cycle() {
        if games.is_empty() {
            break;
        }
        for (game_state, universes) in games.drain() {
            for ((r0, r1), r2) in (1..=3).cartesian_product(1..=3).cartesian_product(1..=3) {
                let (mut positions, mut scores) = game_state;
                positions[player] = ((positions[player] - 1 + r0 + r1 + r2) % 10) + 1;
                scores[player] += positions[player];

                if scores[player] >= 21 {
                    universes_won[player] += universes;
                } else {
                    *temp_games.entry((positions, scores)).or_insert(0) += universes;
                }
            }
        }
        std::mem::swap(&mut games, &mut temp_games);
    }
    dbg!(universes_won);
}
