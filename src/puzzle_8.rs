use regex::Regex;
use std::{
    collections::{hash_map::Keys, HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> u32 {
    let file_path = "assets/puzzle8/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    two(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u32 {
    let mut lines_iter = lines.iter();
    let l_r_instructions = *lines_iter.next().unwrap();
    let mut fake_binary_tree = HashMap::<&str, [&str; 2]>::new();
    lines_iter.next();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines_iter {
        let caps = re.captures(line).unwrap();
        fake_binary_tree.insert(
            caps.get(1).unwrap().as_str(),
            [caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()],
        );
    }
    let mut count: u32 = 0;
    let mut curr_location = "AAA";
    'outer: loop {
        for c in l_r_instructions.chars() {
            count += 1;
            if c == 'L' {
                curr_location = fake_binary_tree.get(curr_location).unwrap()[0];
            } else if c == 'R' {
                curr_location = fake_binary_tree.get(curr_location).unwrap()[1];
            } else {
                unreachable!();
            }
            if curr_location == "ZZZ" {
                break 'outer;
            }
        }
    }
    count
}

fn two(lines: Vec<&str>) -> u32 {
    let mut lines_iter = lines.iter();
    let l_r_instructions = *lines_iter.next().unwrap();
    let mut fake_binary_tree = HashMap::<&str, [&str; 2]>::new();
    lines_iter.next();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines_iter {
        let caps = re.captures(line).unwrap();
        fake_binary_tree.insert(
            caps.get(1).unwrap().as_str(),
            [caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()],
        );
    }

    let mut count: u32 = 0;
    let mut curr_locations = Vec::<&str>::new();

    for k in fake_binary_tree.keys() {
        if k.ends_with('A') {
            curr_locations.push(*k);
        }
    }

    println!("Finding cycles... ");
    let mut starter_cycles = HashMap::<_, _>::new();
    for loc in curr_locations.iter() {
        let cycles = track_path(l_r_instructions, loc, &fake_binary_tree);
        starter_cycles.insert(*loc, cycles);
    }

    let mut periods = Vec::<u64>::new();

    for start in starter_cycles {
        println!("{}", start.0);
        for occ in start.1 {
            if occ.0 .0.ends_with('Z') {
                println!("Stoppable here {:?}", occ);
                periods.push(occ.1[0] as u64);
            }
            if occ.1.len() > 2 {
                println!("Twice {:?}", occ);
            }
        }
    }

    fn gcd(mut x: u64, mut y: u64) -> u64 {
        print!("gcd of {} and {} ", x, y);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        println!("is {}", x );
        x
    }

    fn lcm(x: u64, y: u64) -> u64 {
        x * (y / gcd(x, y))
    }

    count = periods.as_slice().iter().copied().reduce(|acc, el| lcm(acc, el)).unwrap() as u32;

    // 'outer: loop {
    //     for c in l_r_instructions.chars() {
    //         count += 1;
    //         // println!("{:?}", curr_locations);
    //         curr_locations = curr_locations
    //             .iter()
    //             .map(|el| {
    //                 if c == 'L' {
    //                     fake_binary_tree.get(el).unwrap()[0]
    //                 } else if c == 'R' {
    //                     fake_binary_tree.get(el).unwrap()[1]
    //                 } else {
    //                     unreachable!();
    //                 }
    //             })
    //             .collect();

    //         let all_z = curr_locations
    //             .iter()
    //             .fold(true, |acc, x| acc && x.ends_with('Z'));
    //         if all_z {
    //             break 'outer;
    //         }
    //     }
    // }
    count
}

fn track_path<'a>(
    l_r: &'a str,
    start: &'a str,
    tree: &HashMap<&'a str, [&'a str; 2]>,
) -> HashMap<(&'a str, usize), Vec<usize>> {
    let mut count: usize = 0;
    let mut curr_location = start;

    // tracks where this process has been with current node and index within "path"
    let mut visited_states = HashMap::<(&str, usize), Vec<usize>>::new();
    println!("  for {}...", start);
    'finder: loop {
        for (i, c) in l_r.chars().enumerate() {
            count += 1;
            if c == 'L' {
                curr_location = tree.get(curr_location).unwrap()[0];
            } else if c == 'R' {
                curr_location = tree.get(curr_location).unwrap()[1];
            } else {
                unreachable!();
            }

            let state_key = (curr_location, i);

            if let Some(v) = visited_states.get_mut(&state_key) {
                v.push(count);
                if v.len() > 1 {
                    break 'finder;
                }
            } else {
                visited_states.insert(state_key, vec![count]);
            }
        }
    }

    visited_states
}
