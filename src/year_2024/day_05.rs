use crate::Solution;

pub struct Puzzle;

type Rules = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

fn parse_input(input: &str) -> (Rules, Updates) {
    let parts = input.split_once("\n\n").unwrap();

    let rules: Rules = parts
        .0
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line.split('|').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let updates: Updates = parts
        .1
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_ordered(update: &Vec<u32>, rules: &Rules) -> bool {
    rules.iter().all(|(before, after)| {
        match (
            update.iter().position(|&x| x == *before),
            update.iter().position(|&x| x == *after),
        ) {
            (Some(b_pos), Some(a_pos)) => b_pos < a_pos,
            _ => true,
        }
    })
}

fn reorder_update(update: &Vec<u32>) -> Vec<u32> {
    update.clone()
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);

        // Filter all valid updates, then sum their middle value
        let sum = updates
            .iter()
            .filter(|&update| is_ordered(update, &rules))
            .fold(0, |acc, n| acc + n[n.len() / 2]);

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);

        // Filter all invalid updates, then reorder and sum their middle
        let sum = updates
            .iter()
            .filter(|&update| !is_ordered(update, &rules))
            .fold(0, |acc, update| acc + reorder_update(update)[update.len() / 2]);

        sum.to_string()
    }
}
