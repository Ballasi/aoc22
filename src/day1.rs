use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn max_calories_from_file(path: String) -> std::io::Result<()> {
    let mut calories = vec![];
    let file = BufReader::new(File::open(path)?);
    let mut current_calories = 0;
    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        current_calories += line.parse::<u32>().unwrap();
    }
    calories.sort();
    calories.reverse();
    println!("{}", calories[0] + calories[1] + calories[2]);
    Ok(())
}

fn test(path: String) {
    let file = BufReader::new(File::open(path).unwrap());
    dbg!(file.lines());
    /*let array = file.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let array = array.split(|x| x == "15");
    dbg!(array);*/
}

fn main() {
    let mut args = env::args();
    let caller = args.next().unwrap();
    match args.next() {
        Some(file) => test(file),//max_calories_from_file(file).unwrap(),
        None => println!("Usage: {} <path>", caller),
    }
}
