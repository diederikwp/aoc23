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
    pub fn num_tiles_reacheable_after(&self, n_steps: u64, with_wrapping: bool) -> u64 {
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
        u64::try_from(
            reached_after
                .into_iter()
                .filter(|&(_pos, steps)| steps % 2 == n_steps % 2)
                .count(),
        )
        .unwrap()
    }

    /// More efficient version of num_tiles_reacheable_after for large n_steps.
    /// Only for the wrapping case.
    pub fn num_tiles_reacheable_extrapolated(&self, n_steps: u64) -> u64 {
        // In my puzzle input (and I suspect everyones input), there are
        // straight horizontal and vertical lines without rocks from S to the
        // edges, allowing the edge to be reached in 65 steps, and the next
        // repeated garden 131 steps after that. So after the first 65 steps,
        // the length of the outer perimeter that can be reached will grow
        // linearly with fixed increments every 131 steps, and therefore the
        // area inside it will grow quadratically every 131 steps.

        let offset = n_steps % 131;
        let n_periods = (n_steps - offset) / 131;
        if n_periods < 3 {
            return self.num_tiles_reacheable_after(n_steps, true);
        }

        let first_periods = [
            self.num_tiles_reacheable_after(offset, true),
            self.num_tiles_reacheable_after(offset + 131, true),
            self.num_tiles_reacheable_after(offset + 262, true),
        ];
        let diffdiff =
            (first_periods[2] - first_periods[1]) - (first_periods[1] - first_periods[0]);

        let mut curr = first_periods[2];
        let mut prev = first_periods[1];
        let mut total = first_periods[2];
        for _ in 2..n_periods {
            total += curr - prev + diffdiff;
            prev = curr;
            curr = total;
        }

        total
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
    }

    #[test]
    fn test_num_tiles_reacheable_extrapolated() {
        let input = crate::template::read_file("inputs", Day::new(21).unwrap());
        let garden: Garden = input.parse().unwrap();

        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(6),
            garden.num_tiles_reacheable_after(6, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(65),
            garden.num_tiles_reacheable_after(65, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(100),
            garden.num_tiles_reacheable_after(100, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(131),
            garden.num_tiles_reacheable_after(131, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(392),
            garden.num_tiles_reacheable_after(392, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(393),
            garden.num_tiles_reacheable_after(393, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(394),
            garden.num_tiles_reacheable_after(394, true)
        );
        assert_eq!(
            garden.num_tiles_reacheable_extrapolated(650),
            garden.num_tiles_reacheable_after(650, true)
        );
    }
}
