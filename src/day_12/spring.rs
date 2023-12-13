use std::slice::Windows;

#[derive(Debug, PartialEq, Clone)]
enum SpringCondition {
    Damaged,
    Good,
    Unknown,
}

#[derive(Debug)]
struct SpringRecord {
    condition: Vec<SpringCondition>,
    record: Vec<u32>,
}

impl SpringRecord {
    fn check_cond(&self, cond: &Vec<SpringCondition>) -> bool {
        let mut g_rec = vec![];
        let mut cnt = 0;
        for cond in cond.iter() {
            if cond == &SpringCondition::Damaged {
                cnt += 1;
            } else if 0 < cnt {
                g_rec.push(cnt);
                cnt = 0;
            }
        }
        if 0 < cnt {
            g_rec.push(cnt);
        }
        self.record == g_rec
    }
    fn valid(&self) -> bool {
        let mut g_rec = vec![];
        let mut cnt = 0;
        for cond in self.condition.iter() {
            if cond == &SpringCondition::Damaged {
                cnt += 1;
            } else if 0 < cnt {
                g_rec.push(cnt);
                cnt = 0;
            }
        }
        if 0 < cnt {
            g_rec.push(cnt);
        }
        self.record == g_rec
    }
    fn unfold(&self) -> SpringRecord {
        let mut condition = vec![];
        condition.append(&mut self.condition.clone());
        condition.push(SpringCondition::Unknown);
        condition.append(&mut self.condition.clone());
        condition.push(SpringCondition::Unknown);
        condition.append(&mut self.condition.clone());
        condition.push(SpringCondition::Unknown);
        condition.append(&mut self.condition.clone());
        condition.push(SpringCondition::Unknown);
        condition.append(&mut self.condition.clone());
        let mut record = vec![];
        record.append(&mut self.record.clone());
        record.append(&mut self.record.clone());
        record.append(&mut self.record.clone());
        record.append(&mut self.record.clone());
        record.append(&mut self.record.clone());
        SpringRecord { condition, record }
    }
    fn generate_posibale(&self) -> usize {
        let mut conditions = vec![vec![]];

        for item in &self.condition {
            if item == &SpringCondition::Unknown {
                // split
                let mut good = (&conditions).clone();
                for gen in &mut good {
                    gen.push(SpringCondition::Good);
                }
                for gen in &mut conditions {
                    gen.push(SpringCondition::Damaged);
                }
                conditions.append(&mut good);
            } else {
                for gen in &mut conditions {
                    gen.push(item.clone());
                }
            }
        }
        conditions.iter().map(|con | SpringRecord { condition: con.clone(), record: self.record.clone() }.valid()   ).count()
    }

    fn generate_posibale_rec(&self) -> Vec<SpringRecord> {
        let mut pos = vec![];
        let mut conditions = vec![vec![]];

        for item in &self.condition {
            if item == &SpringCondition::Unknown {
                // split
                let mut good = (&conditions).clone();
                for gen in &mut good {
                    gen.push(SpringCondition::Good);
                }
                for gen in &mut conditions {
                    gen.push(SpringCondition::Damaged);
                }
                conditions.append(&mut good);
            } else {
                for gen in &mut conditions {
                    gen.push(item.clone());
                }
            }
        }
        for con in conditions {
            pos.push(SpringRecord { condition: con, record: self.record.clone() })
        }
        pos
    }


    fn amount_posibale(&self) -> usize {
        let mut pos = 1;
        for item in &self.record {
            let mut multi = 0;
            println!("atempt to insert {}", item);
            for slice in self.condition.windows(2 + *item as usize) {
                if !(slice.first().unwrap() == &SpringCondition::Damaged) && !(slice.last().unwrap() == &SpringCondition::Damaged) {
                    for i in 1..=*item as usize {
                        if slice[i] == SpringCondition::Good {
                            break;
                        }
                    }
                    multi += 1;
                }

            }
            pos *= multi;
        }
        pos
    }
}

// ???.### 1,1,3
fn generate_record(line: &str) -> SpringRecord {
    let (cond_str, record_str) = line.split_once(' ').expect("aalways valid?");

    let condition = cond_str
        .chars()
        .map(|c| match c {
            '.' => SpringCondition::Good,
            '#' => SpringCondition::Damaged,
            _ => SpringCondition::Unknown,
        })
        .collect();
    let record = record_str
        .split(',')
        .map(|c| c.parse::<u32>().expect("allways valid"))
        .collect();
    SpringRecord { condition, record }
}

pub fn unscramble() {
    let contents = std::fs::read_to_string("docs/12/short.txt").expect("whoops");
    let records: Vec<SpringRecord> = contents.lines().map(generate_record).collect();

    let cnt: usize = records
        .iter()
        .map(|item| {
            item.unfold().amount_posibale()
        })
        .sum();

    println!("Total: {}", cnt);
}
