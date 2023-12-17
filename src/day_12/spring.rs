#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
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
    fn check_valid_placments(&self, cond: &Vec<SpringCondition>) -> bool {
        let len = self.condition.len();
        for (idx, item) in cond.iter().enumerate() {
            if len <= idx {
                break;
            }
            if self.condition[idx] == SpringCondition::Unknown {
            } else {
                if self.condition[idx] != *item {
                    return false;
                }
            }
        }
        true
    }
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
        let cmp_val = self.record.iter().map(|&x| x as usize).sum();
        // for amount_ins in &self.record {
        //     for record in &mut conditions {

        //     }
        // }
        // 0
        // println!("line item {:?}", self.condition);

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
            conditions = conditions
                .into_iter()
                .filter(|vec| {
                    vec.iter()
                        .filter(|item| *item == &SpringCondition::Damaged)
                        .count()
                        <= cmp_val
                })
                .collect();
        }
        conditions.iter().filter(|con| self.check_cond(con)).count()
    }

    fn generate_posibale_v2(&self) -> usize {
        let mut setUP = self.condition.clone();
        setUP.push(SpringCondition::Good);
        let mut initial = setUP.as_slice();

        let initial_len = initial.len();
        let mut conditions: Vec<Vec<SpringCondition>> = vec![vec![]];
        let cmp_val: usize = self.record.iter().map(|&x| x as usize).sum();
        let mut rem = cmp_val;
        // println!("line item {:?}", self.condition);
        for amount_ins in &self.record {
            rem -= *amount_ins as usize;
            let mut col = vec![];
            for item in &conditions {
                let len = item.len();
                if initial_len < len {
                    break;
                }
                let mut tool = initial.split_at(len).1;
                if rem >= tool.len() + 1 {
                    continue;
                };
                let place = tool.binary_search(&SpringCondition::Damaged);
                // println!("foond {:?}", place);
                if place.is_ok() {
                    // println!("found {:?}", place.unwrap());
                    let split_loc = place.unwrap() + (*amount_ins as usize) + 1;
                    if split_loc < tool.len() {
                        tool = tool.split_at(split_loc).0;
                    }
                }
                // initial.split_at(len).1.windows(*amount_ins as usize + 1).map(|slice| {
                let cords: Vec<u16> = tool
                    .windows(*amount_ins as usize + 1)
                    .map(|slice| {
                        let cnt = slice
                            .iter()
                            .filter(|item| *item == &SpringCondition::Good)
                            .count();
                        if (cnt < 1 || (slice.last().unwrap() == &SpringCondition::Good && cnt < 2))
                            && slice.last().unwrap() != &SpringCondition::Damaged
                        {
                            // println!("slice good? {:?} ", slice);
                            1
                        } else {
                            0
                        }
                    })
                    .collect();
                let mut temp = vec![];

                temp = cords
                    .iter()
                    .enumerate()
                    .filter(|(_, is)| 0 < **is)
                    .map(|(idx, _)| {
                        let mut temp = item.clone();
                        for _ in 0..idx {
                            temp.push(SpringCondition::Good);
                        }
                        for _ in 0..*amount_ins {
                            temp.push(SpringCondition::Damaged);
                        }
                        temp.push(SpringCondition::Good);
                        temp
                    })
                    .collect();
                col.append(&mut temp);
                // println!("col  {:?}", col);
                // println!("we can insert at {:?}", cords);
                // println!("inserting {} into {:?}", amount_ins, item);
            }
            conditions = col;
            conditions = conditions
                .into_iter()
                .filter(|con| self.check_valid_placments(&con))
                .collect();
        }
        // for con in &conditions {
        //     println!("condition = {:?}", con);
        // }
        conditions = conditions
            .into_iter()
            .filter(|con| self.check_valid_placments(&con))
            .collect();
        return conditions.len();
    }

    fn amount_posibale_expanded(&self) -> usize {
        let base = self.generate_posibale();
        let mut new_con = self.condition.clone();
        let last_rec = *self.record.last().unwrap() as usize;
        let first_rec = self.record[0] as usize;

        if !((self.condition.last().unwrap() == &SpringCondition::Damaged
            && self.condition[self.condition.len() - last_rec] == SpringCondition::Damaged)
            || (self.condition[(self.condition.len() - 1) - (last_rec)] == SpringCondition::Good
                && (1..=last_rec).into_iter().any(|off| {
                    self.condition[self.condition.len() - off] == SpringCondition::Damaged
                })))
        {
            new_con.insert(0, SpringCondition::Unknown);
            // new_con.push(SpringCondition::Unknown);
        }
        if !((self.condition[0] == SpringCondition::Damaged
            && self.condition[first_rec - 1] == SpringCondition::Damaged)
            || (self.condition[first_rec] == SpringCondition::Good
                && (0..first_rec)
                    .into_iter()
                    .any(|off| self.condition[off] == SpringCondition::Damaged)))
        {
            new_con.push(SpringCondition::Unknown);
            // new_con.insert(0, SpringCondition::Unknown);
        }
        // snew_con.push(SpringCondition::Unknown);
        // snew_con.insert(0, SpringCondition::Unknown);
        let extended_spring = SpringRecord {
            condition: new_con,
            record: self.record.clone(),
        };
        base * extended_spring.generate_posibale().pow(4)
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
            pos.push(SpringRecord {
                condition: con,
                record: self.record.clone(),
            })
        }
        pos
    }

    fn rec_valid(&self) -> usize {
        let rec = self.record.iter().map(|&x| x as usize).sum::<usize>();
        let dmg = self
            .condition
            .iter()
            .filter(|item| *item == &SpringCondition::Damaged)
            .count();
        let msng = rec - dmg;
        let pos: Vec<usize> = self
            .condition
            .iter()
            .enumerate()
            .filter(|(_, item)| *item == &SpringCondition::Unknown)
            .map(|(idx, _)| idx)
            .collect();

        // let coll = pos.iter().map(|idx|  )
        loop {
            match self
                .condition
                .iter()
                .position(|con| con == &SpringCondition::Unknown)
            {
                Some(a) => {
                    let mut new = self.condition.clone();
                    new[a] = SpringCondition::Good;
                    let springs_a = SpringRecord {
                        condition: new.clone(),
                        record: self.record.clone(),
                    };
                    new[a] = SpringCondition::Damaged;
                    let springs_b = SpringRecord {
                        condition: new.clone(),
                        record: self.record.clone(),
                    };
                    println!("----------------");
                    println!("good {:?}", springs_a);
                    println!("bad {:?}", springs_b);
                    println!("----------------");
                    // fill next bad
                }
                _ => {
                    break;
                }
            };
        }
        0
    }

    fn simple_gues(&self) -> usize {
        let unk = self
            .condition
            .iter()
            .filter(|item| *item == &SpringCondition::Unknown)
            .count();
        let rec = self.record.iter().map(|&x| x as usize).sum::<usize>();
        let dmg = self
            .condition
            .iter()
            .filter(|item| *item == &SpringCondition::Damaged)
            .count();
        let msng = rec - dmg;
        0
    }

    fn amount_posibale(&self) -> usize {
        let mut pos = 1;
        let con_len = self.condition.len();
        let unk = self
            .condition
            .iter()
            .filter(|item| *item == &SpringCondition::Unknown)
            .count();
        let rec = self.record.iter().map(|&x| x as usize).sum::<usize>();
        let rec_len = self.record.len();
        let dmg = self
            .condition
            .iter()
            .filter(|item| *item == &SpringCondition::Damaged)
            .count();
        let msng = rec - dmg;
        println!(
            "{} unknown with {:?} damaged and missing {}, spaces {}",
            unk,
            rec,
            msng,
            con_len - (rec + (rec_len - 1)),
        );
        let recs: Vec<SpringCondition> = vec![];
        for _ in 0..msng {}
        for item in &self.record {
            let mut multi = 0;
            // println!("atempt to insert {}", item);
            for slice in self.condition.windows(2 + *item as usize) {
                if !(slice.first().unwrap() == &SpringCondition::Damaged)
                    && !(slice.last().unwrap() == &SpringCondition::Damaged)
                {
                    for i in 1..=*item as usize {
                        if slice[i] == SpringCondition::Good {
                            break;
                        }
                    }
                    multi += 1;
                }
            }
            pos += multi;
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
    let contents = std::fs::read_to_string("docs/12/spring.txt").expect("whoops");
    let records: Vec<SpringRecord> = contents
        .replace("....", "...")
        .replace("...", "..")
        .replace("..", ".")
        .lines()
        .map(generate_record)
        .collect();

    let cnt: usize = records
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let x = item.unfold().generate_posibale_v2();
            println!("round {} - item {}", idx, x);
            x
        })
        .sum();

    println!("Total: {}", cnt);
}
