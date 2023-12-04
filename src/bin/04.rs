use advent_of_code::puzzles::d04::CardGame;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_points = 0;
    for line in input.lines() {
        let game: CardGame = line.parse().ok()?;
        total_points += game.points();
    }

    Some(total_points)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
