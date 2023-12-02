use regex::Regex;
use std::error::Error;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(get_2_digits).sum::<Result<u32, _>>().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(replace_digits)
        // .map(replace_numbers)
        .map(|l| get_2_digits(&l))
        .sum::<Result<u32, _>>()
        .ok()
}

fn get_2_digits(line: &str) -> Result<u32, Box<dyn Error>> {
    // We know it's only 2 digits, so we can prevent a heap allocation by using
    // a stack allocated [u8; 2]
    let mut digits = [0, 0];

    // forward search
    for c in line.bytes() {
        if c.is_ascii_digit() {
            digits[0] = c;
            break;
        }
    }

    // backward search
    for c in line.bytes().rev() {
        if c.is_ascii_digit() {
            digits[1] = c;
            break;
        }
    }

    if digits[0] == 0 || digits[1] == 0 {
        return Err("Less than 2 digits in line".into());
    }

    Ok((std::str::from_utf8(&digits))?.parse()?)
}

fn replace_digits(line: &str) -> String {
    let mut replaced = String::from(line);

    // Todo: don't recompile regex every time
    // Todo: find a single regexp that captures either 1 or 2 numbers
    let rx1 = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let rx2 = Regex::new(concat!(
        r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*",
        r"(\d|one|two|three|four|five|six|seven|eight|nine).*?$"
    ))
    .unwrap();

    // First try capturing 2 numbers
    match rx2.captures(line) {
        None => (),
        Some(caps) => {
            let c1 = caps.get(1).unwrap();
            let c2 = caps.get(2).unwrap();
            replaced.replace_range(c2.start()..c2.end(), word2digit(c2.as_str()));
            replaced.replace_range(c1.start()..c1.end(), word2digit(c1.as_str()));

            return replaced;
        }
    };

    // There is at most 1 number
    match rx1.captures(line) {
        None => (),
        Some(caps) => {
            let c = caps.get(1).unwrap();
            replaced.replace_range(c.start()..c.end(), word2digit(c.as_str()));
        }
    };

    replaced
}

fn word2digit(written: &str) -> &str {
    match written {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        s => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_2_digits() {
        assert_eq!(get_2_digits("1abc2").unwrap(), 12);
        assert_eq!(get_2_digits("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(get_2_digits("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(get_2_digits("treb7uchet").unwrap(), 77);
    }

    #[test]
    fn test_replace_written_digits() {
        assert_eq!(replace_digits("two1nine"), "219");
        assert_eq!(replace_digits("eightwothree"), "8wo3");
        assert_eq!(replace_digits("abcone2threexyz"), "abc123xyz");
        assert_eq!(replace_digits("xtwone3four"), "x2ne34");
        assert_eq!(replace_digits("4nineeightseven2"), "49eight72");
        assert_eq!(replace_digits("zoneight234"), "z1ight234");
        assert_eq!(replace_digits("7pqrstsixteen"), "7pqrst6teen");
        assert_eq!(replace_digits("abcde"), "abcde");
        assert_eq!(replace_digits("80073"), "80073");
        assert_eq!(replace_digits("two"), "2");
        assert_eq!(replace_digits("onene"), "1ne");
        assert_eq!(replace_digits("oneight"), "1ight"); // or on8?
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        // Only digits
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));

        // Digits and numbers
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
