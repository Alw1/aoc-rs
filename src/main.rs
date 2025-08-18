use aoc_rust::{benchmark_fn, utils::{build_registry, SolutionRegistry}};
use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Solution Year
   #[arg(short, long)]
    year: u32,

    /// Solution Day
    #[arg(short, long)]
    day: u8,

    // Enable benchmark to display solution runtime
     #[arg(short, long)]
    benchmark: bool
}

fn main() {
    let args = Cli::parse();

    let input = std::fs::read_to_string(format!("inputs/year_{}/day{}.txt", args.year, args.day)).unwrap();

    let solutions: SolutionRegistry = build_registry();

    let solution = match solutions.get(&(args.year, args.day)) {
        Some(x) => x,
        _ => panic!("Solution not found for {}/{}", args.day, args.year)

    };

    println!("Advent of Code {} Solutions:", args.year);
    println!("Part 1: {}", solution.part1(&input));
    println!("Part 2: {}", solution.part2(&input));

    if args.benchmark {
        benchmark_fn!(solution.part1(&input));
        benchmark_fn!(solution.part2(&input));
    }

}

//
// use regex::Regex;
// use std::fs;
//
// fn parse_presents(input: &String) -> Vec<(u32, u32, u32)> {
//     let dimensions_re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
//     let mut presents = Vec::new();
//     for dimension in dimensions_re.captures_iter(input) {
//         let length: u32 = dimension[1].parse().unwrap();
//         let width: u32 = dimension[2].parse().unwrap();
//         let height: u32 = dimension[3].parse().unwrap();
//         presents.push((length, width, height));
//     }
//     return presents
// }
//
// fn main() {
//     let input = fs::read_to_string("input.txt").expect("ERROR: File not read");
//     part1(&input);
//     part2(&input);
// }
//
// fn part1(input: &String) {
//     let presents = parse_presents(&input);
//     let total: u32 = presents
//         .iter()
//         .map(|(l, w, h)| {
//             let sides = [l * w, w * h, h * l];
//             2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap()
//         })
//         .sum();
//     println!("Part 1: {}", total);
// }
//
// fn part2(input: &String) {
//     todo!();
// }
