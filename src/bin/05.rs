use advent_of_code::puzzles::d05::Almanac;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let almanac: Almanac = input.parse().ok()?;

    let mut min_location_num = u64::MAX;
    for &seed_num in almanac.seeds() {
        min_location_num = min_location_num.min(almanac.get_location_num(seed_num));
    }
    Some(min_location_num)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
