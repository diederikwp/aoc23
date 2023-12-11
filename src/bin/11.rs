use advent_of_code::puzzles::d11::Galaxies;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i32> {
    let galaxies: Galaxies = input.parse().unwrap();
    Some(galaxies.sum_pairwise_dist())
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
