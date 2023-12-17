pub fn point_of() {
    let contents = std::fs::read_to_string("docs/13/mirror.txt").expect("whoops");
    let maps: Vec<Vec<&str>> = contents
        .split("\n\n")
        .map(|map| map.lines().collect())
        .collect();

    // for map in &maps {
    //     println!("split {}", get_split_1(map) );
    // }

    let su: u32 = maps.iter().map(get_split_1).sum();

    println!("sum: {}", su);
}

fn get_split_1(vec: &Vec<&str>) -> u32 {
    for (idx, line) in vec.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        let mut fixed_smudged = false;
        if !fixed_smudged {
            let amount = vec[idx]
                .chars()
                .into_iter()
                .zip(vec[idx - 1].chars().into_iter())
                .filter(|tup| tup.0 != tup.1)
                .count();
            if amount == 1 {
                fixed_smudged = true;
            }
        }
        if vec[idx] == vec[idx - 1] || fixed_smudged {
            let mut is_valid = true;
            // println!("idx: {}", line);
            let len = vec.len();
            let steps = (len - idx).min(idx);
            for off in 1..steps {
                // println!("cmp line: {}", vec[idx + off]);
                // println!("cmp line: {}", vec[idx - off - 1]);
                // println!("");
                if (vec[idx + off] != vec[idx - off - 1]) {
                    if fixed_smudged {
                        is_valid = false;
                        break;
                    }
                    let amount = vec[idx + off]
                        .chars()
                        .into_iter()
                        .zip(vec[idx - off - 1].chars().into_iter())
                        .filter(|tup| tup.0 != tup.1)
                        .count();
                    if amount == 1 {
                        fixed_smudged = true;
                    } else {
                        is_valid = false;
                        break;
                    }
                }
            }
            if is_valid && fixed_smudged {
                return 100 * idx as u32;
            }
            //  println!("finished comp");
        }
    }

    let r_len = vec[0].len();

    for idx in 0..r_len {
        if idx == 0 {
            continue;
        }
        let mut fixed_smudged = false;

        let cmp_a: Vec<char> = vec
            .iter()
            .map(|item| item.as_bytes()[idx - 1])
            .map(|c_b| c_b as char)
            .collect();
        let cmp_b: Vec<char> = vec
            .iter()
            .map(|item| item.as_bytes()[idx])
            .map(|c_b| c_b as char)
            .collect();
        // println!("cmpa {:?}", cmp_a);
        // println!("cmpb {:?}", cmp_b);
        // println!("");
        if !fixed_smudged {
            let amount = cmp_a
                .iter()
                .zip(cmp_b.iter())
                .filter(|tup| tup.0 != tup.1)
                .count();
            if amount == 1 {
                fixed_smudged = true;
                // println!("fixed: smudge");
            }
        }
        if cmp_a == cmp_b || fixed_smudged {
            let mut is_valid = true;
            let steps = (r_len - idx).min(idx);
            for off in 1..steps {
                let cmp_a: Vec<char> = vec
                    .iter()
                    .map(|item| item.as_bytes()[idx - off - 1])
                    .map(|c_b| c_b as char)
                    .collect();
                let cmp_b: Vec<char> = vec
                    .iter()
                    .map(|item| item.as_bytes()[idx + off])
                    .map(|c_b| c_b as char)
                    .collect();

                // println!("cmpa {:?}", cmp_a);
                // println!("cmpa {:?}", cmp_b);
                // println!("");
                if (cmp_a != cmp_b) {
                    if fixed_smudged {
                        is_valid = false;
                        break;
                    }
                    let amount = cmp_a
                        .iter()
                        .zip(cmp_b.iter())
                        .filter(|tup| tup.0 != tup.1)
                        .count();
                    if amount == 1 {
                        fixed_smudged = true;
                    } else {
                        is_valid = false;
                        break;
                    }
                }
            }
            // println!("is valid: {}",is_valid);
            if is_valid && fixed_smudged {
                return idx as u32;
            }
        }
    }
    println!("invalid on x");
    0
}
