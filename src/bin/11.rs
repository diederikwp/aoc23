use advent_of_code::puzzles::d11::Galaxies;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    let mut galaxies: Galaxies = input.parse().unwrap();
    galaxies.expand(2);
    Some(galaxies.sum_pairwise_dist())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut galaxies: Galaxies = input.parse().unwrap();
    galaxies.expand(1_000_000);
    Some(galaxies.sum_pairwise_dist())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }
    // No test case available for part 1. See unit tests in crate::puzzles::d11
}
