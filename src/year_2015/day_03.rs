use crate::Solution;
use std::collections::HashMap;

pub struct Puzzle;

type Coordinate = (i8, i8);

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut houses: HashMap<Coordinate, u32> = HashMap::new();
        let mut location: Coordinate = (0, 0);

        for direction in input.chars() {
            // Drop present at current house
            houses.insert(location, 1);

            match direction {
                '>' => location.0 += 1,
                '<' => location.0 -= 1,
                '^' => location.1 += 1,
                'v' => location.1 -= 1,
                _ => {}
            }

            match houses.get(&location) {
                Some(&x) => {
                    // If visited before, drop another present
                    houses.insert(location, x + 1);
                }
                _ => {
                    // Else, drop first present
                    houses.insert(location, 1);
                }
            };
        }

        houses.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut houses: HashMap<Coordinate, u32> = HashMap::new();
        let mut santa_location: Coordinate = (0, 0);
        let mut santas_turn: bool = false;
        let mut robot_location: Coordinate = (0, 0);
        let mut location: &mut Coordinate = &mut santa_location;

        for direction in input.chars() {
            // Drop present at current house
            houses.insert(*location, 1);

            match direction {
                '>' => location.0 += 1,
                '<' => location.0 -= 1,
                '^' => location.1 += 1,
                'v' => location.1 -= 1,
                _ => {}
            }

            match houses.get(&location) {
                Some(&x) => {
                    // If visited before, drop another present
                    houses.insert(*location, x + 1);
                }
                _ => {
                    // Else, drop first present
                    houses.insert(*location, 1);
                }
            };

            location = if santas_turn {
                &mut santa_location
            } else {
                &mut robot_location
            };

            santas_turn = !santas_turn;
        }

        houses.len().to_string()
    }
}
