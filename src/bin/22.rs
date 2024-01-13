use advent_of_code::puzzles::d22::BrickPile;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let brick_pile: BrickPile = input.parse().unwrap();
    Some(brick_pile.n_bricks_destroyable())
}

pub fn part_two(input: &str) -> Option<u32> {
    let brick_pile: BrickPile = input.parse().unwrap();
    Some(brick_pile.sum_falling_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
