#[derive(PartialEq)]
enum Operation {
    Equal(u32),
    Dash,
}

struct AdvHashMap {
    tag: String,
    operation: Operation,
    box_id: u32,
}

pub fn display() {
    let contents = std::fs::read_to_string("docs/15/seq.txt").expect("whoops");
    let pre_hash: Vec<&str> = contents.split(',').collect();
    // for item in pre_hash {
    //     let mut running = 0;
    //     for c in item.chars() {
    //         let c_int = c as u32;
    //         running += c_int;
    //         running *= 17;
    //         running = running % 256;
    //         println!("char: {}, asci value {}", c, c as u32)
    //     }
    //     println!("after hash {}", running);
    //     println!();
    //     println!("fn: {}", hash_1(item));
    // }

    let adv_hash: Vec<AdvHashMap> = pre_hash
        .iter()
        .map(|line| generate_HASHMAP(*line))
        .collect();

    let sum: u32 = pre_hash
        .into_iter()
        .map(hash_1)
        .inspect(|item| println!("has: {}", item))
        .sum();
    println!("foo {}", sum);
    for item in &adv_hash {
        println!("box: {}", item.box_id);
    }

    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for item in adv_hash.into_iter() {
        let val = match item.operation {
            Operation::Equal(x) => Some(x),
            _ => None,
        };

        let loc = boxes[item.box_id as usize]
            .iter()
            .enumerate()
            .filter(|(_, (tag, _))| tag.to_owned() == item.tag)
            .map(|(idx, _)| idx)
            .next();
        if loc.is_some() {
            match item.operation {
                Operation::Dash => {
                    boxes[item.box_id as usize].remove(loc.unwrap());
                }
                Operation::Equal(x) => {
                    boxes[item.box_id as usize][loc.unwrap()] = (item.tag, x);
                }
            }
            // println!("some");
        } else {
            if val.is_some() {
                boxes[item.box_id as usize].push((item.tag, val.unwrap()));
            }
            // println!("none");
        }
    }
    let sum: u32 = boxes
        .into_iter()
        .enumerate()
        .map(|(outer_idx, vec)| {
            vec.into_iter()
                .enumerate()
                .map(|(idx, (_, val))| (outer_idx as u32 + 1) * (idx as u32 + 1) * val)
                .sum::<u32>()
        })
        .sum();
    // println!("box 0 {:?}", boxes[0]);
    // println!("box 3 {:?}", boxes[3]);
    println!("sum {}", sum);
}

fn hash_1(line: &str) -> u32 {
    line.chars().fold(0, |mut accum, c| {
        accum += c as u32;
        accum *= 17;
        accum % 256
    })
}

fn generate_HASHMAP(seq: &str) -> AdvHashMap {
    if seq.ends_with("-") {
        let tag_str = seq.split_once("-").expect("whhoops").0;
        return AdvHashMap {
            tag: tag_str.to_owned(),
            operation: Operation::Dash,
            box_id: hash_1(tag_str),
        };
    }
    let (tag_str, lens_str) = seq.split_once("=").expect("whoops");
    AdvHashMap {
        tag: tag_str.to_owned(),
        operation: Operation::Equal(lens_str.parse::<u32>().unwrap()),
        box_id: hash_1(tag_str),
    }
}
