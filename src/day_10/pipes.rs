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
                let mut step = 0;
                let mut prev_loc = (i,j);
                // baased on our input we can go straight down
                let mut this_loc = (i + 1, j);
                loop {
                    let tmp_loc = prev_loc;
                    prev_loc = this_loc;
                    match matrix[this_loc.0][this_loc.1] {
                        'S' => {
                            // transform based on our input
                            matrix[this_loc.0][this_loc.1] = 'I';
                        },
                        '|' => {
                            matrix[this_loc.0][this_loc.1] = 'I'; 
                            this_loc = (this_loc.0 + this_loc.0 - tmp_loc.0, this_loc.1)
                        },
                        '-' => {
                            matrix[this_loc.0][this_loc.1] = '_';
                            this_loc = (this_loc.0, this_loc.1 + this_loc.1 - tmp_loc.1)
                        },
                        'L' => {
                            matrix[this_loc.0][this_loc.1] = 'l';
                            if this_loc.0 == tmp_loc.0 {
                                this_loc = (this_loc.0 - 1, this_loc.1)
                            } else {
                                this_loc = (this_loc.0, this_loc.1 + 1)
                            }
                        },
                        'J' => {
                            matrix[this_loc.0][this_loc.1] = 'j';
                            if this_loc.0 == tmp_loc.0 {
                                this_loc = (this_loc.0 - 1, this_loc.1)
                            } else {
                                this_loc = (this_loc.0, this_loc.1 - 1)
                            }
                        },
                        '7' => {
                            matrix[this_loc.0][this_loc.1] = '/';
                            if this_loc.0 == tmp_loc.0 {
                                this_loc = (this_loc.0 + 1, this_loc.1)
                            } else {
                                this_loc = (this_loc.0, this_loc.1 - 1)
                            }
                        },
                        'F' => {
                            matrix[this_loc.0][this_loc.1] = 'f';
                            if this_loc.0 == tmp_loc.0 {
                                this_loc = (this_loc.0 + 1, this_loc.1)
                            } else {
                                this_loc = (this_loc.0, this_loc.1 + 1)
                            }
                        },
                        _ => unreachable!()                        
                    }

                    step += 1;
                    if prev_loc == (i,j) {
                        break;
                    }
                }
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
