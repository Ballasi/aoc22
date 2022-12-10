const INPUT: &str = include_str!("/tmp/input");

fn main() {
    for (m, x) in [("A", 4), ("B", 14)] {
        println!(
            "Part {}: {}",
            m,
            INPUT
                .as_bytes()
                .windows(x)
                .position(|s| (0..x).all(|i| s.iter().filter(|&a| a == &s[i]).count() == 1))
                .unwrap()
                + x
        );
    }
}
