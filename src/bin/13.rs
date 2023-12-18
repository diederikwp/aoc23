use advent_of_code::puzzles::d13::Valley;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let valley: Valley = input.parse().unwrap();
    Some(valley.sum_symmetry_score())
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
