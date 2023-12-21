use advent_of_code::puzzles::d20::ModuleNetwork;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut network: ModuleNetwork = input.parse().unwrap();
    let (n_low, n_high) = network.press_multiple_and_count_pulses(1000);

    Some(n_low * n_high)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut network: ModuleNetwork = input.parse().unwrap();
    Some(network.steps_until_rx_first_low())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(320_00_000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    // No test case for part 2
}
