use std::collections::HashSet;

// const INPUT_SIZE: usize = 10;
const INPUT_SIZE: usize = 110;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

pub fn cast_beam() {
    let contents = std::fs::read_to_string("docs/16/map.txt").expect("whoops");
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    let mut energized: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        println!();
    }

    let mut list_of_starts = vec![];

    for i in 0..INPUT_SIZE {
        list_of_starts.push(((i, 0), CardinalDirection::East));
        list_of_starts.push(((i, INPUT_SIZE - 1), CardinalDirection::West));
        list_of_starts.push(((INPUT_SIZE - 1, i), CardinalDirection::North));
        list_of_starts.push(((0, i), CardinalDirection::South));
    }
    let mut curr_best = 0;
    while !list_of_starts.is_empty() {
        let start = list_of_starts.pop().unwrap();
        let val = Some((
            Some(start),
            None::<((usize, usize), CardinalDirection)>,
        ));
        let mut locations = vec![val];
        let mut seen_start = HashSet::new();
        while !locations.is_empty() {
            let items_un = locations.pop();
            if items_un.is_none() {
                continue;
            }
            let items = items_un.unwrap();
            println!("{}", locations.len());
            match items {
                Some(tup) => {
                    match tup.0 {
                        Some((loc, direction)) => {
                            let pos_pos = send_beam(loc, &direction, &matrix, &mut energized);
                            if !seen_start.contains(&pos_pos) {
                                seen_start.insert(pos_pos.clone());
                                locations.push(pos_pos);
                            }
                        }
                        _ => {}
                    }
                    match tup.1 {
                        Some((loc, direction)) => {
                            let pos_pos = send_beam(loc, &direction, &matrix, &mut energized);
                            if !seen_start.contains(&pos_pos) {
                                seen_start.insert(pos_pos.clone());
                                locations.push(pos_pos);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            // for linr in &locations {
            //     println!("loc: {:?}", linr);
            // }
            println!();
            println!();
            // for line in energized {
            //     println!("line: {:?}", line);
            // }
        }

        // for line in &energized {
        //     println!("line: {:?}", line);
        // }
        let sum: usize = energized
            .into_iter()
            .map(|arr| arr.into_iter().filter(|c| c == &'#').count())
            .sum();
        curr_best = sum.max(curr_best);

        energized = [['.'; INPUT_SIZE]; INPUT_SIZE];
    }

    println!("energized {}", curr_best);
}

fn send_beam(
    loc: (usize, usize),
    direction: &CardinalDirection,
    matrix: &[[char; INPUT_SIZE]; INPUT_SIZE],
    energized: &mut [[char; INPUT_SIZE]; INPUT_SIZE],
) -> Option<(
    Option<((usize, usize), CardinalDirection)>,
    Option<((usize, usize), CardinalDirection)>,
)> {
    energized[loc.0][loc.1] = '#';
    let mut next_loc = loc;
    let mut current_direction = direction;
    match matrix[loc.0][loc.1] {
        '-' => match current_direction {
            CardinalDirection::East | CardinalDirection::West => {}
            CardinalDirection::North | CardinalDirection::South => {
                let loc_east = get_next_loc(next_loc, &CardinalDirection::East);
                let loc_west = get_next_loc(next_loc, &CardinalDirection::West);
                let mut part_a = None;
                let mut part_b = None;
                if loc_east.is_some() {
                    part_a = Some((next_loc, CardinalDirection::East));
                }
                if loc_west.is_some() {
                    part_b = Some((next_loc, CardinalDirection::West));
                }
                return Some((part_a, part_b));
            }
        },
        '|' => match current_direction {
            CardinalDirection::North | CardinalDirection::South => {}
            CardinalDirection::East | CardinalDirection::West => {
                let loc_north = get_next_loc(next_loc, &CardinalDirection::North);
                let loc_south = get_next_loc(next_loc, &CardinalDirection::South);
                let mut part_a = None;
                let mut part_b = None;
                if loc_north.is_some() {
                    part_a = Some((next_loc, CardinalDirection::North));
                }
                if loc_south.is_some() {
                    part_b = Some((next_loc, CardinalDirection::South));
                }
                return Some((part_a, part_b));
            }
        },
        '/' => match current_direction {
            CardinalDirection::North => current_direction = &CardinalDirection::East,
            CardinalDirection::South => current_direction = &CardinalDirection::West,
            CardinalDirection::West => current_direction = &CardinalDirection::South,
            CardinalDirection::East => current_direction = &CardinalDirection::North,
        },
        '\\' => match current_direction {
            CardinalDirection::North => current_direction = &CardinalDirection::West,
            CardinalDirection::South => current_direction = &CardinalDirection::East,
            CardinalDirection::West => current_direction = &CardinalDirection::North,
            CardinalDirection::East => current_direction = &CardinalDirection::South,
        },
        '.' => {}
        _ => {}
    }
    loop {
        let new_loc = get_next_loc(next_loc, current_direction);
        if new_loc.is_some() {
            energized[new_loc.unwrap().0][new_loc.unwrap().1] = '#';
            next_loc = new_loc.unwrap();
            match matrix[new_loc.unwrap().0][new_loc.unwrap().1] {
                '-' => match current_direction {
                    CardinalDirection::East | CardinalDirection::West => {}
                    CardinalDirection::North | CardinalDirection::South => {
                        energized[new_loc.unwrap().0][new_loc.unwrap().1] = '#';
                        let loc_east = get_next_loc(next_loc, &CardinalDirection::East);
                        let loc_west = get_next_loc(next_loc, &CardinalDirection::West);
                        let mut part_a = None;
                        let mut part_b = None;
                        if loc_east.is_some() {
                            part_a = Some((next_loc, CardinalDirection::East));
                        }
                        if loc_west.is_some() {
                            part_b = Some((next_loc, CardinalDirection::West));
                        }
                        return Some((part_a, part_b));
                    }
                },
                '|' => match current_direction {
                    CardinalDirection::North | CardinalDirection::South => {}
                    CardinalDirection::East | CardinalDirection::West => {
                        energized[new_loc.unwrap().0][new_loc.unwrap().1] = '#';
                        let loc_north = get_next_loc(next_loc, &CardinalDirection::North);
                        let loc_south = get_next_loc(next_loc, &CardinalDirection::South);
                        let mut part_a = None;
                        let mut part_b = None;
                        if loc_north.is_some() {
                            part_a = Some((next_loc, CardinalDirection::North));
                        }
                        if loc_south.is_some() {
                            part_b = Some((next_loc, CardinalDirection::South));
                        }
                        return Some((part_a, part_b));
                    }
                },
                '/' => match current_direction {
                    CardinalDirection::North => current_direction = &CardinalDirection::East,
                    CardinalDirection::South => current_direction = &CardinalDirection::West,
                    CardinalDirection::West => current_direction = &CardinalDirection::South,
                    CardinalDirection::East => current_direction = &CardinalDirection::North,
                },
                '\\' => match current_direction {
                    CardinalDirection::North => current_direction = &CardinalDirection::West,
                    CardinalDirection::South => current_direction = &CardinalDirection::East,
                    CardinalDirection::West => current_direction = &CardinalDirection::North,
                    CardinalDirection::East => current_direction = &CardinalDirection::South,
                },
                '.' => {}
                _ => {}
            }
        } else {
            return None;
        }
        energized[new_loc.unwrap().0][new_loc.unwrap().1] = '#';
    }
}

fn get_next_loc(loc: (usize, usize), direction: &CardinalDirection) -> Option<(usize, usize)> {
    match direction {
        CardinalDirection::North => {
            if loc.0 == 0 {
                None
            } else {
                Some((loc.0 - 1, loc.1))
            }
        }
        CardinalDirection::South => {
            if loc.0 == (INPUT_SIZE - 1) {
                None
            } else {
                Some((loc.0 + 1, loc.1))
            }
        }
        CardinalDirection::West => {
            if loc.1 == 0 {
                None
            } else {
                Some((loc.0, loc.1 - 1))
            }
        }
        CardinalDirection::East => {
            if loc.1 == (INPUT_SIZE - 1) {
                None
            } else {
                Some((loc.0, loc.1 + 1))
            }
        }
    }
}
