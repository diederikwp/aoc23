use advent_of_code::puzzles::d24::Hail;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u32> {
    let hail: Hail = input.parse().unwrap();
    let range = 200_000_000_000_000f64..400_000_000_000_000f64;
    Some(hail.count_intersections_within_xy(&range, &range))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
