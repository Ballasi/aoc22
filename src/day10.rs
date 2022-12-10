const INPUT: &str = include_str!("/tmp/input");

fn part_a() -> i16 {
    let parsed = INPUT.lines().map(|l| {
        if l.starts_with("addx") {
            (2, l.split(' ').nth(1).unwrap().parse::<i16>().unwrap())
        } else {
            (1, 0)
        }
    });

    let mut register = 1;
    let mut sum = 0;
    let mut cycle = 0;

    let add_at = vec![20, 60, 100, 140, 180, 220];
    for (cycle_count, add) in parsed {
        for at in add_at.iter() {
            if cycle < *at && cycle + cycle_count >= *at {
                sum += at * register;
            }
        }
        register += add;
        cycle += cycle_count;
    }

    sum
}

const CRT_WIDTH: i16 = 40;

fn part_b() {
    let parsed = INPUT.lines().map(|l| {
        if l.starts_with("addx") {
            (2, l.split(' ').nth(1).unwrap().parse::<i16>().unwrap())
        } else {
            (1, 0)
        }
    });

    let mut register = 1;
    let mut cycle = 0;

    for (cycle_count, add) in parsed {
        for i in 0..cycle_count {
            if register - 1 <= cycle % 40 && register + 1 >= cycle % 40 {
                print!("█");
            } else {
                print!(" ");
            }
            if i + 1 == cycle_count {
                register += add;
            }
            cycle += 1;
            if cycle % CRT_WIDTH == 0 {
                println!();
            }
        }
    }
}

fn main() {
    println!("Part A: {}", part_a());
    println!("Part B:");
    part_b();
}
