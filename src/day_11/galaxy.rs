const INPUT_SIZE: usize = 140;
// const INPUT_SIZE: usize = 10;

pub fn distance() {
    let contents = std::fs::read_to_string("docs/11/galaxy.txt").expect("whoops");
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        println!();
    }

    let line_expand: Vec<usize> = matrix
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| *c == '.'))
        .map(|(idx, _)| idx)
        .collect();
    let mut row_expand = vec![];
    for j in 0..INPUT_SIZE {
        let mut galaxy = false;
        for i in 0..INPUT_SIZE {
            if matrix[i][j] == '#' {
                galaxy = true;
                break;
            }
        }
        if !galaxy {
            row_expand.push(j);
        }
    }
    let expanded_map: Vec<Vec<char>> = matrix.iter().map(|slice| slice.to_vec()).collect();
    // let r_l = row_expand.len();
    // while !row_expand.is_empty() {
    //     let idx = row_expand.pop().unwrap();
    //     println!("whant to insert {}", idx);
    //     for vec in &mut expanded_map {
    //         vec.insert(idx, '.')
    //     }
    // }
    // while !line_expand.is_empty() {
    //     let idx = line_expand.pop().unwrap();
    //     expanded_map.insert(idx, vec!['.'; INPUT_SIZE + r_l]);
    //     println!("whant to insert {}", idx);
    // }
    let mut galaxy_loc: Vec<(i64, i64)> = vec![];
    for (i, line) in expanded_map.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == '#' {
                let loc = get_real_loc((i, j), &line_expand, &row_expand);
                galaxy_loc.push(loc);
            }
        }
    }
    // for line in expanded_map {
    //     for char in line {
    //         print!("{}", char);
    //     }
    //     println!("");
    // }
    let mut dst = 0;
    loop {
        let loc = galaxy_loc.pop().unwrap();
        if galaxy_loc.is_empty() {
            break;
        }
        for cmp_loc in &galaxy_loc {
            dst += (loc.0 - cmp_loc.0).abs() + (loc.1 - cmp_loc.1).abs();
        }
    }
    println!("distance total is {}", dst);
}

fn get_real_loc(
    loc: (usize, usize),
    line_ofsets: &Vec<usize>,
    row_ofsets: &Vec<usize>,
) -> (i64, i64) {
    let line_multi = line_ofsets.iter().filter(|place| place < &&loc.0).count() as i64;
    let row_multi = row_ofsets.iter().filter(|place| place < &&loc.1).count() as i64;
    let dist_multi = 1000000 - 1;
    (
        loc.0 as i64 + (line_multi * dist_multi),
        loc.1 as i64 + (row_multi * dist_multi),
    )
}
