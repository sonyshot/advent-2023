use std::fs::read_to_string;

pub fn solve() -> u32 {
    let filepath = "assets/puzzle2/input.txt";
    let mut sum: u32 = 0;

    let total_colors = (12, 13, 14);

    let input_string = read_to_string(filepath).unwrap();

    let all_games = input_string.lines().map(|el| parse_game(el));

    sum = all_games.map(|el| {println!("{:?}", el);el.min_colors.0*el.min_colors.1*el.min_colors.2}).sum();

    sum
}

fn valid_game(true_colors: (u32, u32, u32), game: GameConfiguration) -> bool{
    for round in game.rounds {
        if round.0 > true_colors.0 || round.1 > true_colors.1 || round.2 > true_colors.2 {
            return false;
        }
    }
    return true;
}

fn parse_game(file_line: &str) -> GameConfiguration {
    let mut split_line = file_line.split(':');
    let identifier_string = split_line.next().unwrap();
    let games_string = split_line.next().unwrap();

    let id = identifier_string
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // println!("{:?}", id);
    let parsed_games = parse_record(games_string);

    let mut min_colors = (0, 0, 0);

    for round in parsed_games.clone() {
        if round.0 > min_colors.0 {
            min_colors.0 = round.0;
        }
        if round.1 > min_colors.1 {
            min_colors.1 = round.1;
        }
        if round.2 > min_colors.2 {
            min_colors.2 = round.2;
        }
    }

    GameConfiguration {
        id,
        rounds: parsed_games,
        min_colors
    }
}

fn parse_record(game_record: &str) -> Vec<(u32, u32, u32)> {
    //returns rgb in that order
    // println!("");
    let games = game_record.split(";");
    let mut output = Vec::<(u32, u32, u32)>::new();

    for round in games {
        let mut round_colors = (0, 0, 0);
        let counts = round.split(',');
        // println!("{:?}", counts);

        for count in counts {
            let val = count.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
            // println!("{}", val);
            if count.contains("red") {
                round_colors.0 = val;
            } else if count.contains("green") {
                round_colors.1 = val;
            } else if count.contains("blue") {
                round_colors.2 = val;
            }
        }
        output.push(round_colors);
    }

    output
}

#[derive(Debug, Clone)]
struct GameConfiguration {
    id: u32,
    rounds: Vec<(u32, u32, u32)>,
    min_colors: (u32, u32, u32)
}

