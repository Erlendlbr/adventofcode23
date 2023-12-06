use std::fs;

struct Race {
    time: i64,
    dist: i64,
}

fn get_min_spd(time: i64, dist: i64) -> i64 {
    for speed in 1..=time {
        if dist < ((time - speed) * speed) {
            println!("the speed is {}", speed);
            return speed;
        }
    }
    0
}

fn get_max_spd(time: i64, dist: i64) -> i64 {
    for speed in (1..=time).rev() {
        if dist < ((time - speed) * speed) {
            println!("the speed is {}", speed);
            return speed;
        }
    }
    0
}

impl Race {
    fn get_pos(&self) -> i64 {
        let min = get_min_spd(self.time, self.dist);
        let max = get_max_spd(self.time, self.dist);
        println!("max max max {}", max);
        max - min + 1
    }
    fn get_pos_v1(&self) -> i64 {
        let mut cnt = 0;
        for speed in 1..=self.time {
            let timeremaining = self.time - speed;
            if self.dist < (timeremaining * speed) {
                cnt += 1;
            }
        }
        cnt
    }
}

pub fn calc_nr() {
    let file1 = "docs/6/race.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    let (line1, line2) = contents.split_once('\n').unwrap();
    let line1 = line1
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|str| str.parse::<i64>().unwrap());
    let line2 = line2
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|str| str.parse::<i64>().unwrap());

    let amout = line1
        .zip(line2)
        .map(|(time, dist)| Race { time, dist })
        .map(|race| race.get_pos_v1())
        .fold(1, |tot, pos| tot * pos);

    println!("races = {} ", amout);
}

pub fn calc_one_race() {
    let file1 = "docs/6/short.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");

    let (line1, line2) = contents.split_once('\n').unwrap();
    let line1 = line1
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .fold("".to_owned(), |comp, str| comp + str)
        .parse::<i64>()
        .unwrap();
    let line2 = line2
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .fold("".to_owned(), |comp, str| comp + str)
        .parse::<i64>()
        .unwrap();

    let race = Race {
        time: line1,
        dist: line2,
    };

    let amout = race.get_pos();

    println!("races = {} ", amout);
}
