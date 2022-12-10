#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("/tmp/input");

// utils
fn priority_from_char(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => unreachable!(),
    }
}

// part one
fn compute_rucksack(rucksack: &str) -> u16 {
    let n = rucksack.len();
    if n == 0 {
        return 0;
    }

    let first_compartment = &rucksack[..n / 2];
    let second_compartment = &rucksack[n / 2..];

    first_compartment
        .chars()
        .find(|c| second_compartment.contains(*c))
        .map(|c| priority_from_char(c) as u16)
        .unwrap()
}

// part two
fn compute_badges(compartments: [&str; 3]) -> u16 {
    compartments[0]
        .chars()
        .find(|c| compartments[1].contains(*c) && compartments[2].contains(*c))
        .map(|c| priority_from_char(c) as u16)
        .unwrap()
}

fn main() {
    println!(
        "Sum of priorities: {}",
        INPUT.split("\n").map(compute_rucksack).sum::<u16>()
    );

    println!(
        "Sum of priorities of badges: {}",
        INPUT
            .lines()
            .array_chunks::<3>()
            .map(compute_badges)
            .sum::<u16>()
    );
}
