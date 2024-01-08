use advent_of_code::puzzles::d25::Wiring;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let wiring: Wiring = input.parse().unwrap();
    let min_cut_sizes = wiring.min_cut().component_sizes();
    Some(u32::try_from(min_cut_sizes.0 * min_cut_sizes.1).unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(42) // There is no puzzle to solve for part 2 :)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
