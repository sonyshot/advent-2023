use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle2/input.txt";
    let mut sum: u32 = 0;
    let mut schematic =  Vec::<Vec<char>>::new();
    let mut done_indices = HashMap::<(usize, usize), bool>::new();

    let syms = [""];

    for line in read_to_string(filepath).unwrap().lines(){
        let mut curr_line = Vec::<char>::new();
        for c in line.chars(){
            curr_line.push(c);
        }
        schematic.push(curr_line);
    }

    for x in 0..schematic.len() {
        for y in 0..schematic[0].len(){

        }
    }

    sum
}

