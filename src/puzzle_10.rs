use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

pub fn solve() -> u64 {
    let file_path = "assets/puzzle10/input.txt";
    let file_lines = read_to_string(file_path).unwrap();
    let file_lines_lines: Vec<&str> = file_lines.lines().collect();
    one(file_lines_lines)
}

fn one(lines: Vec<&str>) -> u64 {
    let mut sum = 0;

    let width = lines[0].len();
    let height = lines.len();

    let mut map = vec![vec![Tile::None; width]; height];
    let mut start_point = (0, 0);

    // key is direction of motion onto tile X and value is corresponding next step
    let directionality = HashMap::<(Direction, Tile), (i32, i32, Direction)>::from([
        ((Direction::Left, Tile::DownRight), (1, 0, Direction::Down)),
        ((Direction::Left, Tile::UpRight), (-1, 0, Direction::Up)),
        ((Direction::Left, Tile::LeftRight), (0, -1, Direction::Left)),
        ((Direction::Right, Tile::DownLeft), (1, 0, Direction::Down)),
        ((Direction::Right, Tile::UpLeft), (-1, 0, Direction::Up)),
        (
            (Direction::Right, Tile::LeftRight),
            (0, 1, Direction::Right),
        ),
        ((Direction::Up, Tile::DownLeft), (0, -1, Direction::Left)),
        ((Direction::Up, Tile::DownRight), (0, 1, Direction::Right)),
        ((Direction::Up, Tile::UpDown), (-1, 0, Direction::Up)),
        ((Direction::Down, Tile::UpLeft), (0, -1, Direction::Left)),
        ((Direction::Down, Tile::UpRight), (0, 1, Direction::Right)),
        ((Direction::Down, Tile::UpDown), (1, 0, Direction::Down)),
    ]);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '│' => map[i][j] = Tile::UpDown,
                '┘' => map[i][j] = Tile::UpLeft,
                '┌' => map[i][j] = Tile::DownRight,
                '┐' => map[i][j] = Tile::DownLeft,
                '└' => map[i][j] = Tile::UpRight,
                '─' => map[i][j] = Tile::LeftRight,
                'S' => {
                    map[i][j] = Tile::Start;
                    start_point = (i, j)
                }
                _ => map[i][j] = Tile::None,
            }
        }
    }

    let mut curr_index: (usize, usize) = start_point;
    let mut curr_dir: Direction = Direction::Up;

    let from_start = vec![
        (Direction::Up, map[curr_index.0 - 1][curr_index.1]),
        (Direction::Down, map[curr_index.0 + 1][curr_index.1]),
        (Direction::Left, map[curr_index.0][curr_index.1 - 1]),
        (Direction::Right, map[curr_index.0][curr_index.1 + 1]),
    ];

    for test in from_start {
        if let Some(thing) = directionality.get(&test) {
            sum += 1;
            curr_index = (
                (curr_index.0 as i32 + thing.0) as usize,
                (curr_index.1 as i32 + thing.1) as usize,
            ); //weeee casting
            curr_dir = thing.2;
            break;
        }
    }

    loop {
        sum += 1;
        if map[curr_index.0][curr_index.1] == Tile::Start {
            break;
        }
        if let Some(thing) = directionality.get(&(curr_dir, map[curr_index.0][curr_index.1])) {
            curr_index = (
                (curr_index.0 as i32 + thing.0) as usize,
                (curr_index.1 as i32 + thing.1) as usize,
            ); //weeee casting
            curr_dir = thing.2;
        } else {
            println!("Woopsie");
            println!(
                "{:?} {:?} {:?}",
                curr_dir, curr_index, map[curr_index.0][curr_index.1]
            );
            break;
        }
    }

    sum / 2
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Tile {
    UpLeft,
    UpRight,
    UpDown,
    DownLeft,
    DownRight,
    LeftRight,
    Start,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
