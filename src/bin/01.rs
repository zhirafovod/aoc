use std::collections::HashMap;
use std::ops::Add;

advent_of_code::solution!(1);

// fn calculate_abs_diff(first: i32, second: i32) -> Option<u32> {
//     let abs_diff = (first - second).abs();
//
//     abs_diff.try_into().ok()
// }

pub fn lines_to_left_right_numbers(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = input.lines().collect();
    let n = lines.len();
    let mut nums_left = Vec::with_capacity(n);
    let mut nums_right = Vec::with_capacity(n);
    lines.into_iter().for_each(|l| {
        let parts: Vec<&str>= l.trim().split_whitespace().collect();
        let num_left: i32 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: First part is not a valid number");
                return;
            }
        };

        let num_right: i32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: Second part is not a valid number");
                return;
            }
        };
        // println!("Number 1: {}", num_left);
        // println!("Number 2: {}", num_right);
        nums_left.push(num_left);
        nums_right.push(num_right);
    });
    (nums_left, nums_right)
}

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{input}");
    let (mut nums_left, mut nums_right) = lines_to_left_right_numbers(input);
    // let lines: Vec<&str> = input.lines().collect();
    // let n = lines.len();
    // let mut nums_left = Vec::with_capacity(n);
    // let mut nums_right = Vec::with_capacity(n);
    // lines.into_iter().for_each(|l| {
    //     let parts: Vec<&str>= l.trim().split_whitespace().collect();
    //     let num_left: i32 = match parts[0].parse() {
    //         Ok(n) => n,
    //         Err(_) => {
    //             eprintln!("Error: First part is not a valid number");
    //             return;
    //         }
    //     };
    //
    //     let num_right: i32 = match parts[1].parse() {
    //         Ok(n) => n,
    //         Err(_) => {
    //             eprintln!("Error: Second part is not a valid number");
    //             return;
    //         }
    //     };
    //     // println!("Number 1: {}", num_left);
    //     // println!("Number 2: {}", num_right);
    //     nums_left.push(num_left);
    //     nums_right.push(num_right);
    // });

    nums_left.sort();
    nums_right.sort();

    // let total_difference = nums_left.iter()
    //     .zip(nums_right.iter())
    //     .map(|(left, right)| left.abs_diff(*right).try_into().ok())
    //     .sum(); // conversion to i32?? FML...
    // total_difference;
    let mut total_difference: u32 = 0;
    nums_left.iter()
        .zip(nums_right.iter())
        .for_each(|(left, right)| {
            // match calculate_abs_diff(*left, *right) {
            //     Some(value) => println!("left: {}, right: {}, Abs Difference: Some({})", left, right, value),
            //     None => println!("Left: {}, right: {}, Abs Difference: None", left, right),
            // }
            total_difference += (left - right).abs() as u32;
        });
    Some(total_difference)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut nums_left, mut nums_right) = lines_to_left_right_numbers(input);
    let mut counts = HashMap::new();
    nums_right.iter().for_each(|n| {
        *counts.entry(n).or_insert(0) += 1;
    });
    let mut sum = 0;
    nums_left.iter().for_each(|n| {
        if counts.get(n).is_some() {
            sum += n * counts.get(n).unwrap();
        };
    });
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
