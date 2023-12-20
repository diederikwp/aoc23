use std::{error::Error, str::FromStr};

pub struct DigPlan(Vec<InstructionParams>);

impl FromStr for DigPlan {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<InstructionParams>, _>>()?;

        Ok(DigPlan(instructions))
    }
}

impl DigPlan {
    pub fn dig_terrain_using_depth(self) -> Terrain {
        let instructions = self.0.into_iter().map(|ip| ip.into_instruction_as_is());
        DigPlan::dig_terrain(instructions)
    }

    pub fn dig_terrain_using_color(self) -> Terrain {
        let instructions = self
            .0
            .into_iter()
            .map(|ip| ip.into_instruction_using_color());
        DigPlan::dig_terrain(instructions)
    }

    fn dig_terrain<I: IntoIterator<Item = Instruction>>(instructions: I) -> Terrain {
        // Make a list of the vertices of the dug out path
        let mut verts = Vec::new();
        let mut pos = (0, 0);

        for instruction in instructions.into_iter() {
            let (dy, dx) = match instruction.direction {
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Up => (-1, 0),
            };

            pos = (
                pos.0 + i64::from(instruction.depth) * dy,
                pos.1 + i64::from(instruction.depth) * dx,
            );
            verts.push(pos);
        }

        Terrain(verts)
    }
}

pub struct Terrain(Vec<(i64, i64)>); // Vec of vertices

impl Terrain {
    pub fn total_area(&self) -> u64 {
        self.interior_area() + self.perimeter_area()
    }

    fn interior_area(&self) -> u64 {
        // assuming the dug path forms a closed loop, calculate the area using
        // the trapezoid formula.
        let sum = std::iter::zip(&self.0, self.0.iter().cycle().skip(1))
            .map(|(vert, next_vert)| (vert.0 + next_vert.0) * (vert.1 - next_vert.1))
            .sum::<i64>()
            .abs()
            / 2;

        u64::try_from(sum).unwrap()
    }

    fn perimeter_area(&self) -> u64 {
        // Since we work with discrete "pixels", the perimeter has itself an
        // area. Going clockwise, there is a contribution of 3/4 for every right
        // corner and 1/4 for every left corner.
        let n = self.0.len();
        let n_left_right_pairs = (n - 4) / 2;
        let n_right = 4 + n_left_right_pairs;
        let n_left = n_left_right_pairs;
        let area_corners = (3 * n_right + n_left) / 4;

        // Every straight piece of perimeter contributes 1/2
        let area_straight = std::iter::zip(&self.0, self.0.iter().cycle().skip(1))
            .map(|(vert, next_vert)| {
                (vert.0 - next_vert.0).abs() + (vert.1 - next_vert.1).abs() - 1
            })
            .sum::<i64>()
            .abs()
            / 2;

        u64::try_from(area_corners).unwrap() + u64::try_from(area_straight).unwrap()
    }
}

struct InstructionParams {
    direction: Direction,
    depth: u8,
    color: [u8; 3],
}

impl FromStr for InstructionParams {
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
        let rgb_str = parts
            .next()
            .ok_or::<String>("Missing RGB".into())?
            .get(2..8)
            .ok_or::<String>("rgb string should have 8 chars".into())?;
        if parts.next().is_some() {
            return Err("Too many parts".into());
        }

        let color = [
            u8::from_str_radix(&rgb_str[0..2], 16)?,
            u8::from_str_radix(&rgb_str[2..4], 16)?,
            u8::from_str_radix(&rgb_str[4..6], 16)?,
        ];

        Ok(InstructionParams {
            direction,
            depth,
            color,
        })
    }
}

impl InstructionParams {
    fn into_instruction_as_is(self) -> Instruction {
        Instruction {
            direction: self.direction,
            depth: u32::from(self.depth),
        }
    }

    fn into_instruction_using_color(self) -> Instruction {
        let depth =
            u32::from_be_bytes([0, self.color[0], self.color[1], self.color[2] & 0xF0]) >> 4;
        let direction_num = self.color[2] & 0x0F;
        let direction = match direction_num {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction number"),
        };

        Instruction { direction, depth }
    }
}

struct Instruction {
    direction: Direction,
    depth: u32,
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
