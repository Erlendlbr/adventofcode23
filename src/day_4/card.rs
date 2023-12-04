use std::fs;
use std::collections::BTreeMap;

pub fn calc_games() {
    let file1 = "docs/4/card.txt";
    let mut tot = 0;
    let mut card_hit = 0;
    let contents = fs::read_to_string(file1)
        .expect("Should have been able to read the file");
    for line in contents.split('\n') {
        let (_, game_str) = line.split_once(':').unwrap();
        let (left_str, right_str) = game_str.split_once('|').unwrap();

        let left_num = left_str.trim().split(' ');
        let right_num = right_str.trim().split(' ');

        for item in left_num {
            for res in right_num.clone() {
                if (item == res) && {item != ""} {
                    card_hit += 1;
                }
            }
        }
        if 0 < card_hit {
            tot += (2 as i32).pow(card_hit - 1);
        }
        card_hit = 0;
    }
    println!("The total is {}", tot);
}

pub fn calc_cascade() {
    let file1 = "docs/4/card.txt";
    let mut tot = 0;
    let mut card_hit = 0;
    let mut casc: BTreeMap<usize, usize> = BTreeMap::new();
    let contents = fs::read_to_string(file1)
        .expect("Should have been able to read the file");

    for (count, line) in contents.split('\n').enumerate() {
        let (_, game_str) = line.split_once(':').unwrap();
        let (left_str, right_str) = game_str.split_once('|').unwrap();

        let left_num = left_str.trim().split(' ');
        let right_num = right_str.trim().split(' ');

        for item in left_num {
            for res in right_num.clone() {
                if (item == res) && {item != ""} {
                    card_hit += 1;
                }
            }
        }
        let multi = &casc.get(&count).unwrap_or(&1).clone();
        for x in 1..= card_hit {
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
        
        card_hit = 0;
    }
    println!("The cum tot = {}", tot);
}