const INPUT: &str = include_str!("/tmp/input");
use std::collections::HashMap;

type Directories = HashMap<String, Dir>;
type TotalSize = HashMap<String, u32>;

#[derive(Eq, Hash, PartialEq)]
struct Dir {
    file_size: u32,
    sub_directories: Vec<String>,
}

fn format_dir(directories: &mut Directories) {
    let mut pwd = vec![];
    for d in INPUT.split("$ cd ") {
        if d.is_empty() {
            continue;
        }
        let curr_dir = d.lines().nth(0).unwrap();
        if curr_dir == ".." {
            pwd.pop();
            continue;
        }
        pwd.push(curr_dir);
        directories.insert(
            pwd.join("/"),
            Dir {
                file_size: d
                    .lines()
                    // get strings starting with numbers
                    .filter(|x| {
                        x.chars().nth(0).unwrap() <= '9' && x.chars().nth(0).unwrap() >= '1'
                    })
                    // parse numbers
                    .map(|x| x.split(' ').nth(0).unwrap().parse::<u32>().unwrap())
                    .sum::<u32>(),
                sub_directories: d
                    .lines()
                    .filter(|x| x.starts_with("dir "))
                    // remove "dir "
                    .map(|x| String::from(&x[4..]))
                    .collect::<Vec<String>>(),
            },
        );
    }
}

fn get_total_size(dir: &str, directories: &Directories) -> u32 {
    let d = directories.get(dir).unwrap();
    d.file_size
        + d.sub_directories
            .iter()
            .map(|x| {
                let mut s = String::from(dir);
                s.push_str("/");
                s.push_str(x);
                get_total_size(&s, directories)
            })
            .sum::<u32>()
}

fn main() {
    let mut directories = Directories::new();
    let mut dir_total = TotalSize::new();

    // format dir
    format_dir(&mut directories);

    // resolve dir
    for l in directories.keys() {
        dir_total.insert(l.to_string(), get_total_size(l, &directories));
    }

    println!(
        "Part A: {}",
        dir_total
            .iter()
            .map(|(_, x)| x)
            .filter(|&&x| x <= 100_000)
            .sum::<u32>()
    );

    let unused = 70000000 - dir_total.get("/").unwrap();
    println!(
        "Part B: {}",
        dir_total
            .iter()
            .map(|(_, x)| x)
            .filter(|&&x| x + unused >= 30_000_000)
            .min()
            .unwrap()
    );
}
