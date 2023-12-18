use advent_of_code::puzzles::d14::Platform;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform: Platform = input.parse().unwrap();
    platform.slide_north();
    Some(platform.total_load())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform: Platform = input.parse().unwrap();
    platform.spin(1_000_000_000);
    Some(platform.total_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
