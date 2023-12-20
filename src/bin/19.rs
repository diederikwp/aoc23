use advent_of_code::puzzles::d19::System;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let system: System = input.parse().unwrap();
    Some(system.sum_accepted())
}

pub fn part_two(input: &str) -> Option<u64> {
    let system: System = input.parse().unwrap();
    Some(system.n_distinct_accepted())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167_409_079_868_000u64));
    }
}
