const INPUT: &str = include_str!("/tmp/input");
const MAX_ARRAY: usize = 10;

fn visited(tail_size: usize) -> usize {
    let mut visited = vec![(0, 0)];

    let mut rope = [(0, 0); MAX_ARRAY];

    for line in INPUT.lines() {
        let direction = line.split(' ').nth(0).unwrap();
        let count = line.split(' ').nth(1).unwrap().parse::<i16>().unwrap();

        for _ in 0..count {
            rope[0] = match direction {
                "U" => (rope[0].0, rope[0].1 + 1),
                "D" => (rope[0].0, rope[0].1 - 1),
                "L" => (rope[0].0 - 1, rope[0].1),
                "R" => (rope[0].0 + 1, rope[0].1),
                _ => unreachable!(),
            };

            for i in 0..tail_size {
                if (rope[i].0 - rope[i + 1].0 as i16).abs() > 1
                    && (rope[i].1 - rope[i + 1].1 as i16).abs() > 1
                {
                    rope[i + 1] = (
                        if rope[i].0 > rope[i + 1].0 {
                            rope[i + 1].0 + 1
                        } else {
                            rope[i + 1].0 - 1
                        },
                        if rope[i].1 > rope[i + 1].1 {
                            rope[i + 1].1 + 1
                        } else {
                            rope[i + 1].1 - 1
                        },
                    );
                } else if rope[i].0 - rope[i + 1].0 < -1 {
                    rope[i + 1].1 = rope[i].1;
                    rope[i + 1].0 = rope[i].0 + 1;
                } else if rope[i].0 - rope[i + 1].0 > 1 {
                    rope[i + 1].1 = rope[i].1;
                    rope[i + 1].0 = rope[i].0 - 1;
                } else if rope[i].1 - rope[i + 1].1 < -1 {
                    rope[i + 1].0 = rope[i].0;
                    rope[i + 1].1 = rope[i].1 + 1;
                } else if rope[i].1 - rope[i + 1].1 > 1 {
                    rope[i + 1].0 = rope[i].0;
                    rope[i + 1].1 = rope[i].1 - 1;
                }

                if i + 1 == tail_size && !visited.contains(&rope[i + 1]) {
                    visited.push(rope[i + 1]);
                }
            }
        }
    }

    visited.iter().count()
}

fn main() {
    println!("Part A: {}", visited(1));
    println!("Part B: {}", visited(9));
}
