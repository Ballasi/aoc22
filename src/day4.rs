const INPUT: &str = include_str!("/tmp/input_day4");

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let mut sections = line.split(",");
    let first_section = sections.next().unwrap();
    let second_section = sections.next().unwrap();

    let mut fs = first_section.split("-");
    let mut ss = second_section.split("-");

    let fsl = fs.next().unwrap().parse::<i32>().unwrap();
    let fsr = fs.next().unwrap().parse::<i32>().unwrap();
    let ssl = ss.next().unwrap().parse::<i32>().unwrap();
    let ssr = ss.next().unwrap().parse::<i32>().unwrap();

    (fsl, fsr, ssl, ssr)
}

fn main() {
    println!(
        "Fully contains: {}",
        INPUT
            .lines()
            .filter(|line| {
                let (fsl, fsr, ssl, ssr) = parse_line(line);
                (fsl >= ssl && fsr <= ssr) || (ssl >= fsl && ssr <= fsr)
            })
            .count()
    );

    println!(
        "Any overlap: {}",
        INPUT
            .lines()
            .filter(|line| {
                let (fsl, fsr, ssl, ssr) = parse_line(line);
                (fsl >= ssl && fsl <= ssr)
                    || (fsr >= ssl && fsr <= ssr)
                    || (ssl >= fsl && ssl <= fsr)
                    || (ssr >= fsl && ssr <= fsr)
            })
            .count()
    );
}
