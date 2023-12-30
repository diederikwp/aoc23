use advent_of_code::puzzles::d23::Map;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = input.parse().unwrap();
    Some(map.longest_path_len_directed())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Map = input.parse().unwrap();
    Some(map.longest_path_len_undirected())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
