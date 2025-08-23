use crate::Solution;
use fancy_regex::*;

pub struct Puzzle;

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let pattern = r"^(?=(?:[^aeiou]*[aeiou]){3,})(?=.*(.)\1)(?!.*(ab|cd|pq|xy)).*$";
        let re = Regex::new(pattern).unwrap();

        for line in input.lines() {
            println!("{}", line);
        }

        let count = input
            .lines()
            .filter(|string| re.is_match(string).unwrap_or(true))
            .count();

        format!("{}", count)
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
