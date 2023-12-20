use std::{error::Error, str::FromStr};

use ndarray::Array2;

pub struct DigPlan(Vec<Instruction>);

impl FromStr for DigPlan {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Instruction>, _>>()?;

        Ok(DigPlan(instructions))
    }
}

impl DigPlan {
    pub fn dig_terrain(&self) -> Terrain {
        // Make a list of coordinates of the dug out path, keeping track of the
        // bounding box.
        let mut dug_tiles = Vec::new();
        let mut y_range = (0, 0);
        let mut x_range = (0, 0);
        let mut pos = (0, 0);

        for idx in 0..self.0.len() {
            let instruction = &self.0[idx];
            let ((dy, dx), tile) = match instruction.direction {
                Direction::Right => ((0, 1), b'-'),
                Direction::Down => ((1, 0), b'|'),
                Direction::Left => ((0, -1), b'-'),
                Direction::Up => ((-1, 0), b'|'),
            };

            for _ in 0..instruction.depth {
                pos.0 += dy;
                pos.1 += dx;
                dug_tiles.push((pos, tile));

                y_range = (isize::min(y_range.0, pos.0), isize::max(y_range.1, pos.0));
                x_range = (isize::min(x_range.0, pos.1), isize::max(x_range.1, pos.1));
            }

            // Replace the last one by the appropriate corner
            let next_instruction = &self.0[(idx + 1) % self.0.len()]; // assume circular path
            let tile = match (instruction.direction, next_instruction.direction) {
                (Direction::Right, Direction::Up) => b'J',
                (Direction::Right, Direction::Down) => b'7',
                (Direction::Down, Direction::Right) => b'L',
                (Direction::Down, Direction::Left) => b'J',
                (Direction::Left, Direction::Up) => b'L',
                (Direction::Left, Direction::Down) => b'F',
                (Direction::Up, Direction::Right) => b'F',
                (Direction::Up, Direction::Left) => b'7',

                _ => panic!("Invalid instruction sequence"), // cannot turn 180° or repeat same direction
            };
            dug_tiles.pop();
            dug_tiles.push((pos, tile));
        }

        // Transform to a 2D array.
        let shape = (
            usize::try_from(y_range.1 - y_range.0 + 1).unwrap(),
            usize::try_from(x_range.1 - x_range.0 + 1).unwrap(),
        );
        let mut terrain = Array2::from_elem(shape, b'.');
        for ((y, x), tile) in dug_tiles {
            let pos = (
                usize::try_from(y - y_range.0).unwrap(),
                usize::try_from(x - x_range.0).unwrap(),
            );
            terrain[pos] = tile;
        }

        Terrain(terrain)
    }
}

pub struct Terrain(Array2<u8>);

impl Terrain {
    pub fn interior_area(&self) -> u32 {
        // assuming the dug path forms a closed loop
        let mut area = 0;

        for y in 0..self.0.shape()[0] {
            let mut state = ScanState::Outside;

            for x in 0..self.0.shape()[1] {
                match self.0[(y, x)] {
                    b'-' => area += 1,
                    b'|' => {
                        area += 1;
                        state = match state {
                            ScanState::Inside => ScanState::Outside,
                            ScanState::Outside => ScanState::Inside,
                            _ => panic!("Invalid tile"),
                        }
                    }
                    b'.' => match state {
                        ScanState::Inside => area += 1,
                        ScanState::Outside => (),
                        _ => panic!("Invalid tile"),
                    },
                    b'L' => {
                        area += 1;
                        state = match state {
                            ScanState::Inside => ScanState::UpperEdge,
                            ScanState::Outside => ScanState::LowerEdge,
                            _ => panic!("Invalid tile"),
                        }
                    }
                    b'J' => {
                        area += 1;
                        state = match state {
                            ScanState::UpperEdge => ScanState::Inside,
                            ScanState::LowerEdge => ScanState::Outside,
                            _ => panic!("Invalid tile"),
                        }
                    }
                    b'7' => {
                        area += 1;
                        state = match state {
                            ScanState::UpperEdge => ScanState::Outside,
                            ScanState::LowerEdge => ScanState::Inside,
                            _ => panic!("Invalid tile"),
                        }
                    }
                    b'F' => {
                        area += 1;
                        state = match state {
                            ScanState::Inside => ScanState::LowerEdge,
                            ScanState::Outside => ScanState::UpperEdge,
                            _ => panic!("Invalid tile"),
                        }
                    }
                    _ => panic!("Invalid tile"),
                };
            }
        }

        area
    }
}

struct Instruction {
    direction: Direction,
    depth: u8,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = parts
            .next()
            .ok_or::<String>("Missing direction".into())?
            .parse()?;
        let depth = parts
            .next()
            .ok_or::<String>("Missing depth".into())?
            .parse()?;
        parts.next().ok_or::<String>("Missing RGB".into())?; // skip the rgb for now

        if parts.next().is_some() {
            return Err("Too many parts".into());
        }

        Ok(Instruction { direction, depth })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Direction should be 1 byte".into())
        } else {
            match s.chars().next().unwrap() {
                'R' => Ok(Direction::Right),
                'D' => Ok(Direction::Down),
                'L' => Ok(Direction::Left),
                'U' => Ok(Direction::Up),
                _ => Err("Invalid character".into()),
            }
        }
    }
}

enum ScanState {
    Inside,
    Outside,
    LowerEdge,
    UpperEdge,
}
