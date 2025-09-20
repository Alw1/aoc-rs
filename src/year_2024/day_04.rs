use crate::Solution;

pub struct Puzzle;

// fn check_right(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y + 1],
//             arr[x][y + 2],
//             arr[x][y + 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_left(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) < 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y - 1],
//             arr[x][y - 2],
//             arr[x][y - 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_up(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - x) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x - 1][y],
//             arr[x - 2][y],
//             arr[x - 3][y]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_down(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - x) < 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x + 1][y],
//             arr[x + 2][y],
//             arr[x + 3][y]
//         );
//
//         if word == "XMAS" {
//             Some(1)
//         }
//     } else {
//         None
//     }
// }
//
// fn check_upper_right(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y + 1],
//             arr[x][y + 2],
//             arr[x][y + 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_upper_left(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y + 1],
//             arr[x][y + 2],
//             arr[x][y + 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_lower_right(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y + 1],
//             arr[x][y + 2],
//             arr[x][y + 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }
//
// fn check_lower_left(x: usize, y: usize, arr: Vec<Vec<&str>>) -> Option<u32> {
//     if (arr[x].len() - y) > 3 {
//         let word = format!(
//             "{}{}{}{}",
//             arr[x][y],
//             arr[x][y + 1],
//             arr[x][y + 2],
//             arr[x][y + 3]
//         );
//
//         if word == "XMAS" {
//             return Some(1);
//         } else {
//             return None;
//         }
//     }
//
//     None
// }

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let arr: Vec<Vec<&str>> = input.lines().map(|line| line.split("").collect()).collect();

        let mut count: u32 = 0;

        // let direction_check: [fn(usize, usize, Vec<Vec<&str>>) -> Option<u32>; 8] = [
        //     check_right,
        //     check_left,
        //     check_up,
        //     check_down,
        //     check_upper_right,
        //     check_upper_left,
        //     check_lower_right,
        //     check_lower_left,
        // ];
        //
        // for x in 0..arr.len() {
        //     for y in 0..arr[x].len() {
        //         count += direction_check
        //             .iter()
        //             .map(|dir| dir(x, y, &arr).unwrap_or(0))
        //             .sum::<u32>();
        //     }
        // }

        format!("{}", count)
    }

    fn part2(&self, input: &str) -> String {
        let count = 0;

        format!("{}", count)
    }
}
