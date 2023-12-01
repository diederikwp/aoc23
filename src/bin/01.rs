use std::error::Error;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(get_2_digits).sum::<Result<u32, _>>().ok()
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
