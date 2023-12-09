use std::fs;

pub fn get_top_num() {
    let file1 = "docs/9/chain.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    let data: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let num: i32 = data.iter().map(|vec| calc_vec_num(vec.as_slice())).sum();
    println!("Top num is {}", num);
    let rev: i32 = data
        .iter()
        .map(|vec| calc_vec_num_rev(vec.as_slice()))
        .sum();
    println!("the reverse is {}", rev);
}

fn calc_vec_num(vec: &[i32]) -> i32 {
    let mut col_vec = vec![vec.to_owned()];
    loop {
        let look_up = col_vec.last().unwrap();
        let work_vec: Vec<i32> = look_up
            .iter()
            .enumerate()
            .filter(|tup| 0 < tup.0)
            .map(|tup| tup.1 - look_up[tup.0 - 1])
            .collect();
        if work_vec.iter().all(|i| i == &0) {
            break;
        }
        col_vec.push(work_vec);
    }
    col_vec.iter().map(|vec| vec.last().unwrap()).sum()
}

fn calc_vec_num_rev(vec: &[i32]) -> i32 {
    let mut rev = vec.to_owned();
    rev.reverse();
    calc_vec_num(rev.as_slice())
}
