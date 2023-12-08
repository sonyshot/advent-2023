use regex::Regex;
use std::cmp::{Eq, Ordering, PartialEq};
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() -> u32 {
    let file_path = "assets/puzzle7/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let mut sum = 0;

    let mut hands = Vec::<Hand>::new();

    let card_ordering = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('J', -1),
        ('T', 8),
        ('9', 7),
        ('8', 6),
        ('7', 5),
        ('6', 4),
        ('5', 3),
        ('4', 2),
        ('3', 1),
        ('2', 0),
    ]);

    for line in file_lines.lines() {
        hands.push(Hand::new(line));
    }

    hands.sort_by(|a, b| {
        let mut res = Ordering::Equal;
        if a.classify() < b.classify() {
            res = Ordering::Less;
        } else if a.classify() > b.classify() {
            res = Ordering::Greater;
        } else {
            let mut a_chars = a.cards.chars();
            let mut b_chars = b.cards.chars();
            for _ in 0..5 {
                let a_ord = card_ordering.get(&a_chars.next().unwrap()).unwrap();
                let b_ord = card_ordering.get(&b_chars.next().unwrap()).unwrap();
                // println!("  |  a value is {} - b value is {}", a, b, a_ord, b_ord);
                if a_ord < b_ord {
                    res = Ordering::Less;
                    break;
                } else if a_ord > b_ord {
                    res = Ordering::Greater;
                    break;
                }
            }
        }
        // println!("comparing {:?} to {:?} | Result is {:?}", a, b, res);
        return res;
    });

    println!("{:?}", hands);

    for (i, hand) in hands.iter().enumerate() {
        println!("{:?} is rank {} with type {} thus adding {}", hand, i+1, hand.classify(), (i as u32 + 1) * hand.bid,);
        sum += (i as u32 + 1) * hand.bid;
    }

    sum
}

#[derive(Debug, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

enum HandType {
    Fiver,
    Fourer,
    Fuller,
    Threeer,
    Twoer,
    Oneer,
    Noner,
}

impl Hand<'_> {
    pub fn new(file_line: &str) -> Hand<'_> {
        let re = Regex::new(r"(\w+)\s(\d+)").unwrap();
        let cards = re.captures(file_line).unwrap().get(1).unwrap().as_str();
        let bid = re
            .captures(file_line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        Hand { cards, bid }
    }

    // I would've liked to use HandType, but there are so many steps to apply ordering to them
    fn classify(&self) -> u32 {
        let mut char_count = HashMap::<char, u32>::new();
        for c in self.cards.chars() {
            if let Some(count) = char_count.get_mut(&c) {
                *count += 1;
            } else {
                char_count.insert(c, 1);
            }
        }
        let j_count = char_count.remove(&'J');
        if let Some(x) = j_count {
            char_count.insert('J', x);
        }
        match char_count.len() {
            4 => {
                if let Some(_) = j_count {
                    // only 1 or 2 Js possible, both cases turn one pair into three of a kind
                    return 3;
                } else {
                    return 1; // one pair
                }
            }
            3 => {
                for ent in char_count {
                    if ent.1 == 3 {
                        if let Some(_) = j_count {
                            // 3 of a kind hand becomes 4 of a kind if J is one of the sets
                            return 5;
                        } else {
                            return 3; // three of a kind
                        }
                    }
                }
                if let Some(jc) = j_count {
                    if jc == 2 {
                        // 2 js in a two pair becomes 4 of a kind
                        return 5;
                    } else {
                        // 1 j in a two pair becomes full house
                        return 4;
                    }
                } else {
                    return 2; // two pairs
                }
            }
            2 => {
                if let Some(_) = j_count {
                    // both 4 of a kind and full house become 5 of a kind
                    return 6;
                } else {
                    let an_entry = char_count.iter().next().unwrap();
                    if an_entry.1 == &4 || an_entry.1 == &1 {
                        return 5; // four of a kind
                    } else {
                        return 4; //full house
                    }
                }
            }
            1 => {
                return 6; // 5 of a kind
            }
            _ => {
                if let Some(_) = j_count {
                    // one card J? promote to one pair
                    return 1;
                } else {
                    return 0; // High Card
                }
            }
        }
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sclass = self.classify();
        let oclass = other.classify();
        self.classify().cmp(&other.classify())
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.bid.partial_cmp(&other.bid)
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}
