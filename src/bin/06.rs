use advent_of_code::puzzles::d06::BoatTable;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let table: BoatTable = input.parse().ok()?;
    Some(table.n_ways_to_win())
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
