use std::{fs::read_to_string, collections::HashMap, cmp::Ordering};


pub fn solve() -> u32 {
    let filepath = "assets/puzzle1/input.txt";
    let mut sum: u32 = 0;

    for line in read_to_string(filepath).unwrap().lines(){
        sum += named_numbers(&line);
    }
    sum
}

fn named_numbers(file_line: &str) -> u32{
    let mut firsts = Vec::new();
    let mut lasts = Vec::new();

    let number_strings = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
    ];

    let number_strings_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for spelled_num in number_strings {
        firsts.push((spelled_num, file_line.find(spelled_num)));
        lasts.push((spelled_num, file_line.rfind(spelled_num)));
    }

    firsts.sort_by(|a, b| {
        if let Some(pos_a) = a.1 {
            if let Some(pos_b) = b.1 {
                pos_a.cmp(&pos_b)
            } else {
                Ordering::Less
            }
        } else {
            if let Some(_) = b.1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });

    lasts.sort_by(|b, a| {
        if let Some(pos_a) = a.1 {
            if let Some(pos_b) = b.1 {
                pos_a.cmp(&pos_b)
            } else {
                Ordering::Greater
            }
        } else {
            if let Some(_) = b.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    });

    let digit1 = number_strings_map.get(firsts[0].0).unwrap_or(&firsts[0].0).to_string().parse::<u32>().unwrap();
    let digit2 = number_strings_map.get(lasts[0].0).unwrap_or(&lasts[0].0).to_string().parse::<u32>().unwrap();

    digit1*10+digit2
}

// fn extract_number(file_line: &str) -> u32 {
//     let mut first: Option<u32> = None;
//     let mut last: Option<u32> = None;
//     let processed_str = preprocess_line(file_line);
//     for c in processed_str.chars() {
//         if let Ok(digit) = c.to_string().parse::<u32>() {
//             first = Some(digit);
//             break;
//         }
//     }
//     for c in processed_str.chars().rev() {
//         if let Ok(digit) = c.to_string().parse::<u32>() {
//             last = Some(digit);
//             break;
//         }
//     }
//     let val = first.unwrap()*10+last.unwrap();
//     val
// }