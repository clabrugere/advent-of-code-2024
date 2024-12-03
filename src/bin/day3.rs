use regex::Regex;
use std::fs;

const PATH: &str = "data/day3.txt";

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|captures| {
            let (_, [a, b]) = captures.extract();

            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .fold(0, |acc, (a, b)| acc + a * b)
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(
        r"(?<on>do\(\))|(?<off>don't\(\))|(?<op>mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\))",
    )
    .unwrap();

    let mut on = true;
    let mut answer = 0;

    for captures in re.captures_iter(input) {
        if on && captures.name("off").is_some() {
            on = false;
        } else if !on && captures.name("on").is_some() {
            on = true;
        } else if let (Some(num1), Some(num2)) = (captures.name("num1"), captures.name("num2")) {
            if on {
                let num1 = num1.as_str().parse::<i32>().unwrap();
                let num2 = num2.as_str().parse::<i32>().unwrap();

                answer += num1 * num2;
            }
        }
    }

    answer
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
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let answer = part1(input);
        let expected = 161;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_part2() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let answer = part2(input);
        let expected = 48;
        assert_eq!(answer, expected);
    }
}
