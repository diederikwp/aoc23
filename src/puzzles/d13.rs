use std::{error::Error, str::FromStr};

use ndarray::{s, Array, Array2};

pub struct Valley {
    patterns: Vec<Pattern>,
}

impl FromStr for Valley {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = Vec::new();
        for pattern_slice in s.split("\n\n") {
            patterns.push(pattern_slice.parse()?);
        }

        Ok(Valley { patterns })
    }
}

impl Valley {
    pub fn sum_symmetry_score(&self) -> u32 {
        self.patterns.iter().map(|p| p.symmetry_score()).sum()
    }
}

struct Pattern {
    grid: Array2<u8>,
}

impl FromStr for Pattern {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let height = s.lines().count();
        let grid =
            Array::from_iter(s.bytes().filter(|&b| b != b'\n')).into_shape((height, width))?;

        Ok(Pattern { grid })
    }
}

impl Pattern {
    fn symmetry_score(&self) -> u32 {
        let height = self.grid.shape()[0];
        let width = self.grid.shape()[1];

        for row_idx in 0..(height - 1) {
            if self.has_horizontal_symmetry_at(row_idx) {
                return 100 * (u32::try_from(row_idx).unwrap() + 1);
            }
        }

        for col_idx in 0..(width - 1) {
            if self.has_vertical_symmetry_at(col_idx) {
                return u32::try_from(col_idx).unwrap() + 1;
            }
        }

        0
    }

    fn has_horizontal_symmetry_at(&self, row_idx: usize) -> bool {
        let height = self.grid.shape()[0];

        for delta in 0..usize::min(row_idx + 1, height - row_idx - 1) {
            let row_before = self.grid.slice(s![row_idx - delta, ..]);
            let row_after = self.grid.slice(s![row_idx + delta + 1, ..]);
            if row_before != row_after {
                return false;
            }
        }

        true
    }

    fn has_vertical_symmetry_at(&self, col_idx: usize) -> bool {
        let width = self.grid.shape()[1];

        for delta in 0..usize::min(col_idx + 1, width - col_idx - 1) {
            let col_before = self.grid.slice(s![.., col_idx - delta]);
            let col_after = self.grid.slice(s![.., col_idx + delta + 1]);
            if col_before != col_after {
                return false;
            }
        }

        true
    }
}
