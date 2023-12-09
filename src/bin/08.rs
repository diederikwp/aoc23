use advent_of_code::puzzles::d08::Network;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let network: Network = input.parse().unwrap();
    Some(network.n_steps_from_to_single("AAA", "ZZZ"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let network: Network = input.parse().unwrap();
    Some(network.n_steps_all_a_to_all_z())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
