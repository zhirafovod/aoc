use indexmap::map::IndexMap;
use regex::Regex;

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

// in this part, we should parse the string and extract all the numbers.
//
// Origianlly I tried to convert the bigger numbers will be prefaced with
// a normal digit and ending with a digit/*teen, and hundred, thousand, "*lion", but it does not
// seem to be the case in the test input
//
// "one" = 1
// "two" => 2
// ...
// nine => 9
//
// sixteen => 16,
// nineteen == 19
//
// thirty, forty, fifty...
// hundred, thousand, million, billion, trillion
//
// test cases:
// two1nine => 219
// eightwothree => 8wo3
// abcone2threexyz => abc123xyz
// xtwone3four => x2ne34
// 4nineeightseven2 => 49872
// zoneight234 => 1234
// 7pqrstsixteen => 7ppgrst16
//
fn words_to_digits(l: String) -> String {
    // use indexMap to preserve key insertion order, so we can handle "millions" before "million"
    let mut number_map = IndexMap::new();
    // number_map.insert("zero", "0"); // we don't care about zero.
    number_map.insert("one", "1");
    number_map.insert("two", "2");
    number_map.insert("three", "3");
    number_map.insert("four", "4");
    number_map.insert("five", "5");
    number_map.insert("six", "6");
    number_map.insert("seven", "7");
    number_map.insert("eight", "8");
    number_map.insert("nine", "9");

    // Below actually not in the puzzle description and not important
    // number_map.insert("ten", "10");
    // number_map.insert("eleven", "11");
    // number_map.insert("twelve", "12");
    // number_map.insert("thirteen", "13");
    // number_map.insert("fourteen", "14");
    // number_map.insert("fifteen", "15");
    // number_map.insert("sixteen", "16");
    // number_map.insert("seventeen", "17");
    // number_map.insert("eighteen", "18");
    // number_map.insert("nineteen", "19");
    // number_map.insert("twenty", "20");
    // number_map.insert("thirty", "30");
    // number_map.insert("forty", "40");
    // number_map.insert("fifty", "50");
    // number_map.insert("sixty", "60");
    // number_map.insert("seventy", "70");
    // number_map.insert("eighty", "80");
    // number_map.insert("ninety", "90");
    // number_map.insert("hundred", "100");
    // number_map.insert("hundreds", "100");
    // number_map.insert("thousands", "1000");
    // number_map.insert("thousand", "1000");
    // number_map.insert("millions", "1000000");
    // number_map.insert("million", "1000000");
    // number_map.insert("billions", "1000000000");
    // number_map.insert("billion", "1000000000");
    // number_map.insert("trillions", "1000000000000");
    // number_map.insert("trillion", "1000000000000");

    // Compile a regex from the map keys. The regex will be like "one|two|...|trillion"
    // keys will be matched in order
    let regex_pattern = number_map.keys().cloned().collect::<Vec<_>>().join("|");
    let re = Regex::new(&regex_pattern).unwrap();

    let result = re.replace_all(&l, |caps: &regex::Captures| {
        let word = &caps[0];
        number_map.get(word).unwrap_or(&word).to_string()
    });

    result.to_string()
}

fn string_to_digit(l: String) -> u32 {
    let mut number_map = IndexMap::new();
    number_map.insert("one", "1");
    number_map.insert("two", "2");
    number_map.insert("three", "3");
    number_map.insert("four", "4");
    number_map.insert("five", "5");
    number_map.insert("six", "6");
    number_map.insert("seven", "7");
    number_map.insert("eight", "8");
    number_map.insert("nine", "9");


    let regex_pattern = number_map.keys().cloned().collect::<Vec<_>>().join("");
    let re = Regex::new(&regex_pattern).unwrap();
    // re.captu
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let lower_lines = input.to_lowercase();
    lower_lines.lines().for_each(|l| {
        result += num_from_string(words_to_digits(String::from(l)));

    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 53651);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 53896);
    }

    // #[test]
    // fn test_string1() {
    //     let result = num_from_string(String::from("fivepqxlpninevh2xxsnsgg63pbvdnqptmg"));
    //     assert_eq!(result, 23);
    // }
    //
    // #[test]
    // fn test_part2_string1() {
    //     let result = num_from_string(words_to_digits(String::from("fivepqxlpninevh2xxsnsgg63pbvdnqptmg")));
    //     assert_eq!(result, 53);
    // }
    //
    // #[test]
    // fn test_part2_strings() {
    //     let test_cases: Vec<(&str, u32)> = vec![
    //         ("two1nine", 29),
    //         ("eightwothree", 83),
    //         ("abcone2threexyz", 13),
    //         ("xtwone3four", 24),
    //         ("4nineeightseven2", 42),
    //         ("zoneight234", 14),
    //         ("7pqrstsixteen", 76),
    //     ];
    //     let mut sum = 0_u32;
    //     test_cases.iter().for_each(|(input, expected)| {
    //         let result = num_from_string(words_to_digits(input.to_string()));
    //         sum += result;
    //         assert_eq!(result, *expected, "Result {result} for {input} did not match expected value {expected}.");
    //     });
    //     assert_eq!(sum, 281);
    // }

}
