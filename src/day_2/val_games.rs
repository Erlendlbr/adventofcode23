use std::fs;

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn sum_games() {
    let file1 = "docs/2/games.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let accum: u32 = contents.split('\n').map(power_round).sum();

    println!("Sum is: {}", accum);
}

pub fn validate_games() {
    let nr_cube = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let file1 = "docs/2/games.txt";

    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    // println!("The file: {}", contents);

    let accum: u32 = contents
        .split('\n')
        .map(|line| validate(line, &nr_cube))
        .sum();

    println!("Sum is: {}", accum);
}

fn validate(line: &str, cube_def: &Cubes) -> u32 {
    let (game, data) = line.split_once(':').unwrap();
    let (_, c) = game.split_once(' ').unwrap();
    for round in data.split(';') {
        if !validate_round(round, cube_def) {
            return 0;
        }
    }
    c.parse::<u32>().unwrap()
}

fn validate_round(round: &str, cube_def: &Cubes) -> bool {
    let pulled_cubes = round.split(',');
    for cube_t in pulled_cubes {
        let (nr_str, collour) = cube_t.trim().split_once(' ').unwrap();
        let nr = nr_str.parse::<u32>().unwrap();
        match collour {
            "red" => {
                if cube_def.red < nr {
                    return false;
                }
            }
            "green" => {
                if cube_def.green < nr {
                    return false;
                }
            }
            "blue" => {
                if cube_def.blue < nr {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

fn power_round(line: &str) -> u32 {
    let (_game, data) = line.split_once(':').unwrap();
    let cub = get_min(data);
    cub.red * cub.green * cub.blue
}

fn get_min(line: &str) -> Cubes {
    let mut cub = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };
    for round in line.split(';') {
        let pulled_cubes = round.split(',');
        for cube_t in pulled_cubes {
            let (nr_str, collour) = cube_t.trim().split_once(' ').unwrap();
            let nr = nr_str.parse::<u32>().unwrap();
            match collour {
                "red" => {
                    if cub.red < nr {
                        cub.red = nr
                    }
                }
                "green" => {
                    if cub.green < nr {
                        cub.green = nr
                    }
                }
                "blue" => {
                    if cub.blue < nr {
                        cub.blue = nr
                    }
                }
                _ => (),
            }
        }
    }
    cub
}
