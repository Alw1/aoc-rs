use crate::Solution;

pub struct Puzzle;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        left.push(nums[0].parse::<i32>().expect("Failed to parse number"));
        right.push(nums[1].parse::<i32>().expect("Failed to parse number"));
    }

    (left, right)
}

fn count_occurance(num: &i32, list: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for x in list.iter() {
        if *x == *num {
            sum += 1;
        }
    }

    sum
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        // let (mut left, mut right) = parse_input(input);

        let sum = 0;

        format!("{}", sum)
    }

    fn part2(&self, input: &str) -> String {
        let (left, right) = parse_input(input);

        let sum: i32 = left
            .iter()
            .map(|num| num * count_occurance(num, &right))
            .sum();

        format!("{}", sum)
    }
}
