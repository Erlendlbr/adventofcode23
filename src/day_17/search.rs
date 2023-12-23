use std::collections::{BTreeMap, VecDeque};
use std::ops::Index;

const INPUT_SIZE: usize = 13;
// const INPUT_SIZE: usize = 141;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

pub fn find_short() {
    let contents = std::fs::read_to_string("docs/17/short.txt").expect("whoops");
    let mut matrix: [[char; INPUT_SIZE]; INPUT_SIZE] = [['.'; INPUT_SIZE]; INPUT_SIZE];
    for (i, line) in contents.split('\n').enumerate() {
        for (j, ch) in line.chars().enumerate() {
            print!("{}", ch);
            matrix[i][j] = ch;
        }
        println!();
        let end = (INPUT_SIZE-1,INPUT_SIZE-1);
        let res = star_rest((0,0),end, matrix);

        println!("res = {}", res)
    }



    println!("foo");
}

fn star_rest(start:(usize,usize), goal: (usize,usize), matrix: [[char; INPUT_SIZE];INPUT_SIZE])  -> usize {
    let mut current = start;
    let mut score = 0;
    // let cameFrom = BTreeMap::new();
    let mut moving = VecDeque::new();
    moving.push_front(((0,1),CardinalDirection::East));
    moving.push_front(((0,2),CardinalDirection::East));
    moving.push_front(((0,3),CardinalDirection::East));
    let a =moving.index(0);
    let b =moving.index(1);
    let c =moving.index(2);
    println!("a {:?}", a);
    println!("b {:?}", b);
    println!("c {:?}", c);
    0
}