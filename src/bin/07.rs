use advent_of_code::puzzles::d07::HandsList;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let hands_list: HandsList = input.parse().unwrap();
    Some(hands_list.total_winnings())
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
