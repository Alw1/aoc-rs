use crate::Solution;
use fancy_regex::*;

pub struct Puzzle;

type Point = (u32, u32);
type Range = (Point, Point);

#[derive(Debug)]
enum Instruction {
    On(Range),
    Off(Range),
    Toggle(Range),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let instr_pattern = r"^(?P<instruction>turn on|turn off|toggle)\s+(?P<start_point>\d+,\d+)\s+through\s+(?P<end_point>\d+,\d+)";

    let instr_re = Regex::new(instr_pattern).unwrap();
    //
    // instr_re.captures_iter(input).map(|instr| {
    //
    //     if let Some(instr) 
    //     let start: Point = &instr["start_point"];
    //     let end: Point = &instr["end_point"];
    //
    // });
    //
    // Capture groups (instruction, start, end)

    // let test_pattern = r"(turn on|turn off|toggle)

    let instructions = Vec::new();

    instructions
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let grid = [[false; 1000]; 1000];

        let instructions = parse_input(input);

        print!("{:?}", instructions);

        todo!();
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
