use crate::Solution;
use std::cmp;

pub struct Puzzle;

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn smallest_side(&self) -> u32 {
        let sides = [self.l * self.w, self.w * self.h, self.h * self.l];
        *sides.iter().min().unwrap()
    }

    fn calculate_wrapping_paper(&self) -> u32 {
        (2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l) + self.smallest_side()
    }

    fn calculate_ribbon_length(&self) -> u32 {
        let a = cmp::max(
            cmp::min(self.l, self.w),
            cmp::min(cmp::max(self.l, self.w), self.h),
        );
        let b = cmp::min(cmp::min(self.l, self.w), self.h);

        a + a + b + b + self.volume()
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut presents = Vec::new();

        for line in input.lines() {
            let dimensions: Vec<u32> = line
                .split('x')
                .map(|s| s.parse().expect("Invalid dimension"))
                .collect();

            presents.push(Present {
                l: dimensions[0],
                w: dimensions[1],
                h: dimensions[2],
            });
        }

        let sum = presents
            .iter()
            .fold(0, |acc, x| x.calculate_wrapping_paper() + acc);

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut presents = Vec::new();

        for line in input.lines() {
            let dimensions: Vec<u32> = line
                .split('x')
                .map(|s| s.parse().expect("Invalid dimension"))
                .collect();

            presents.push(Present {
                l: dimensions[0],
                w: dimensions[1],
                h: dimensions[2],
            });
        }

        let sum = presents
            .iter()
            .fold(0, |acc, x| x.calculate_ribbon_length() + acc);

        sum.to_string()
    }
}
