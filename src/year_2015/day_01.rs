use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut floor = 0;

        for x in input.chars() {
            if x == '(' {
                floor += 1;
            } else if x == ')' {
                floor -= 1;
            }
        }

        format!("{}", floor)
    }

    fn part2(&self, input: &str) -> String {
        let mut floor = 0;
        let mut basement = 0;

        for (i, x) in input.chars().enumerate() {
            if x == '(' {
                floor += 1;
            } else if x == ')' {
                floor -= 1;
            }

            if floor == -1 {
                basement = i + 1;
                break;
            }
        }

        format!("{}", basement)
    }
}
