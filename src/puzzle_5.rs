use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() -> u64 {
    let filepath = "assets/puzzle5/input.txt";
    let mut sum: u64 = 0;

    let file_lines = read_to_string(filepath).unwrap();

    let mut current_map = "";

    let seed_regex = regex::Regex::new(r"(\d+ \d+)").unwrap();
    let mut seeds = Vec::<u64>::new();

    let map_regex = regex::Regex::new(r"(?<name>\w+)-to").unwrap();
    let mut maps = HashMap::<&str, Vec<(u64, u64, u64)>>::new();

    for line in file_lines.lines() {
        if line.contains("seeds"){
            for m in seed_regex.captures_iter(line){
                let pair_split: Vec<&str> = m.get(0).unwrap().as_str().split(" ").collect();
                let start = pair_split[0].parse::<u64>().unwrap();
                let length = pair_split[1].parse::<u64>().unwrap();
                let mut range: Vec<u64> = (start..start+length).collect();
                seeds.append(&mut range);
                // seeds.push(m.get(0).unwrap().as_str().parse::<u64>().unwrap());
            }
        } else if line.contains("map") {
            current_map = map_regex.captures(line).unwrap().name("name").unwrap().as_str();
            maps.insert(current_map, Vec::<(u64, u64, u64)>::new());
        } else if line == "" {
            //do nothing
        } else {
            //fill current map
            let tuple_vec: Vec<u64> = line.split(" ").map(|el| el.parse::<u64>().unwrap()).collect();
            maps.get_mut(current_map).unwrap().push((tuple_vec[0], tuple_vec[1], tuple_vec[2]));
        }
    }
    let mut smallest = u64::MAX;
    for seed in seeds {
        //println!("Starting at {}", seed);
        let res = trace_seed(seed, &maps);
        //println!("  ended at {}", res);
        if res < smallest {
            smallest = res;
        }
    }
    smallest
}

fn trace_seed(seed: u64, maps: &HashMap<&str, Vec<(u64, u64, u64)>>) -> u64{
    let transitions = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
    let mut search_key = seed;
    for trans in transitions {
        //println!(" through {}", trans);
        match maps.get(trans) {
            Some(ranges) => {
                for values in ranges {
                    if values.1 <= search_key && search_key < values.1 + values.2 {
                        //println!(" within range {:?}", values);
                        search_key = values.0 + (search_key - values.1);
                        break;
                    }
                }
            }
            None => break
        }
        //println!(" to {}", search_key);
    }
    search_key
}
