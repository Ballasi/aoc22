const INPUT: &str = include_str!("/tmp/input");

fn parse_header() -> (Vec<Vec<char>>, usize) {
    let m = INPUT.lines().position(|x| x.is_empty()).unwrap();
    let n = INPUT
        .lines()
        .nth(m - 1)
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .count();

    let mut stacks = vec![];
    for i in 0..n {
        stacks.push(
            INPUT
                .lines()
                .take(m - 1)
                .collect::<Vec<&str>>()
                .iter()
                .rev()
                .map(|x| x.as_bytes()[i * 4 + 1] as char)
                .filter(|&x| x != ' ')
                .collect::<Vec<char>>(),
        );
    }
    (stacks, n)
}

fn main() {
    let (mut stacks, n) = parse_header();
    let moves = INPUT.lines().skip(n + 1);
    for m in moves {
        let mut split = m.split(' ');
        split.next().unwrap();
        let num = split.next().unwrap().parse::<usize>().unwrap();
        split.next().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap();
        split.next().unwrap();
        let to = split.next().unwrap().parse::<usize>().unwrap();
        /*for _ in 0..num {
            let elem = stacks[from-1].pop().unwrap();
            stacks[to-1].push(elem);
        }*/
        let mut middle = vec![];
        for _ in 0..num {
            middle.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..num {
            stacks[to - 1].push(middle.pop().unwrap());
        }
    }
    println!(
        "{}",
        stacks
            .iter()
            .map(|x| *x.last().unwrap())
            .collect::<String>()
    );
}
