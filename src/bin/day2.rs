use std::fs;

const PATH: &str = "data/day2.txt";

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        reports.push(levels);
    }

    reports
}

fn is_safe(report: &[i32]) -> bool {
    let sign = report[0] < report[1];
    report.windows(2).all(|levels| {
        let (l0, l1) = (levels[0], levels[1]);

        (l1 - l0).abs() > 0 && (l1 - l0).abs() <= 3 && (l0 < l1) == sign
    })
}

fn part1(input: &str) -> usize {
    let reports = parse_input(input);
    reports.iter().filter(|&report| is_safe(report)).count()
}

fn part2(input: &str) -> usize {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|&report| {
            let dampened =
                (0..report.len()).any(|i| is_safe(&[&report[..i], &report[i + 1..]].concat()));

            is_safe(report) || dampened
        })
        .count()
}

fn main() {
    let input = fs::read_to_string(PATH).unwrap();
    let answer_part1 = part1(&input);
    let answer_part2 = part2(&input);

    println!("Part 1: {answer_part1}");
    println!("Part 2: {answer_part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        let answer = part1(input);
        let expected = 2;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_part2() {
        let input: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let answer = part2(input);
        let expected = 4;
        assert_eq!(answer, expected);
    }
}
