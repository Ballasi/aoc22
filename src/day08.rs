// clearly not my best day so far

const INPUT: &str = include_str!("/tmp/input");

fn get(x: usize, y: usize, input: &Vec<&str>) -> Option<u32> {
    input[y].chars().nth(x)?.to_digit(10)
}

fn part_a() -> usize {
    let mut visible: Vec<(usize, usize)> = vec![];

    let input = INPUT.lines().collect::<Vec<&str>>();

    let m = input.first().unwrap().len();
    let n = input.len();

    for x in 1..m - 1 {
        // top to bottom
        let mut size = get(x, 0, &input).unwrap();
        for i in 1..n - 1 {
            let s = get(x, i, &input).unwrap();
            if s > size {
                size = s;
                if !visible.contains(&(x, i)) {
                    visible.push((x, i));
                }
            }
        }

        // bottom to top
        let mut size = get(x, n - 1, &input).unwrap();
        for i in (1..n - 1).rev() {
            let s = get(x, i, &input).unwrap();
            if s > size {
                size = s;
                if !visible.contains(&(x, i)) {
                    visible.push((x, i));
                }
            }
        }
    }

    for y in 1..n - 1 {
        // left to right
        let mut size = get(0, y, &input).unwrap();
        for i in 1..m - 1 {
            let s = get(i, y, &input).unwrap();
            if s > size {
                size = s;
                if !visible.contains(&(i, y)) {
                    visible.push((i, y));
                }
            }
        }

        // bottom to top
        let mut size = get(m - 1, y, &input).unwrap();
        for i in (1..m - 1).rev() {
            let s = get(i, y, &input).unwrap();
            if s > size {
                size = s;
                if !visible.contains(&(i, y)) {
                    visible.push((i, y));
                }
            }
        }
    }

    visible.len() + (m + n) * 2 - 4
}

fn get_score(x: usize, y: usize, m: usize, n: usize, input: &Vec<&str>) -> usize {
    let xysize = get(x, y, &input).unwrap();
    let mut top_score = 0;
    if y > 0 {
        let mut i = y - 1;
        top_score = 1;
        if get(x, i, &input).unwrap() < xysize {
            while i > 0 {
                i -= 1;
                let s = get(x, i, &input).unwrap();
                if s >= xysize {
                    top_score = y - i;
                    break;
                } else {
                    top_score += 1;
                }
            }
        }
    }

    let mut bottom_score = 0;
    if y < n - 1 {
        let mut i = y + 1;
        bottom_score = 1;
        if get(x, i, &input).unwrap() < xysize {
            while i < n - 1 {
                i += 1;
                let s = get(x, i, &input).unwrap();
                if s >= xysize {
                    bottom_score += 1;
                    break;
                } else {
                    bottom_score += 1;
                }
            }
        }
    }

    let mut left_score = 0;
    if x > 0 {
        let mut i = x - 1;
        left_score = 1;
        if get(i, y, &input).unwrap() < xysize {
            while i > 0 {
                i -= 1;
                let s = get(i, y, &input).unwrap();
                if s >= xysize {
                    left_score += 1;
                    break;
                } else {
                    left_score += 1;
                }
            }
        }
    }

    let mut right_score = 0;
    if x < m - 1 {
        let mut i = x + 1;
        right_score = 1;
        if get(i, y, &input).unwrap() < xysize {
            while i < m - 1 {
                i += 1;
                let s = get(i, y, &input).unwrap();
                if s >= xysize {
                    right_score += 1;
                    break;
                } else {
                    right_score += 1;
                }
            }
        }
    }
    top_score * bottom_score * left_score * right_score
}

fn part_b() -> usize {
    let mut scores = vec![];

    let input = INPUT.lines().collect::<Vec<&str>>();

    let m = input.first().unwrap().len();
    let n = input.len();

    for y in 0..n {
        for x in 0..m {
            scores.push(get_score(x, y, m, n, &input));
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    println!("Part A: {}", part_a());
    println!("Part B: {}", part_b());
}
