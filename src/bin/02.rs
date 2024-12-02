advent_of_code::solution!(2);

// 1) compare the abs diff is in [1-3]
// 2) compare increasing or decreasing monotonically.
pub fn part_one(input: &str) -> Option<u32> {
    let mut safes = 0;
    input.lines().for_each(|l| {
        let mut increasing: Option<bool> = None;
        let arr: Vec<i32> = l
            .trim()
            .split_whitespace()
            .map(|e|  e.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert");
        if arr.len() <= 1 {
            println!("less that 2 elements in the sequence {l}");
            return;
        }
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i-1];
            if diff.abs() == 0 || diff.abs() > 3 {
                println!("Change is too big or too small for {l} got a diff {diff}");
                return;
            }
            match increasing {
                None => increasing = Some(diff > 0),
                Some(true) => {
                    if diff < 0 {
                        println!("increasing sequence {l} got a diff {diff}");
                        return;
                    }
                }
                Some(false) => {
                    if diff > 0 {
                        println!("decreasing sequence {l} got a diff {diff} on {i}");
                        return;
                    }
                }
            }
        }
        println!("{l} is safe!");
        safes += 1;
    });
    Some(safes as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    // 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    // 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    // 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
    //
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    //
    fn part1_examples() {
        let result = part_one(r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#);
        assert_eq!(result.unwrap(), 2_u32, "result does not match");

    }

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
