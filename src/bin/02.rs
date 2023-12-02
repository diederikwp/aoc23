use advent_of_code::config::SETTINGS;
use std::{error::Error, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for CubeSet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b) = (0, 0, 0);

        for part in s.split(", ") {
            let (num_str, color_str) = part.split_once(' ').ok_or("Invalid syntax")?;
            let num: u8 = num_str.parse()?;
            match color_str {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => return Err("invalid color".into()),
            };
            // Note: If the same color occurs multiple times, we take the last
            // occurence.
        }

        Ok(CubeSet {
            red: r,
            green: g,
            blue: b,
        })
    }
}

impl CubeSet {
    fn is_subset(&self, other: &CubeSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Debug)]
struct Game {
    idx: u32,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_sets = Vec::new();

        let (game_str, cube_sets_str) = s.split_once(": ").ok_or("Invalid syntax")?;

        // Parse idx (assuming first word is Game)
        let (_, idx_str) = game_str.split_once(' ').ok_or("Invalid syntax")?;
        let idx = idx_str.parse()?;

        // Parse cubesets
        for cube_set_str in cube_sets_str.split("; ") {
            cube_sets.push(cube_set_str.parse()?);
        }

        Ok(Game { idx, cube_sets })
    }
}

impl Game {
    fn is_posible(&self, bag_contents: &CubeSet) -> bool {
        for cs in &self.cube_sets {
            if !cs.is_subset(bag_contents) {
                return false;
            }
        }
        true
    }

    fn min_power(&self) -> u32 {
        let (mut min_r, mut min_g, mut min_b) = (0, 0, 0);
        for cs in &self.cube_sets {
            min_r = min_r.max(cs.red);
            min_g = min_g.max(cs.green);
            min_b = min_b.max(cs.blue);
        }

        u32::from(min_r) * u32::from(min_g) * u32::from(min_b)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag_contents_ = SETTINGS.day02.bag_contents;
    let bag_contents = CubeSet {
        red: bag_contents_[0],
        green: bag_contents_[1],
        blue: bag_contents_[2],
    };

    let mut n_possible = 0;
    let games = parse_games(input).ok()?;
    for game in games {
        if game.is_posible(&bag_contents) {
            n_possible += game.idx;
        }
    }

    Some(n_possible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input).ok()?;
    Some(games.iter().map(|g| g.min_power()).sum())
}

fn parse_games(input: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut games = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let game: Game = line.parse()?;
        if game.idx != (idx + 1).try_into()? {
            return Err(format!("Invalid game index {}, expected {}", game.idx, idx + 1).into());
        }

        games.push(game);
    }
    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
