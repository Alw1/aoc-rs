use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

type Date = (u32, u8);
pub type SolutionRegistry = HashMap<Date, Box<dyn Solution>>;

#[macro_export]
macro_rules! benchmark_fn {
    ($func_call:expr) => {{
        let start_time = Instant::now();
        let result = $func_call;
        let elapsed_time = start_time.elapsed();
        println!("{:?} executed in: {:?}", $func_call, elapsed_time);
        result
    }};
}

#[macro_export]
macro_rules! register_year {
    ($map:ident, $module:ident, $year:expr) => {
        use crate::$module;
        $map.insert(($year, 1), Box::new($module::day_01::Puzzle));
        $map.insert(($year, 2), Box::new($module::day_02::Puzzle));
        $map.insert(($year, 3), Box::new($module::day_03::Puzzle));
        // $map.insert(($year, 4), Box::new($module::day_04::Puzzle));
        // $map.insert(($year, 5), Box::new($module::day_05::Puzzle));
        // $map.insert(($year, 6), Box::new($module::day_06::Puzzle));
        // $map.insert(($year, 7), Box::new($module::day_07::Puzzle));
        // $map.insert(($year, 8), Box::new($module::day_08::Puzzle));
        // $map.insert(($year, 9), Box::new($module::day_09::Puzzle));
        // $map.insert(($year,10), Box::new($module::day_10::Puzzle));
        // $map.insert(($year,11), Box::new($module::day_11::Puzzle));
        // $map.insert(($year,12), Box::new($module::day_12::Puzzle));
        // $map.insert(($year,13), Box::new($module::day_13::Puzzle));
        // $map.insert(($year,14), Box::new($module::day_14::Puzzle));
        // $map.insert(($year,15), Box::new($module::day_15::Puzzle));
        // $map.insert(($year,16), Box::new($module::day_16::Puzzle));
        // $map.insert(($year,17), Box::new($module::day_17::Puzzle));
        // $map.insert(($year,18), Box::new($module::day_18::Puzzle));
        // $map.insert(($year,19), Box::new($module::day_19::Puzzle));
        // $map.insert(($year,20), Box::new($module::day_20::Puzzle));
        // $map.insert(($year,21), Box::new($module::day_21::Puzzle));
        // $map.insert(($year,22), Box::new($module::day_22::Puzzle));
        // $map.insert(($year,23), Box::new($module::day_23::Puzzle));
        // $map.insert(($year,24), Box::new($module::day_24::Puzzle));
        // $map.insert(($year,25), Box::new($module::day_25::Puzzle));
    };
}

pub fn build_registry() -> SolutionRegistry {
    let mut registry = SolutionRegistry::new();
    register_year!(registry, year_2015, 2015);
    registry
}

pub fn fetch_input(date: &Date) -> String {
    let solution_path = PathBuf::from(format!(".inputs/{}/{}", date.0, date.1));

    if solution_path.exists() {
         fs::read_to_string(solution_path).unwrap()
    }
    else {
        String::from("temp")
    }
    /*
        1. Check cache if solution has been pulled already
        2. If in cache, return. Else, pull from website and cache it
    */
}

#[derive(Parser, Debug)]
#[command(author="Alex Wyatt", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Test with example input
    Test {
        /// Puzzle Date (Ex: 2025-1 or 2025/1)
        #[arg(value_parser = parse_date)]
        date: Date,

        // Test input filepath
        #[arg(value_name = "INPUT")]
        test: PathBuf,
    },
    /// Run puzzle using real input
    Run {
        /// Puzzle Date (Ex: 2025-1 or 2025/1)
        #[arg(value_parser = parse_date)]
        date: Date,
    },
    /// Benchmark solution time
    Time {
        /// Puzzle Date (Ex: 2025-1 or 2025/1)
        #[arg(value_parser = parse_date)]
        date: Date,
    },
    /// List all implemented solutions (defaults list all solutions)
    List {
        // List all implemented solutions for a given year (No flag will display all solutions)
        #[arg(short, long)]
        year: bool
    }
}

fn parse_date(s: &str) -> Result<Date, String> {
    let mut parts = s.split(|c| c == '-' || c == '/');
    let year = parts
        .next()
        .and_then(|y| y.parse().ok())
        .ok_or("Invalid year")?;
    let day = parts
        .next()
        .and_then(|d| d.parse().ok())
        .ok_or("Invalid day")?;
    Ok((year, day))
}
