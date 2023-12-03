use advent_of_code::puzzles::d03::{Number, Schematic};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::new(input);
    let selected_nums = schematic.select_part_number_idxs();

    Some(
        schematic
            .numbers()
            .iter()
            .filter(|Number { idx: i, val: _ }| selected_nums[*i])
            .map(|Number { idx: _, val }| val)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the schematic
    let schematic = Schematic::new(input);
    Some(schematic.total_gear_ratio())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
