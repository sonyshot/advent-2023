use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() -> u32 {
    let filepath = "assets/puzzle4/input.txt";
    let mut sum: u32 = 0;

    let file_text = read_to_string(filepath).unwrap();
    let mut scratchers = Vec::<ScratchCard>::new();
    let mut counts = Vec::<u32>::new();
    let max_lookback = 10;

    //should try moving this into a fold call 
    for (i, line) in file_text.lines().enumerate() {
        let scratch = ScratchCard::new(line);
        let mut curr_count = 1;
        let start_index = scratchers.len().saturating_sub(max_lookback);
        println!("from {} to {}", start_index, i);
        for (j, scr) in scratchers[start_index..i].iter().enumerate(){
            println!("{} has inf {}", scr.id, scr.influence());
            if scr.influence() >= (i+1) as u32 {
                println!("{} contributes {}", scr.id, counts[j+start_index]);
                curr_count += counts[j+start_index]
            }
        }
        // sum += scratch.score();
        println!("{} has count {}",i+1,curr_count);
        println!("");
        counts.push(curr_count);
        scratchers.push(scratch);
    }
    // scratchers.iter().fold(&[], ||);

    println!("{:?}", counts);
    sum = counts.iter().sum();

    sum
}

fn count_copies(scratchers: &[ScratchCard], prev_count: &[u32]) -> u32{
    let mut output = 0;

    //max lookback is 10 because that's the max matches that can occur on a single card
    let max_lookback = 10;
    

    output
}

struct ScratchCard {
    id: u32,
    winning_nums: HashMap<u32, usize>, //number, index
    my_nums: HashMap<u32, usize>,      //number, index
}

impl ScratchCard {
    pub fn new(line: &str) -> Self {
        let re1 = Regex::new(r"Card\s*(?<id>\d+)").unwrap();
        let re2 = Regex::new(r"(?<num>\d+)").unwrap();

        let mut full_split = line.split(":");
        let id_string = full_split.next().unwrap();
        let mut num_split = full_split.next().unwrap().split("|");
        let winners_string = num_split.next().unwrap();
        let my_string = num_split.next().unwrap();

        let mut id = 0;

        for cap in re1.captures_iter(id_string) {
            id = cap.name("id").unwrap().as_str().parse::<u32>().unwrap();
        }
        let mut winning_nums = HashMap::<u32, usize>::new();
        for (i, cap2) in re2.captures_iter(winners_string).enumerate() {
            winning_nums.insert(
                cap2.name("num").unwrap().as_str().parse::<u32>().unwrap(),
                i,
            );
        }
        let mut my_nums = HashMap::<u32, usize>::new();
        for (j, cap3) in re2.captures_iter(my_string).enumerate() {
            my_nums.insert(
                cap3.name("num").unwrap().as_str().parse::<u32>().unwrap(),
                j,
            );
        }
        ScratchCard {
            id,
            winning_nums,
            my_nums,
        }
    }

    fn influence(&self) -> u32 {
        self.id + self.matches()
    }

    fn matches(&self) -> u32 {
        let mut count = 0;
        for mine in &self.my_nums {
            if self.winning_nums.contains_key(&mine.0) {
                count += 1;
            }
        }
        count
    }

    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        }
    }
}
