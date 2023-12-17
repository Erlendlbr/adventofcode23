// const INPUT_SIZE: usize = 10;
const INPUT_SIZE: usize = 100;

pub fn rocks() {
    let contents = std::fs::read_to_string("docs/14/map.txt").expect("whoops");
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        println!();
    }
    println!();
    println!();

    for id in 1..=10_000 {
        // 1_000_000_000
        matrix = tilt_north(&mut matrix);
        matrix = tilt_west(&mut matrix);
        matrix = tilt_south(&mut matrix);
        matrix = tilt_east(&mut matrix);

        let weigt: usize = matrix
            .iter()
            .enumerate()
            .map(|(idx, line)| (line.iter().filter(|c| *c == &'O').count()) * (INPUT_SIZE - idx))
            .sum();
        println!("{}: {}", id, weigt)
        // if weigt == 64 {
        //     println!("id: {}", id);
        // }
    }

    // 104619
    // need to program find cycles afte i stabalized to get awnser

    println!();
    println!();
    let mut weigt = 0;
    for (idx, line) in matrix.iter().enumerate() {
        weigt += (line.iter().filter(|c| *c == &'O').count()) * (INPUT_SIZE - idx);

        // println!("line: {:?}", line);
    }

    println!("foo {}", weigt);
}

fn tilt_north(matrix: &mut [[char; INPUT_SIZE]; INPUT_SIZE]) -> [[char; INPUT_SIZE]; INPUT_SIZE] {
    for j in 0..INPUT_SIZE {
        for i in 0..INPUT_SIZE {
            if matrix[i][j] == 'O' {
                for x in (0..i).rev() {
                    match matrix[x][j] {
                        '.' => {
                            matrix[x + 1][j] = '.';
                            matrix[x][j] = 'O';
                        }
                        '#' | 'O' => {
                            // println!("foun O");
                            break;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                // println!("finished match");
            }
            // print!("{}",matrix[i][j]);
        }
        // println!("");
    }
    *matrix
}

fn tilt_south(matrix: &mut [[char; INPUT_SIZE]; INPUT_SIZE]) -> [[char; INPUT_SIZE]; INPUT_SIZE] {
    for j in 0..INPUT_SIZE {
        for i in (0..INPUT_SIZE).rev() {
            if matrix[i][j] == 'O' {
                for x in (i + 1)..INPUT_SIZE {
                    match matrix[x][j] {
                        '.' => {
                            matrix[x - 1][j] = '.';
                            matrix[x][j] = 'O';
                        }
                        '#' | 'O' => {
                            // println!("foun O");
                            break;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                // println!("finished match");
            }
            // print!("{}",matrix[i][j]);
        }
        // println!("");
    }
    *matrix
}

fn tilt_east(matrix: &mut [[char; INPUT_SIZE]; INPUT_SIZE]) -> [[char; INPUT_SIZE]; INPUT_SIZE] {
    for i in 0..INPUT_SIZE {
        for j in (0..INPUT_SIZE).rev() {
            if matrix[i][j] == 'O' {
                // println!("{},{}",i,j);
                for x in (j + 1)..INPUT_SIZE {
                    match matrix[i][x] {
                        '.' => {
                            matrix[i][x - 1] = '.';
                            matrix[i][x] = 'O';
                        }
                        '#' | 'O' => {
                            // println!("found O");
                            break;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                // println!("finished match");
            }
            // print!("{}",matrix[i][j]);
        }
        // println!("");
    }
    *matrix
}

fn tilt_west(matrix: &mut [[char; INPUT_SIZE]; INPUT_SIZE]) -> [[char; INPUT_SIZE]; INPUT_SIZE] {
    for i in 0..INPUT_SIZE {
        for j in 0..INPUT_SIZE {
            if matrix[i][j] == 'O' {
                for x in (0..j).rev() {
                    match matrix[i][x] {
                        '.' => {
                            matrix[i][x + 1] = '.';
                            matrix[i][x] = 'O';
                        }
                        '#' | 'O' => {
                            // println!("foun O");
                            break;
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                // println!("finished match");
            }
            // print!("{}",matrix[i][j]);
        }
        // println!("");
    }
    *matrix
}
