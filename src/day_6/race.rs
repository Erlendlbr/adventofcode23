use std::fs;

struct Race {
    time: i64,
    dist: i64,
}

impl Race {
    fn get_pos(&self) -> i64 {
        let mut cnt = 0;
        for speed in 1..=self.time {
            let timeremaining = self.time - speed;
            if self.dist < timeremaining * speed {
                cnt += 1;
            }
        }
        cnt
    }
}

pub fn calc_nr() {
    let file1 = "docs/6/race.txt";
    // let file1 = "docs/5/short.txt";
    let contents = fs::read_to_string(file1)
        .expect("Should have been able to read the file");

    let (line1,line2) = contents.split_once('\n').unwrap();
    let line1 = line1.split_once(':').unwrap().1.split_ascii_whitespace()
        .map(|str| str.parse::<i64>().unwrap());
    let line2 = line2.split_once(':').unwrap().1.split_ascii_whitespace()
        .map(|str| str.parse::<i64>().unwrap());

    let amout = line1.zip(line2)
        .map(| (time, dist) | Race{time, dist })
        .map(|race| race.get_pos()).fold(1, | tot, pos | tot * pos );

    
    println!("races = {} ", amout);
}

pub fn calc_one_race() {
    let file1 = "docs/6/race.txt";
    // let file1 = "docs/5/short.txt";
    let contents = fs::read_to_string(file1)
        .expect("Should have been able to read the file");

    let (line1,line2) = contents.split_once('\n').unwrap();
    let line1 = line1.split_once(':').unwrap().1.split_ascii_whitespace()
        .fold("".to_owned(), | comp, str | comp + str)
        .parse::<i64>().unwrap();
    let line2 = line2.split_once(':').unwrap().1.split_ascii_whitespace()
        .fold("".to_owned(), | comp, str |comp + str)
        .parse::<i64>().unwrap();

    let race = Race{time: line1, dist: line2};

    let amout = race.get_pos();

    
    println!("races = {} ", amout);
}
