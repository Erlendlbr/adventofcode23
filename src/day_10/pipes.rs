use std::usize;

const INPUT_SIZE: usize = 140;
// const INPUT_SIZE: usize = 5;

pub fn traverse() {
    let contents = std::fs::read_to_string("docs/10/long.txt").expect("whoops");
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        println!();
    }

    for i in 0..INPUT_SIZE {
        for j in 0..INPUT_SIZE {
            if matrix[i][j] == 'S' {
                let step = step_through((i, j), (i + 1, j), &mut matrix);
                println!("we had to take {} steps", step);
                println!("So to midway {} steps", step / 2);
                break;
            }
        }
    }
    let mut box_cnt = 0;
    let mut in_box = false;
    let mut on_line = false;
    println!("initial state: {}, {}", in_box, on_line);
    for line in matrix.iter() {
        in_box = false;
        for ele in line {
            match ele {
                'I' => {
                    on_line = true;
                    in_box = !in_box;
                }
                '_' => {
                    on_line = true;
                }
                'l' => {
                    on_line = true;
                }
                'j' => {
                    on_line = true;
                }
                '/' => {
                    on_line = true;
                    in_box = !in_box;
                }
                'f' => {
                    on_line = true;
                    in_box = !in_box;
                }
                _ => {
                    on_line = false;
                }
            }

            if in_box && !on_line {
                box_cnt += 1;
            }
        }
    }
    println!("amount in box is {}", box_cnt);
}

fn step_through(
    prev: (usize, usize),
    check: (usize, usize),
    matrix: &mut [[char; INPUT_SIZE]; INPUT_SIZE],
) -> u32 {
    match matrix[check.0][check.1] {
        'S' => {
            matrix[check.0][check.1] = 'I';

            1
        }
        '|' => {
            matrix[check.0][check.1] = 'I';
            step_through(check, (check.0 + check.0 - prev.0, check.1), matrix) + 1
        }
        '-' => {
            matrix[check.0][check.1] = '_';
            step_through(check, (check.0, check.1 + check.1 - prev.1), matrix) + 1
        }
        'L' => {
            matrix[check.0][check.1] = 'l';
            if check.0 == prev.0 {
                step_through(check, (check.0 - 1, check.1), matrix) + 1
            } else {
                step_through(check, (check.0, check.1 + 1), matrix) + 1
            }
        }
        'J' => {
            matrix[check.0][check.1] = 'j';
            if check.0 == prev.0 {
                step_through(check, (check.0 - 1, check.1), matrix) + 1
            } else {
                step_through(check, (check.0, check.1 - 1), matrix) + 1
            }
        }
        '7' => {
            matrix[check.0][check.1] = '/';
            if check.0 == prev.0 {
                step_through(check, (check.0 + 1, check.1), matrix) + 1
            } else {
                step_through(check, (check.0, check.1 - 1), matrix) + 1
            }
        }
        'F' => {
            matrix[check.0][check.1] = 'f';
            if check.0 == prev.0 {
                step_through(check, (check.0 + 1, check.1), matrix) + 1
            } else {
                step_through(check, (check.0, check.1 + 1), matrix) + 1
            }
        }
        _ => {
            println!("check {:?}", check);
            unreachable!()
        }
    }
}
