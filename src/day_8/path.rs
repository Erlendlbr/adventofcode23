use std::collections::HashMap;
use std::fs;
enum Direction {
    Left,
    Right,
}

pub fn traverse_multi_map() {
    let file1 = "docs/8/path.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let (directions_str, maps) = contents.split_once("\n\n").unwrap();
    println!("dr strr {}", directions_str);
    let steps: Vec<Direction> = directions_str
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    let new_str = maps.replace([')', '(', ' '], "");
    let map = new_str.lines().fold(HashMap::new(), |mut acc, line| {
        let (index, rest) = line.split_once('=').unwrap();
        let tup = rest.split_once(',').unwrap();
        acc.insert(index, tup);
        acc
    });

    let locs: Vec<&str> = map
        .keys()
        .copied()
        .filter(|key| key.ends_with('A'))
        .collect();

    println!("we are starting with: {:?}", locs);

    let counts: Vec<i64> = locs
        .iter()
        .copied()
        .map(|item| {
            let mut loc = item;
            let mut cnt = 0;
            while !loc.ends_with('Z') {
                for step in &steps {
                    loc = match step {
                        Direction::Left => map.get(loc).unwrap().0,
                        Direction::Right => map.get(loc).unwrap().1,
                    };
                    cnt += 1;
                    if loc.ends_with('Z') {
                        break;
                    }
                }
            }
            cnt
        })
        .collect();

    println!("Our counts {:?}", counts);

    let len_step = steps.len() as i64;
    println!("we take {} steps", len_step);

    let smpl = counts.iter().fold(len_step, |mut acc, i| {
        acc *= i / len_step;
        acc
    });

    println!("It took {} steps to the end", smpl);

    let cnt = counts.iter().fold(1, |mut acc, i| {
        acc = num::integer::lcm(acc, *i);
        acc
    });

    println!("It took {} steps to the end", cnt);
}

pub fn traverse_map() {
    let file1 = "docs/8/path.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let (directions_str, maps) = contents.split_once("\n\n").unwrap();
    println!("dr strr {}", directions_str);
    let steps: Vec<Direction> = directions_str
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    let new_str = maps.replace([')', '(', ' '], "");
    let map = new_str.lines().fold(HashMap::new(), |mut acc, line| {
        let (index, rest) = line.split_once('=').unwrap();
        let tup = rest.split_once(',').unwrap();
        acc.insert(index, tup);
        acc
    });
    let mut loc = "AAA";
    let mut cnt = 0;
    while loc != "ZZZ" {
        for step in &steps {
            loc = match step {
                Direction::Left => map.get(loc).unwrap().0,
                Direction::Right => map.get(loc).unwrap().1,
            };
            cnt += 1;
            if loc == "ZZZ" {
                break;
            }
        }
    }
    println!("It took {} steps to the end", cnt);
}
