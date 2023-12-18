use std::time::{SystemTime, UNIX_EPOCH};

mod puzzle_1;
mod puzzle_2;
mod puzzle_3;
mod puzzle_4;
mod puzzle_5;
mod puzzle_6;
mod puzzle_7;
mod puzzle_8;
mod puzzle_9;
mod puzzle_10;
mod puzzle_11;
mod puzzle_12;
mod puzzle_13;
mod puzzle_14;
mod puzzle_15;
mod puzzle_16;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", puzzle_16::solve());
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Executed in {} seconds", (end-start).as_secs());
}
