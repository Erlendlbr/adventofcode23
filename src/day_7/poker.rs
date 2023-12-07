use std::collections::HashMap;
use std::{fs, iter, option};

const PART_TWO: bool = true;

struct Hand {
    cards: String,
    bet: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn char_to_num(c: &char) -> u32 {
    match c {
        'T' => 10,
        'J' => {
            if PART_TWO {
                1
            } else {
                11
            }
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 9
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.get_type();
        let b = other.get_type();
        if a == b {
            for item in self.cards.chars().zip(other.cards.chars()).into_iter() {
                if !(item.0 == item.1) {
                    if item.0.is_digit(10) && item.1.is_digit(10) {
                        return item.1.cmp(&item.0);
                    } else if item.0.is_digit(10) {
                        return char_to_num(&item.1).cmp(&item.0.to_digit(10).unwrap());
                        // return std::cmp::Ordering::Greater;
                    } else if item.1.is_digit(10) {
                        return item.1.to_digit(10).unwrap().cmp(&char_to_num(&item.0));
                    } else {
                        return char_to_num(&item.1).cmp(&char_to_num(&item.0));
                    }
                }
            }
            return std::cmp::Ordering::Equal;
            } else {
            a.cmp(&b)
        }
    }
}

impl Hand {
    
    fn get_type(&self) -> HandType {
        let mut char_map = self.cards.chars()
                .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
        // for x in char_map.iter() {
        //     println!(" we have c: {}  times: {}", x.0, x.1);
        // };
        let mut may_need = None;
        if PART_TWO {
            may_need = char_map.remove(&'J');
        }
        let mut som: Vec<(char, u32)>  = char_map.into_iter().map(|item| item ).collect();
        som.sort_by_key(|item| item.1);
        som.reverse();

        println!("som som : {:?}", som);
        if may_need.is_some() {
            if som.is_empty() {
                som.push(('J',5));
            } else {
                som[0].1 += may_need.unwrap();
            }
        }
        
        if som[0].1 == 5 {
            HandType::FiveKind
        } else if som[0].1 ==4 {
            HandType::FourKind
        } else if som[0].1 == 3 {
            if som[1].1 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreKind
            }
        } else if som[0].1 == 2 {
            if som[1].1  == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair 
            }       
        }else {
            HandType::HighCard
        }
    }
}

pub fn parse_hands() {
    let file1 = "docs/7/hand.txt";
    let contents = fs::read_to_string(file1).expect("Should have been able to read the file");
    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|tup| Hand {
                    cards: tup.0.to_string(),
                    bet: tup.1.parse::<u32>().unwrap(),
                })
                .unwrap()
        })
        .collect();

    hands.sort();
    hands.reverse();

    // for (i, hand) in hands.iter().enumerate() {println!(" {} we have {}, with bet {}", i+1, hand.cards, hand.bet);}
    let sum: u32 = hands.iter().enumerate().map(| (i,hand) | (i as u32 + 1) * hand.bet ).sum();
    println!("tada: {}", sum)
}
