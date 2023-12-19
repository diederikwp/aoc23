use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};

pub struct MirrorGrid {
    grid: Array2<u8>,
}

impl FromStr for MirrorGrid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let linear_grid = Array::from_iter(s.bytes().filter(|&b| b != b'\n'));
        let height = linear_grid.len() / width;

        Ok(MirrorGrid {
            grid: linear_grid.into_shape((height, width))?,
        })
    }
}

impl MirrorGrid {
    pub fn follow_beam(&self, entry_pos: (isize, isize), entry_direction: Direction) -> BeamPath {
        let mut path = Array2::from_elem(self.grid.raw_dim(), 0);
        let mut beam_heads = vec![(entry_pos, entry_direction)];

        while let Some((head_pos, head_direction)) = beam_heads.pop() {
            // The currently examined beam is about to enter head_pos, traveling
            // in direction head_direction.

            // Skip if leaving grid
            if head_pos.0 < 0
                || head_pos.1 < 0
                || head_pos.0 >= isize::try_from(path.shape()[0]).unwrap()
                || head_pos.1 >= isize::try_from(path.shape()[1]).unwrap()
            {
                continue;
            }
            let head_pos_u = (
                usize::try_from(head_pos.0).unwrap(),
                usize::try_from(head_pos.1).unwrap(),
            );

            // If it has already entered this position in this direction before,
            // it will continue along a path that we have already followed
            // before, so we can quit. Otherwise, we mark it in the bitfield.
            let direction_bit = head_direction as u8;
            if path[head_pos_u] & direction_bit > 0 {
                continue;
            }
            path[head_pos_u] |= direction_bit;

            self.follow_beam_step(
                head_pos,
                head_direction,
                self.grid[head_pos_u],
                &mut beam_heads,
            )
        }

        BeamPath(path)
    }

    fn follow_beam_step(
        &self,
        head_pos: (isize, isize),
        head_direction: Direction,
        element: u8,
        beam_heads: &mut Vec<((isize, isize), Direction)>,
    ) {
        match element {
            b'.' => beam_heads.push((head_direction.travel_from(head_pos), head_direction)),

            b'/' => {
                let (new_pos, new_direction) = match head_direction {
                    Direction::North => ((head_pos.0, head_pos.1 + 1), Direction::East),
                    Direction::East => ((head_pos.0 - 1, head_pos.1), Direction::North),
                    Direction::South => ((head_pos.0, head_pos.1 - 1), Direction::West),
                    Direction::West => ((head_pos.0 + 1, head_pos.1), Direction::South),
                };
                beam_heads.push((new_pos, new_direction));
            }

            b'\\' => {
                let (new_pos, new_direction) = match head_direction {
                    Direction::North => ((head_pos.0, head_pos.1 - 1), Direction::West),
                    Direction::East => ((head_pos.0 + 1, head_pos.1), Direction::South),
                    Direction::South => ((head_pos.0, head_pos.1 + 1), Direction::East),
                    Direction::West => ((head_pos.0 - 1, head_pos.1), Direction::North),
                };
                beam_heads.push((new_pos, new_direction));
            }

            b'|' => match head_direction {
                Direction::North => {
                    beam_heads.push((head_direction.travel_from(head_pos), head_direction))
                }
                Direction::East => {
                    beam_heads.push(((head_pos.0 - 1, head_pos.1), Direction::North));
                    beam_heads.push(((head_pos.0 + 1, head_pos.1), Direction::South));
                }
                Direction::South => {
                    beam_heads.push((head_direction.travel_from(head_pos), head_direction))
                }
                Direction::West => {
                    beam_heads.push(((head_pos.0 - 1, head_pos.1), Direction::North));
                    beam_heads.push(((head_pos.0 + 1, head_pos.1), Direction::South));
                }
            },

            b'-' => match head_direction {
                Direction::North => {
                    beam_heads.push(((head_pos.0, head_pos.1 + 1), Direction::East));
                    beam_heads.push(((head_pos.0, head_pos.1 - 1), Direction::West));
                }
                Direction::East => {
                    beam_heads.push((head_direction.travel_from(head_pos), head_direction))
                }
                Direction::South => {
                    beam_heads.push(((head_pos.0, head_pos.1 + 1), Direction::East));
                    beam_heads.push(((head_pos.0, head_pos.1 - 1), Direction::West));
                }
                Direction::West => {
                    beam_heads.push((head_direction.travel_from(head_pos), head_direction))
                }
            },

            _ => panic!("Invalid character"),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

impl Direction {
    fn travel_from(&self, start_pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (start_pos.0 - 1, start_pos.1),
            Direction::East => (start_pos.0, start_pos.1 + 1),
            Direction::South => (start_pos.0 + 1, start_pos.1),
            Direction::West => (start_pos.0, start_pos.1 - 1),
        }
    }
}

// Each byte is a bitfield with the bit for each direction set if a beam enters
// it in the corresponding direction.
pub struct BeamPath(Array2<u8>);

impl BeamPath {
    pub fn num_energized(&self) -> u32 {
        u32::try_from(self.0.iter().filter(|&&x| x > 0).count()).unwrap()
    }
}
