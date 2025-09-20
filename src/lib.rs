pub mod utils;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub type Date = (u32, u8);
pub type SolutionRegistry = std::collections::HashMap<Date, Box<dyn Solution>>;

register_years! {
    year_2015 => 2015,
    year_2024 => 2024,
}
