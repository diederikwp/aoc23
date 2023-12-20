use advent_of_code::puzzles::d18::DigPlan;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let plan: DigPlan = input.parse().unwrap();
    Some(plan.dig_terrain_using_depth().total_area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let plan: DigPlan = input.parse().unwrap();
    Some(plan.dig_terrain_using_color().total_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
