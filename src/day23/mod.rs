use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::cmp::Reverse;

#[derive(Clone,Ord,PartialOrd,Eq,PartialEq,Hash)]
struct Burrow<const ROOM_SIZE: usize> {
    rooms: [[Option<usize>; ROOM_SIZE]; 4],
    hall: [Option<usize>; 11],
}

impl<const ROOM_SIZE: usize> Burrow<ROOM_SIZE> {
    const ORGANIZED: Burrow<ROOM_SIZE> = Burrow {
        rooms: [
            [Some(0); ROOM_SIZE],
            [Some(1); ROOM_SIZE],
            [Some(2); ROOM_SIZE],
            [Some(3); ROOM_SIZE],
        ],
        hall: [None; 11],
    };
}

impl<const ROOM_SIZE: usize> fmt::Display for Burrow<ROOM_SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#############\n#")?;
        for position in self.hall {
            match position {
                Some(n) => write!(f, "{}", char::from_u32('A' as u32 + n as u32).unwrap())?,
                None => f.write_str(".")?,
            }
        }
        f.write_str("#\n")?;
        for i in 0..ROOM_SIZE {
            if i == 0 {
                f.write_str("###")?;
            } else {
                f.write_str("  #")?;
            }
            for room in self.rooms {
                match room[ROOM_SIZE - i - 1] {
                    Some(n) => write!(f, "{}", char::from_u32('A' as u32 + n as u32).unwrap())?,
                    None => f.write_str(".")?,
                }
                f.write_str("#")?;
            }
            if i == 0 {
                f.write_str("##\n")?;
            } else {
                f.write_str("  \n")?;
            }
        }
        f.write_str("  #########")
    }
}

#[derive(Clone,Ord,PartialOrd,Eq,PartialEq)]
struct GameState<const ROOM_SIZE: usize> {
    energy: usize,
    burrow: Burrow<ROOM_SIZE>,
}

pub fn part1() {
    let part1 = Burrow {
        rooms: [
            [Some(2), Some(1)],
            [Some(3), Some(3)],
            [Some(1), Some(2)],
            [Some(0), Some(0)],
        ],
        hall: [None; 11],
    };
    let part2 = Burrow {
        rooms: [
            [Some(2), Some(3), Some(3), Some(1)],
            [Some(3), Some(1), Some(2), Some(3)],
            [Some(1), Some(0), Some(1), Some(2)],
            [Some(0), Some(2), Some(0), Some(0)],
        ],
        hall: [None; 11],
    };

    println!("Part 1: {}", solve(part1));
    println!();
    println!("Part 2: {}", solve(part2));
}

