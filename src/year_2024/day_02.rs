use crate::Solution;

pub struct Puzzle;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(report: &Vec<u32>) -> bool {
    let increasing = if report[0] > report[1] { true } else { false };

    for window in report.windows(2) {
        let diff: i32 = (window[0] as i32 - window[1] as i32).abs();

        if diff > 3 || diff < 1 {
            return false;
        }

        if increasing && (window[0] < window[1]) {
            return false;
        }

        if !increasing && (window[0] > window[1]) {
            return false;
        }
    }
    true
}

fn is_valid_dampened(report: &Vec<u32>) -> bool {
    if is_valid(report) {
        return true;
    }
    
    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_valid(&modified) {
            return true;
        }
    }
    false
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let reports = parse_input(input);
        let num_valid = reports.iter().filter(|&report| is_valid(&report)).count();

        format!("{:?}", num_valid)
    }

    fn part2(&self, input: &str) -> String {
        let reports = parse_input(input);
        let num_valid = reports.iter().filter(|&report| is_valid_dampened(&report)).count();

        format!("{:?}", num_valid)
    }
}
