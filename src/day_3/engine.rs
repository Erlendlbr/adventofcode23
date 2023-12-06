use std::{fs, usize};

const INPUT_SIZE: usize = 140;
const RADIX: u32 = 10;

pub fn get_engine_nr() {
    let mat = load_engine();
    let en_nr = calculate_engine(mat);
    println!("The nr is: {}", en_nr);
    let ratio = get_ratio(mat);
    println!("gear ratio: {}", ratio);
}

fn load_engine() -> [[char; INPUT_SIZE]; INPUT_SIZE] {
    // const INPUT_SIZE: usize = 10;
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    let file1 = "docs/3/engine.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        print!("\n");
    }
    matrix
}

fn get_ratio(matrix: [[char; INPUT_SIZE]; INPUT_SIZE]) -> u32 {
    let mut tot = 0;
    for i in 0..INPUT_SIZE {
        for j in 0..INPUT_SIZE {
            match matrix[i][j] {
                '*' => {
                    let (nr1, nr2) = get_nr(matrix, (i, j));
                    tot += nr1 * nr2;
                }
                _ => (),
            }
        }
    }
    tot
}

fn get_nr(matrix: [[char; INPUT_SIZE]; INPUT_SIZE], loc: (usize, usize)) -> (u32, u32) {
    let (i, j) = loc;
    let mut nr1 = 0;
    let mut nr2 = 0;

    if matrix[i - 1][j].to_digit(RADIX).unwrap_or(11) < 11 {
        let loc = get_start(matrix, (i - 1, j));
        let t = nr_from_loc(matrix, loc);
        if nr1 == 0 {
            nr1 = t;
        } else if nr2 == 0 {
            nr2 = t;
        } else {
            return (0, 0);
        }
    } else {
        if matrix[i - 1][j - 1].to_digit(RADIX).unwrap_or(11) < 11 {
            let loc = get_start(matrix, (i - 1, j - 1));
            let t = nr_from_loc(matrix, loc);
            if nr1 == 0 {
                nr1 = t;
            } else if nr2 == 0 {
                nr2 = t;
            } else {
                return (0, 0);
            }
        }
        if matrix[i - 1][j + 1].to_digit(RADIX).unwrap_or(11) < 11 {
            let t = nr_from_loc(matrix, (i - 1, j + 1));
            if nr1 == 0 {
                nr1 = t;
            } else if nr2 == 0 {
                nr2 = t;
            } else {
                return (0, 0);
            }
        }
    }
    if matrix[i][j - 1].to_digit(RADIX).unwrap_or(11) < 11 {
        let loc = get_start(matrix, (i, j - 1));
        let t = nr_from_loc(matrix, loc);
        if nr1 == 0 {
            nr1 = t;
        } else if nr2 == 0 {
            nr2 = t;
        } else {
            return (0, 0);
        }
    }
    if matrix[i][j + 1].to_digit(RADIX).unwrap_or(11) < 11 {
        let t = nr_from_loc(matrix, (i, j + 1));
        if nr1 == 0 {
            nr1 = t;
        } else if nr2 == 0 {
            nr2 = t;
        } else {
            return (0, 0);
        }
    }
    if matrix[i + 1][j].to_digit(RADIX).unwrap_or(11) < 11 {
        let loc = get_start(matrix, (i + 1, j));
        let t = nr_from_loc(matrix, loc);
        if nr1 == 0 {
            nr1 = t;
        } else if nr2 == 0 {
            nr2 = t;
        } else {
            return (0, 0);
        }
    } else {
        if matrix[i + 1][j - 1].to_digit(RADIX).unwrap_or(11) < 11 {
            let loc = get_start(matrix, (i + 1, j - 1));
            let t = nr_from_loc(matrix, loc);
            if nr1 == 0 {
                nr1 = t;
            } else if nr2 == 0 {
                nr2 = t;
            } else {
                return (0, 0);
            }
        }
        if matrix[i + 1][j + 1].to_digit(RADIX).unwrap_or(11) < 11 {
            let t = nr_from_loc(matrix, (i + 1, j + 1));
            if nr1 == 0 {
                nr1 = t;
            } else if nr2 == 0 {
                nr2 = t;
            } else {
                return (0, 0);
            }
        }
    }
    (nr1, nr2)
}

fn get_start(matrix: [[char; INPUT_SIZE]; INPUT_SIZE], loc: (usize, usize)) -> (usize, usize) {
    let (i, mut j) = loc;
    while matrix[i][j].to_digit(RADIX).unwrap_or(11) < 11 {
        if j == 0 {
            return (i, j);
        }
        j -= 1;
    }
    (i, j + 1)
}

fn nr_from_loc(matrix: [[char; INPUT_SIZE]; INPUT_SIZE], loc: (usize, usize)) -> u32 {
    let (i, mut j) = loc;
    let mut cur_nr = 0;
    while matrix[i][j].to_digit(RADIX).unwrap_or(11) < 11 {
        cur_nr = (cur_nr * 10) + matrix[i][j].to_digit(RADIX).unwrap();
        j += 1;
        if j == INPUT_SIZE {
            return cur_nr;
        }
    }
    cur_nr
}

fn calculate_engine(matrix: [[char; INPUT_SIZE]; INPUT_SIZE]) -> u32 {
    let mut tot = 0;
    let mut cur_nr = 0;
    let mut slice = vec![];
    for i in 0..INPUT_SIZE {
        for j in 0..INPUT_SIZE {
            match matrix[i][j].to_digit(RADIX) {
                Some(x) => {
                    cur_nr = (cur_nr * 10) + x;
                    slice.push((i + 1, j));
                    slice.push((i + 1, j + 1));
                    slice.push((i, j + 1));
                    if 0 < i {
                        slice.push((i - 1, j));
                        slice.push((i - 1, j + 1));
                    }
                    if 0 < j {
                        slice.push((i, j - 1));
                        slice.push((i + 1, j - 1));
                    }
                    if (0 < i) && (0 < j) {
                        slice.push((i - 1, j - 1));
                    }
                }
                _ => {
                    if 0 < cur_nr {
                        if validate_nr(matrix, slice) {
                            tot += cur_nr;
                        }
                    }
                    cur_nr = 0;
                    slice = vec![];
                }
            }
        }
        if 0 < cur_nr {
            if validate_nr(matrix, slice) {
                tot += cur_nr;
            }
        }
        cur_nr = 0;
        slice = vec![];
    }
    tot
}

fn validate_nr(matrix: [[char; INPUT_SIZE]; INPUT_SIZE], loc: Vec<(usize, usize)>) -> bool {
    for (x, y) in loc {
        if (x < INPUT_SIZE) && (y < INPUT_SIZE) {
            match matrix[x][y] {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
                '.' => {}
                _ => {
                    return true;
                }
            }
        }
    }
    return false;
}
