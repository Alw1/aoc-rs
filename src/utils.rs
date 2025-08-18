use std::collections::HashMap;

/*
    Add utility func here to pull actual test inputs and then hide them in a hidden folder
    where the inputs are cached but not tracked by git (Also make sure session token isn't tracked);
*/

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub type SolutionRegistry = HashMap<(u32, u8), Box<dyn Solution>>;

#[macro_export]
macro_rules! benchmark_fn {
    ($func_call:expr) => {{
        let start_time = Instant::now();
        let result = $func_call;
        let elapsed_time = start_time.elapsed();
        println!("Function executed in: {:?}", elapsed_time);
        result
    }};
}

#[macro_export]
macro_rules! register_year {
    ($map:ident, $module:ident, $year:expr) => {
            use crate::$module;
            $map.insert(($year, 1), Box::new($module::day_01::Puzzle));
            // $map.insert(($year, 2), Box::new($module::day_02::Puzzle));
            // $map.insert(($year, 3), Box::new($module::day_03::Puzzle));
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

