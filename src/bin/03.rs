use regex::Regex;

advent_of_code::solution!(3);


fn apply_mul(l: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for cap in re.captures_iter(l) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        sum += num1 * num2;
    }
    sum

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    input.lines().for_each(|l| {
        let r = apply_mul(l);
        println!("{l}: {r}\n");
        sum += r;
    });
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // let re_do = Regex::new(r"(^(don't|$)");
    // let re_dont = Regex::
    None
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
