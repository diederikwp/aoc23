use advent_of_code::puzzles::d21::Garden;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u64> {
    let garden: Garden = input.parse().unwrap();
    Some(garden.num_tiles_reacheable_after(64, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let garden: Garden = input.parse().unwrap();
    Some(garden.num_tiles_reacheable_extrapolated(26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Testcases can be found in module d21

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
