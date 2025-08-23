use clap::{Parser, Subcommand};
use dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
// use reqwest::*;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;

    // fn completed(&self) -> bool;
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
        $map.insert(($year, 4), Box::new($module::day_04::Puzzle));
        $map.insert(($year, 5), Box::new($module::day_05::Puzzle));
        $map.insert(($year, 6), Box::new($module::day_06::Puzzle));
        $map.insert(($year, 7), Box::new($module::day_07::Puzzle));
        $map.insert(($year, 8), Box::new($module::day_08::Puzzle));
        $map.insert(($year, 9), Box::new($module::day_09::Puzzle));
        $map.insert(($year, 10), Box::new($module::day_10::Puzzle));
        $map.insert(($year, 11), Box::new($module::day_11::Puzzle));
        $map.insert(($year, 12), Box::new($module::day_12::Puzzle));
        $map.insert(($year, 13), Box::new($module::day_13::Puzzle));
        $map.insert(($year, 14), Box::new($module::day_14::Puzzle));
        $map.insert(($year, 15), Box::new($module::day_15::Puzzle));
        $map.insert(($year, 16), Box::new($module::day_16::Puzzle));
        $map.insert(($year, 17), Box::new($module::day_17::Puzzle));
        $map.insert(($year, 18), Box::new($module::day_18::Puzzle));
        $map.insert(($year, 19), Box::new($module::day_19::Puzzle));
        $map.insert(($year, 20), Box::new($module::day_20::Puzzle));
        $map.insert(($year, 21), Box::new($module::day_21::Puzzle));
        $map.insert(($year, 22), Box::new($module::day_22::Puzzle));
        $map.insert(($year, 23), Box::new($module::day_23::Puzzle));
        $map.insert(($year, 24), Box::new($module::day_24::Puzzle));
        $map.insert(($year, 25), Box::new($module::day_25::Puzzle));
    };
}

pub fn build_registry() -> SolutionRegistry {
    let mut registry = SolutionRegistry::new();
    register_year!(registry, year_2015, 2015);
    registry
}

pub fn fetch_input(date: &Date) -> String {
    let solution_path = PathBuf::from(format!(
        "{}/.inputs/year_{}/day_{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        date.0,
        date.1
    ));

    if solution_path.exists() {
        fs::read_to_string(solution_path).unwrap()
    } else {
        let _session_token = dotenv::var("AOC_SESSION_TOKEN").unwrap();
        //
        // let url = format!("https://adventofcode.com/{}/day/{}/input", date.0, date.1);
        //
        // let client = Client::new();
        // let response = client
        //     .get(&url)
        //     .header(COOKIE, format!("session={}", session_token))
        //     .send()
        //     .text();
        //
        // let mut f = fs::File::create::new(solution_path);
        // f.write_all(response)?;
        //
        // reponse
        panic!("Querying website not implemented yet")
    }
}

// List of all completed solutions
pub fn list_solutions(year: Option<&u32>, solutions: &SolutionRegistry) {
    match year {
        Some(year) => {
            let keys: Vec<Date> = (0..26).map(|day| (*year, day)).collect();

            println!("AOC {} Solutions", year);
            for key in keys {
                let _solution = solutions.get(&key);
                println!("Day {:02}: (Add completion status later)", key.1);
            }
        }
        _ => {
            panic!("NOT DONE YET");
        }
    }
}
pub fn create_year(year: &u32) -> io::Result<()> {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let year_dir = PathBuf::from(format!("{}/src/year_{}", project_dir, year));

    let puzzle_template = r#"#![allow(warnings)]
use crate::utils::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        todo!();
    }

    fn part2(&self, input: &str) -> String {
        todo!();
    }
}
"#;

    let module_template = r#"#![allow(warnings)]
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;
"#;

    if year_dir.exists() {
        println!("Year Exists, exiting...");
        return Ok(());
    }

    println!("Creating template for {}", year);
    fs::create_dir_all(&year_dir)?;

    let mut f1 = File::create(year_dir.join("mod.rs"))?;
    f1.write_all(module_template.as_bytes())?;

    for day in 1..=25 {
        let filename = format!("day{:02}.rs", day);
        let mut file = File::create(year_dir.join(filename))?;
        file.write_all(puzzle_template.as_bytes())?;
    }

    Ok(())
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
        file: PathBuf,
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
    List { year: u32 },
    /// Create directory containing templates for a new year
    Create { year: u32 },
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
