use tinyjson::format;

advent_of_code::solution!(1);

fn num_from_string(l: String) -> u32 {
    let mut start:Option<u32> = None;
    let mut end:Option<u32> = None;
    // println!("l: {l}");
    for c in l.chars() {
        if c.is_digit(10) {
            start = c.to_digit(10);
            break;
        };
    };
    for c in l.chars().rev() {
        if c.is_digit(10) {
            end = c.to_digit(10);
            break;
        };
    };

    match (start, end) {
        (Some(start), Some(end)) => format!("{start}{end}").parse::<u32>().unwrap(),
        (Some(start), None) => {
            println!("only start for l {l}, {start}");
            format!("{start}{start}").parse::<u32>().unwrap()
        },
        _ => {
            println!("No match for l {l}");
            0
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    input.lines().for_each(|l| {
        result += num_from_string(String::from(l));

    });
    Some(result)
}

// in this option, we should parse the string and extract all the numbers. One of the challenges,
// that we need to extract composite numbers with "teen". The bigger numbers will be prefaced with
// a normal digit and ending with a digit/*teen, and hundred, thousand, "*lion".
//
// "one" = 1
// "two" => 2
// ...
// nine => 9
// sixteen => 16,
// nineteen == 19
//
// thirty, forty, fifty...
// hundred, thousand, million, billion, trillion
//
// test cases:
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
//
fn words_to_digits(l: String) {

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string1() {
        let result = num_from_string(String::from("fivepqxlpninevh2xxsnsgg63pbvdnqptmg"));
        assert_eq!(result, 23);
    }

    #[test]
    fn test_part2_string1() {
        let result = num_from_string(String::from("fivepqxlpninevh2xxsnsgg63pbvdnqptmg"));
        assert_eq!(result, 23);
    }


    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result.unwrap(), 53651);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     // assert_eq!(result.unwrap(), 1);
    // }
}
