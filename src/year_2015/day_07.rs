#![allow(warnings)]
use crate::Solution;
use std::collections::HashMap;

pub struct Puzzle;

enum Tokens {
    And,
    Or,
    Not,
    LShift,
    RShift,
    Wire(String),
    Number(u16),
    Arrow,
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        // let wires: HashMap<Tokens::Wire> = HashMap::new();
        todo!();

   }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
