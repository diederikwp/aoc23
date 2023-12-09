use advent_of_code::puzzles::d09::Report;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let report: Report = input.parse().unwrap();
    Some(report.sum_extrapolated())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}