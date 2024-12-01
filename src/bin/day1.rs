use std::collections::HashMap;
use std::fs;

const PATH: &str = "data/day1.txt";

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut ids1: Vec<i32> = Vec::new();
    let mut ids2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let ids: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        ids1.push(ids[0]);
        ids2.push(ids[1]);
    }

    (ids1, ids2)
}

fn part1(input: &str) -> i32 {
    let (mut ids1, mut ids2) = parse_input(input);

    ids1.sort();
    ids2.sort();

    ids1.iter()
        .zip(ids2.iter())
        .fold(0, |acc, (id1, id2)| acc + i32::abs(id1 - id2))
}

fn part2(input: &str) -> i32 {
    let (ids1, ids2) = parse_input(input);

    let mut freqs_ids2: HashMap<i32, i32> = HashMap::new();
    ids2.iter().for_each(|&id| {
        freqs_ids2
            .entry(id)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    });

    ids1.iter().fold(0, |acc, val| match freqs_ids2.get(val) {
        Some(freq) => acc + val * freq,
        None => acc,
    })
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
        let input: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

        let answer = part1(input);
        let expected = 11;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_part2() {
        let input: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let answer = part2(input);
        let expected = 31;
        assert_eq!(answer, expected);
    }
}
