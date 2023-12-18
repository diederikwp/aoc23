use advent_of_code::puzzles::d12::Field;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let field: Field = input.parse().unwrap();
    Some(field.total_arrangement_count())
}

pub fn part_two(input: &str) -> Option<u64> {
    let field: Field = input.parse().unwrap();
    Some(field.total_arrangement_count_extended())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
