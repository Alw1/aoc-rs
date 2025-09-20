use crate::Solution;
// use fancy_regex::*;
use regex::*;

pub struct Puzzle;

fn parse_input_part1(input: &str) -> Vec<(i32, i32)> {
    let mul_re = Regex::new(r"mul\((?<X>\d+),(?<Y>\d+)\)").unwrap();

    mul_re
        .captures_iter(input)
        .map(|mul| {
            let x = mul["X"].parse::<i32>().unwrap();
            let y = mul["Y"].parse::<i32>().unwrap();
            (x, y)
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Vec<(i32, i32)> {
    let mul_re =
        Regex::new(r"(mul\((?<X>\d+),(?<Y>\d+)\)|(?<do>do\(\))|(?<dont>don't\(\)))").unwrap();
    let mut enabled = true;

    mul_re
        .captures_iter(input)
        .filter_map(|cap| {
            if cap.name("do").is_some() {
                enabled = true;
                None
            } else if cap.name("dont").is_some() {
                enabled = false;
                None
            } else if enabled {
                let x = cap["X"].parse::<i32>().ok()?;
                let y = cap["Y"].parse::<i32>().ok()?;
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}
impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let sum = parse_input_part1(input)
            .iter()
            .fold(0, |acc, mul| acc + (mul.0 * mul.1));

        format!("{}", sum)
    }

    fn part2(&self, input: &str) -> String {
        let sum = parse_input_part2(input)
            .iter()
            .fold(0, |acc, mul| acc + (mul.0 * mul.1));

        format!("{}", sum)
    }
}
