use crate::utils::Solution;
use md5::*;

pub struct Puzzle;

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut i = 1;
        let mut hash = format!("{}{}", input.trim(), i);
        let mut output = compute(hash);

        while &format!("{:x}", output)[0..5] != "00000" {
            i += 1;
            hash = format!("{}{}", input.trim(), i);
            output = compute(hash);
        }

        format!("{}", i)
    }

    fn part2(&self, input: &str) -> String {
        let mut i = 1;
        let mut hash = format!("{}{}", input.trim(), i);
        let mut output = compute(hash);

        while &format!("{:x}", output)[0..6] != "000000" {
            i += 1;
            hash = format!("{}{}", input.trim(), i);
            output = compute(hash);
        }

        format!("{}", i)
    }
}
