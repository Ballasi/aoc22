use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn weight_of_line_p1(line: String) -> i32 {
    let opponent = line.chars().nth(0).unwrap();
    let player = line.chars().nth(2).unwrap();

    let opponent_score = match opponent {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!(""),
    };

    let player_score = match player {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(""),
    };

    let win_score = ((player_score - opponent_score + 1 as i32).rem_euclid(3)) * 3;

    win_score + player_score
}

fn weight_of_line_p2(line: String) -> i32 {
    let opponent = line.chars().nth(0).unwrap();
    let player = line.chars().nth(2).unwrap();

    let opponent_score = match opponent {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!(""),
    };

    let win_score = match player {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(""),
    };

    let player_score = (opponent_score + win_score - 3 as i32).rem_euclid(3) + 1;

    (win_score - 1) * 3 + player_score
}


fn main() {
    let mut args = env::args();
    let caller = args.next().unwrap();
    match args.next() {
        Some(file) => {
            println!(
                "{}",
                BufReader::new(File::open(file).unwrap())
                    .lines()
                    .map(|x| weight_of_line_p2(x.unwrap()))
                    .sum::<i32>()
            );
        }
        None => println!("Usage: {} <path>", caller),
    }
}
