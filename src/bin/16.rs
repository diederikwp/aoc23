use advent_of_code::puzzles::d16::{Direction, MirrorGrid};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let mirrors: MirrorGrid = input.parse().unwrap();
    Some(mirrors.follow_beam((0, 0), Direction::East).num_energized())
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
