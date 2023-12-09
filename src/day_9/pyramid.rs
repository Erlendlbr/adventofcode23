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

    let rec_sum: i32 = data.iter().map(call_rec).sum();
    println!("recursive calc: {}", rec_sum);
    let rev: i32 = data.iter().map(calc_rev).sum();
    println!("the reverse is {}", rev);
}

fn calc_rev(vec: &Vec<i32>) -> i32 {
    let mut rev = vec.clone();
    rev.reverse();
    call_rec(&rev)
}

fn call_rec(vec: &Vec<i32>) -> i32 {
    vec.last().unwrap() + rec_calc(vec)
}

fn rec_calc(vec: &Vec<i32>) -> i32 {
    let into: Vec<i32> = vec
        .iter()
        .enumerate()
        .filter(|tup| 0 < tup.0)
        .map(|tup| tup.1 - vec[tup.0 - 1])
        .collect();

    if into.iter().all(|i| i == &0) {
        0
    } else {
        into.last().unwrap() + rec_calc(&into)
    }
}
