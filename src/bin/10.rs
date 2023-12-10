use advent_of_code::puzzles::d10::Maze;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Maze = input.parse().unwrap();
    Some(maze.loop_len() / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze: Maze = input.parse().unwrap();
    Some(maze.enclosed_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
