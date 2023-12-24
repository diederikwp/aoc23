use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

pub struct Garden {
    grid: Array2<u8>,
    start_pos: (usize, usize),
}

impl FromStr for Garden {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_idx = 0;
        let width = s.find('\n').unwrap_or(s.len());

        let linear_grid = Array::from_iter(
            s.bytes()
                .filter(|&b| b != b'\n')
                .enumerate()
                .inspect(|&(idx, b)| {
                    if b == b'S' {
                        start_idx = idx
                    }
                })
                .map(|(_idx, b)| b),
        );
        let height = linear_grid.len() / width;
        let grid = linear_grid.into_shape((height, width))?;
        let start_pos = (start_idx / height, start_idx % height);

        Ok(Garden { grid, start_pos })
    }
}

impl Garden {
    pub fn num_tiles_reacheable_after(&self, n_steps: u32, with_wrapping: bool) -> u32 {
        let start_pos = (
            i32::try_from(self.start_pos.0).unwrap(),
            i32::try_from(self.start_pos.1).unwrap(),
        );

        // Keep track of after how many steps a position was first reached in
        // `reached_after`. If a position was reached after n steps, it will be
        // reached again after n + 2, n + 4, n + 6, etc.
        let mut reached_after = HashMap::default();
        reached_after.insert(start_pos, 0);

        let mut curr_positions = HashSet::default();
        curr_positions.insert(start_pos);

        let get_neighbours = if with_wrapping {
            Self::neighbours_with_wrapping
        } else {
            Self::neighbours
        };

        for n in 0..n_steps {
            let mut next_positions = HashSet::default();

            for pos in curr_positions.into_iter() {
                let neighbours = get_neighbours(self, &pos);
                next_positions.extend(
                    neighbours
                        .iter()
                        .filter(|pos| !reached_after.contains_key(pos)),
                );
            }

            for pos in &next_positions {
                reached_after.insert(*pos, n + 1);
            }
            curr_positions = next_positions;
        }

        // if n_steps is even, we can reach all positions we reached after an
        // even number of steps.
        u32::try_from(
            reached_after
                .into_iter()
                .filter(|&(_pos, steps)| steps % 2 == n_steps % 2)
                .count(),
        )
        .unwrap()
    }

    fn neighbours_with_wrapping(&self, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let height = i32::try_from(self.grid.shape()[0]).unwrap();
        let width = i32::try_from(self.grid.shape()[1]).unwrap();

        [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .into_iter()
            .map(move |(dy, dx)| (pos.0 + dy, pos.1 + dx))
            .filter(|&neighbour_pos| {
                let wrapped_pos = (
                    neighbour_pos.0.rem_euclid(height),
                    neighbour_pos.1.rem_euclid(width),
                );
                let usize_pos = (
                    usize::try_from(wrapped_pos.0).unwrap(),
                    usize::try_from(wrapped_pos.1).unwrap(),
                );
                self.grid[usize_pos] != b'#'
            })
            .collect()
    }

    fn neighbours(&self, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let height = i32::try_from(self.grid.shape()[0]).unwrap();
        let width = i32::try_from(self.grid.shape()[1]).unwrap();

        [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .into_iter()
            .map(move |(dy, dx)| (pos.0 + dy, pos.1 + dx))
            .filter(move |&neighbour_pos| {
                neighbour_pos.0 >= 0
                    && neighbour_pos.0 < height
                    && neighbour_pos.1 >= 0
                    && neighbour_pos.1 < width
            })
            .filter(|&neighbour_pos| {
                let usize_pos = (
                    usize::try_from(neighbour_pos.0).unwrap(),
                    usize::try_from(neighbour_pos.1).unwrap(),
                );
                self.grid[usize_pos] != b'#'
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;

    #[test]
    fn test_num_tiles_reacheable_after_wo_wrapping() {
        let input = crate::template::read_file("examples", Day::new(21).unwrap());
        let garden: Garden = input.parse().unwrap();
        assert_eq!(garden.num_tiles_reacheable_after(6, false), 16);
    }

    #[test]
    fn test_num_tiles_reacheable_after_with_wrapping() {
        let input = crate::template::read_file("examples", Day::new(21).unwrap());
        let garden: Garden = input.parse().unwrap();

        assert_eq!(garden.num_tiles_reacheable_after(6, true), 16);
        assert_eq!(garden.num_tiles_reacheable_after(10, true), 50);
        assert_eq!(garden.num_tiles_reacheable_after(50, true), 1594);
        assert_eq!(garden.num_tiles_reacheable_after(100, true), 6536);
        assert_eq!(garden.num_tiles_reacheable_after(500, true), 167004);
        assert_eq!(garden.num_tiles_reacheable_after(1000, true), 668697);
        assert_eq!(garden.num_tiles_reacheable_after(5000, true), 16733044);
    }
}
