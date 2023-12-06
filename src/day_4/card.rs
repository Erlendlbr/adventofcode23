use std::collections::{BTreeMap, HashSet};
use std::fs;

pub fn calc_games() {
    let file1 = "docs/4/card.txt";
    let mut tot = 0;
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    for line in contents.split('\n') {
        let (_, game_str) = line.split_once(':').unwrap();
        let (left_str, right_str) = game_str.split_once('|').unwrap();

        let left_num: HashSet<&str> = left_str.split_ascii_whitespace().into_iter().collect();
        let right_num: HashSet<&str> = right_str.split_ascii_whitespace().into_iter().collect();

        let col = left_num.intersection(&right_num).count();
        if 0 < col {
            tot += (2 as i32).pow((col as u32) - 1);
        }
    }
    println!("The total is {}", tot);
}

pub fn calc_cascade() {
    let file1 = "docs/4/card.txt";
    let mut tot = 0;
    let mut casc: BTreeMap<usize, usize> = BTreeMap::new();
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    for (count, line) in contents.split('\n').enumerate() {
        let (_, game_str) = line.split_once(':').unwrap();
        let (left_str, right_str) = game_str.split_once('|').unwrap();

        let left_num: HashSet<&str> = left_str.split_ascii_whitespace().collect();
        let right_num: HashSet<&str> = right_str.split_ascii_whitespace().collect();

        let col = left_num.intersection(&right_num).count();

        let multi = &casc.get(&count).unwrap_or(&1).clone();
        for x in 1..=col {
            match casc.get(&(count + x)) {
                Some(y) => {
                    casc.insert(count + x, y + multi);
                }
                _ => {
                    casc.insert(count + x, multi + 1);
                }
            }
        }
        tot += multi;
    }
    println!("The cum tot = {}", tot);
}
