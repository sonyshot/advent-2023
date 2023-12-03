use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() -> u32 {
    let filepath = "assets/puzzle3/input.txt";
    let mut sum: u32 = 0;

    // tuple is line number, start, stop; symbol, start
    let mut nums = HashMap::<(usize, usize, usize), &str>::new();
    let mut syms = HashMap::<(usize, usize), &str>::new();

    let file_text = read_to_string(filepath).unwrap();
    let mut line_counter = 0;
    let mut width = 0;

    for line in file_text.lines() {
        let re = Regex::new(r"(?<num>\d+)|(?<sym>\*|\/|\+|=|#|\$|@|%|&|-)").unwrap();
        width = line.len();
        // println!("{:?}", line);
        for cap in re.captures_iter(line) {
            // println!("{:?}", cap);
            if let Some(numba) = cap.name("num") {
                nums.insert((line_counter, numba.start(), numba.end()), numba.as_str());
            }
            if let Some(sym) = cap.name("sym") {
                syms.insert((line_counter, sym.start()), sym.as_str());
            }
        }
        line_counter += 1;
    }

    let height = line_counter;

    // println!("{:?}", nums);
    // println!("");
    // println!("{:?}", syms);

    part_2(nums, syms, height, width)
}

fn part_1(
    mut nums: HashMap<(usize, usize, usize), &str>,
    syms: HashMap<(usize, usize), &str>,
    height: usize,
    width: usize,
) -> u32 {
    let mut sum = 0;

    'outer: for numba in nums.drain() {
        let (line, start, stop) = numba.0;

        let top = line == 0;
        let bot = line == height - 1;
        let left = start == 0;
        let right = stop == width - 1;

        //i guess we really only care about topness and leftness

        if !left {
            if syms.contains_key(&(line, start - 1)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
            if !top && syms.contains_key(&(line - 1, start - 1)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
            if !bot && syms.contains_key(&(line + 1, start - 1)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
        }
        if !right {
            if syms.contains_key(&(line, stop)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
            if !top && syms.contains_key(&(line - 1, stop)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
            if !bot && syms.contains_key(&(line + 1, stop)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue;
            }
        }

        for idx in start..stop {
            if !top && syms.contains_key(&(line - 1, idx)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue 'outer;
            }
            if !bot && syms.contains_key(&(line + 1, idx)) {
                println!("including {:?}", numba);
                sum += numba.1.parse::<u32>().unwrap();
                continue 'outer;
            }
        }
    }
    sum
}

fn part_2(
    mut nums: HashMap<(usize, usize, usize), &str>,
    syms: HashMap<(usize, usize), &str>,
    height: usize,
    width: usize,
) -> u32 {
    let mut sum = 0;
    let mut star_adjacencies = HashMap::<(usize, usize), Vec<&str>>::new();

    for numba in nums.drain() {
        let (line, start, stop) = numba.0;

        let top = line == 0;
        let bot = line == height - 1;
        let left = start == 0;
        let right = stop == width - 1;

        //i guess we really only care about topness and leftness
        let mut point = (0, 0);

        if !left {
            //is there a symbol adjacent on the left
            point = (line, start - 1);
            p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            //is there a symbol diagonally top-left
            if !top {
                point = (line - 1, start - 1);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
            //is there a symbol diagonally bot-left
            if !bot {
                point = (line + 1, start - 1);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
        }
        if !right {
            //is there a symbol adjacent on the right
            point = (line, stop);
            p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            //is there a symbol diagonally top-right
            if !top {
                point = (line - 1, stop);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
            //is there a symbol diagonally bottom-right
            if !bot {
                point = (line + 1, stop);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
        }

        for idx in start..stop {
            if !top {
                point = (line - 1, idx);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
            if !bot {
                point = (line + 1, idx);
                p2_helper(&mut star_adjacencies, numba.1, point, syms.get(&point));
            }
        }
    }
    for ent in star_adjacencies{
        if ent.1.len() == 2 {
            println!("{:?}", ent);
            sum += ent.1[0].parse::<u32>().unwrap()*ent.1[1].parse::<u32>().unwrap();
        }
    }

    sum
}

fn p2_helper<'a>(adj_map: &mut HashMap<(usize, usize), Vec<&'a str>>, numba: &'a str, point: (usize, usize), sym: Option<&&str>){
    let star = "*";
    if let Some(sym) = sym {
        if sym == &star {
            if let Some(existing_entry) = adj_map.get_mut(&point) {
                existing_entry.push(numba);
            } else {
                adj_map.insert(point, vec![numba]);
            }
        }
    }
}