fn solve<const ROOM_SIZE: usize>(burrow: Burrow<ROOM_SIZE>) -> usize {
    println!("{}", &burrow);

    let initial = GameState {
        energy: 0,
        burrow,
    };

    const ENERGY: [usize; 4] = [1, 10, 100, 1000];

    // Records the minimum energy we have managed to use to reach a particular burrow state
    let mut explored_states = HashMap::new();

    // Use a min-heap to always explore the cheapest states first
    let mut heap = BinaryHeap::from([Reverse(initial)]);

    while let Some(Reverse(state)) = heap.pop() {
        if state.burrow == Burrow::ORGANIZED {
            return state.energy;
        }

        // If we have already explored this state using less energy, there is no point
        // continuing this exploration
        if let Some(min_energy) = explored_states.get(&state.burrow) {
            if *min_energy < state.energy {
                continue;
            }
        }

        // First, attempt to move the top amphipod of each room into each
        // available position of the hallway.
        for (room, positions) in state.burrow.rooms.iter().enumerate() {
            for (i, position) in positions.iter().enumerate().rev() {
                let amphipod = match position {
                    Some(amphipod) => *amphipod,
                    None => continue,
                };
                // An amphipod needs to exit if there are amphipods below it that need to move
                if positions[0..=i].iter().any(|position| position != &Some(room)) {
                    let hall_position_init = 2 + room * 2;

                    // First, it needs to move in the hallway above its room. This position is guaranteed to be empty
                    let exit_energy = (ROOM_SIZE - i) * ENERGY[amphipod];

                    // Next, for every free position of the hallway to the right and to the left
                    // generate new game states. If there is none then this amphipod cannot move at
                    // this time
                    for hall_position in (hall_position_init + 1)..11 {
                        // If it's occupied, we're done
                        if state.burrow.hall[hall_position].is_some() {
                            break;
                        }
                        // If it's a room position we must keep going
                        if matches!(hall_position, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        // We found a destination hall position and a new game state
                        let hall_energy = (hall_position - hall_position_init) * ENERGY[amphipod];

                        let mut new_state = state.clone();
                        new_state.energy += exit_energy + hall_energy;
                        new_state.burrow.rooms[room][i] = None;
                        new_state.burrow.hall[hall_position] = Some(amphipod);

                        let mut explore = true;
                        if let Some(min_energy) = explored_states.get(&new_state.burrow) {
                            // If we're about to enter a burrow state we've seen before with less
                            // or equal energy we don't have to explore
                            if *min_energy <= new_state.energy {
                                explore = false;
                            }
                        }
                        if explore {
                            explored_states.insert(new_state.burrow.clone(), new_state.energy);
                            heap.push(Reverse(new_state));
                        }
                    }

                    // Now we go to the left
                    for hall_position in (0..hall_position_init).rev() {
                        // If it's occupied, we're done
                        if state.burrow.hall[hall_position].is_some() {
                            break;
                        }
                        // If it's a room position we must keep going
                        if matches!(hall_position, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        // We found a destination hall position and a new game state
                        let hall_energy = (hall_position_init - hall_position) * ENERGY[amphipod];

                        let mut new_state = state.clone();
                        new_state.energy += exit_energy + hall_energy;
                        new_state.burrow.rooms[room][i] = None;
                        new_state.burrow.hall[hall_position] = Some(amphipod);

                        let mut explore = true;
                        if let Some(min_energy) = explored_states.get(&new_state.burrow) {
                            // If we're about to enter a burrow state we've seen before with less
                            // or equal energy we don't have to explore
                            if *min_energy <= new_state.energy {
                                explore = false;
                            }
                        }
                        if explore {
                            explored_states.insert(new_state.burrow.clone(), new_state.energy);
                            heap.push(Reverse(new_state));
                        }
                    }
                }
                // We only deal with the first amphipod we find
                break;
            }
        }

        // Then, attempt to move each amphipod from the hallway into its respective room
        for (hall_idx, position) in state.burrow.hall.iter().enumerate() {
            let amphipod = match position {
                Some(amphipod) => *amphipod,
                None => continue,
            };

            // First, check if the way to the entry to the room is blocked
            let room_entry = 2 + amphipod * 2;
            let hall_range = if room_entry < hall_idx {
                (room_entry, hall_idx)
            } else {
                (hall_idx + 1, room_entry + 1)
            };
            if state.burrow.hall[hall_range.0..hall_range.1].iter().any(|x| x.is_some()) {
                continue;
            }

            // Then, check if it can go to its room. If there is even one amphipod that shouldn't
            // be there, we can't go
            for (room_idx, occupant) in state.burrow.rooms[amphipod].iter().enumerate() {
                if let Some(occupant) = *occupant {
                    if occupant == amphipod {
                        // We can still go if it's occupied by the right amphipod
                        continue;
                    } else {
                        // No go
                        break;
                    }
                }

                // The path is clear, so generate a new game state
                let hall_energy = (hall_range.1 - hall_range.0) * ENERGY[amphipod];
                let entry_energy = (ROOM_SIZE - room_idx) * ENERGY[amphipod];

                let mut new_state = state.clone();
                new_state.energy += entry_energy + hall_energy;
                new_state.burrow.rooms[amphipod][room_idx] = Some(amphipod);
                new_state.burrow.hall[hall_idx] = None;

                let mut explore = true;
                if let Some(min_energy) = explored_states.get(&new_state.burrow) {
                    // If we're about to enter a burrow state we've seen before with less
                    // or equal energy we don't have to explore
                    if *min_energy <= new_state.energy {
                        explore = false;
                    }
                }
                if explore {
                    explored_states.insert(new_state.burrow.clone(), new_state.energy);
                    heap.push(Reverse(new_state));
                }
                break;
            }
        }
    }
    panic!("couldn't find a solution");
}
