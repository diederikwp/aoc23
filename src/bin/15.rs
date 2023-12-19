use advent_of_code::puzzles::d15::InitSequence;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sequence: InitSequence = input.parse().unwrap();
    Some(sequence.sum_hashes())
}

pub fn part_two(input: &str) -> Option<u32> {
    let sequence: InitSequence = input.parse().unwrap();
    Some(sequence.total_resulting_power())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
