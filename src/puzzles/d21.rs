use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};
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
    pub fn num_tiles_reacheable_after(&self, n_steps: u32) -> u32 {
        let mut reacheable = HashSet::default();
        reacheable.insert(self.start_pos);

        for _ in 0..n_steps {
            let mut new_reacheable = HashSet::default();
            for pos in reacheable.into_iter() {
                new_reacheable.extend(self.neighbours(&pos));
            }
            reacheable = new_reacheable;
        }

        u32::try_from(reacheable.len()).unwrap()
    }

    fn neighbours(&self, pos: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let pos = (i32::try_from(pos.0).unwrap(), i32::try_from(pos.1).unwrap());

        [(0, 1), (-1, 0), (0, -1), (1, 0)]
            .into_iter()
            .map(move |(dy, dx)| {
                (
                    usize::try_from(pos.0 + dy).unwrap(),
                    usize::try_from(pos.1 + dx).unwrap(),
                )
            })
            .filter(|&neighbour_pos| {
                self.grid
                    .get(neighbour_pos)
                    .is_some_and(|&tile| tile != b'#')
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;

    #[test]
    fn test_num_tiles_reacheable_after() {
        let input = crate::template::read_file("examples", Day::new(21).unwrap());
        let garden: Garden = input.parse().unwrap();
        assert_eq!(garden.num_tiles_reacheable_after(6), 16);
    }
}
