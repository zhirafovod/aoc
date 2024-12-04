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
        // println!("{l}: {r}\n");
        sum += r;
    });
    Some(sum as u32)
}

pub fn part_two_slow(input: &str) -> Option<u32> {
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut tokens = re_do.find_iter(input).map(|m| (m.start(), "do", m.end()))
        .chain(re_dont.find_iter(input).map(|m| (m.start(), "don't", m.end())))
        .chain(re_mul.find_iter(input).map(|m| (m.start(), m.as_str(), m.end())))
        .collect::<Vec<_>>();
        // .for_each(|(i, m, _)| println!("{i}:{m:?}"));

    tokens.sort_by_key(|(index, _ , _)| *index);
    tokens.iter().for_each(|e| println!("{e:?}"));
    let mut enabled = true;
    let mut sum = 0;
    for (_, op, _) in tokens {
        match op {
            "don't" => enabled = false,
            "do" => enabled = true,
            l => if enabled { sum += apply_mul(l) },
        }
    }

    Some(sum as u32)
}

pub fn part_two(l: &str) -> Option<u32> {
    part_two_slow(l)
}

pub fn part_two_faster(s: &str) -> Option<u32> {
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_mul = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut cur = 0;
    let mut enabled = true;
    let mut sum = 0;
    while cur < s.len() {
        if &s[cur..].starts_with("do()") {
            enabled = true;
            cur += 4;
        } else if &s[cur..].starts_with("don't()") {
            enabled = true;
            cur += 7
        } else if let Some(caps) = re_mul.captures(&s[cur..]) {
            if caps.len() > 0 {
                sum += apply_mul(&s[cur..]);
            } else {
                cur += 1;
            }
        } else {
            cur += 1;
        }
    };
    Some(cur as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_example() {
        let result = part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
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
