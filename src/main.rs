use aoc_rust::{
    benchmark_fn,
    utils::{Cli, Commands, build_registry, fetch_input},
};
use clap::Parser;
use std::fs;
use std::time::Instant;

fn main() {
    let cli = Cli::parse();

    let solutions = build_registry();

    match &cli.command {
        Commands::Run { date } => {
            let input = fetch_input(&date);
            let solution = solutions.get(&date).unwrap();

            println!("Advent of Code {} Solutions:", date.0);
            println!("Part 1: {}", solution.part1(&input));
            println!("Part 2: {}", solution.part2(&input));
        }
        Commands::Test { date, test } => {
            let input = fs::read_to_string(test).unwrap();
            let solution = solutions.get(&date).unwrap();

            println!("Advent of Code {} Solutions:", date.0);
            println!("Part 1: {}", solution.part1(&input));
            println!("Part 2: {}", solution.part2(&input));
        }
        Commands::Time { date } => {
            let input = fetch_input(&date);
            let solution = solutions.get(&date).unwrap();

            benchmark_fn!(solution.part1(&input));
            benchmark_fn!(solution.part2(&input));
        }
        Commands::List { year } => {
            todo!();
        }
    };
}
