use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::FxHashMap;

pub struct Platform {
    grid: Array2<u8>,
}

impl FromStr for Platform {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let height = s.lines().count();
        let grid =
            Array::from_iter(s.bytes().filter(|&b| b != b'\n')).into_shape((height, width))?;

        Ok(Platform { grid })
    }
}

impl Platform {
    pub fn slide_north(&mut self) {
        for mut col in self.grid.columns_mut() {
            let mut to_idx = 0;
            for i in 0..col.len() {
                match col[i] {
                    b'#' => to_idx = i + 1,
                    b'O' => {
                        col[i] = b'.';
                        col[to_idx] = b'O';
                        to_idx += 1;
                    }
                    b'.' => (),
                    _ => panic!("Illegal character"),
                }
            }
        }
    }

    pub fn slide_east(&mut self) {
        for mut row in self.grid.rows_mut() {
            let mut to_idx = row.len() - 1;
            for i in (0..row.len()).rev() {
                match row[i] {
                    b'#' => to_idx = i.saturating_sub(1),
                    b'O' => {
                        row[i] = b'.';
                        row[to_idx] = b'O';
                        to_idx = to_idx.saturating_sub(1);
                    }
                    b'.' => (),
                    _ => panic!("Illegal character"),
                }
            }
        }
    }

    pub fn slide_south(&mut self) {
        for mut col in self.grid.columns_mut() {
            let mut to_idx = col.len() - 1;
            for i in (0..col.len()).rev() {
                match col[i] {
                    b'#' => to_idx = i.saturating_sub(1),
                    b'O' => {
                        col[i] = b'.';
                        col[to_idx] = b'O';
                        to_idx = to_idx.saturating_sub(1);
                    }
                    b'.' => (),
                    _ => panic!("Illegal character"),
                }
            }
        }
    }

    pub fn slide_west(&mut self) {
        for mut row in self.grid.rows_mut() {
            let mut to_idx = 0;
            for i in 0..row.len() {
                match row[i] {
                    b'#' => to_idx = i + 1,
                    b'O' => {
                        row[i] = b'.';
                        row[to_idx] = b'O';
                        to_idx += 1;
                    }
                    b'.' => (),
                    _ => panic!("Illegal character"),
                }
            }
        }
    }

    pub fn spin(&mut self, n_iter: usize) {
        let mut history: FxHashMap<Array2<u8>, usize> = FxHashMap::default();

        for i in 0..n_iter {
            self.slide_north();
            self.slide_west();
            self.slide_south();
            self.slide_east();

            if let Some(n) = history.get(&self.grid) {
                // It will repeat itself every i - n cycles.
                let period = i - n;
                let value_final = (n_iter - 1 - n) % period + n;

                for (key, &value) in &history {
                    if value == value_final {
                        self.grid = key.clone();
                        return;
                    }
                }
            }

            history.insert(self.grid.clone(), i);
        }
    }

    pub fn total_load(&self) -> u32 {
        let height = self.grid.shape()[0];
        let width = self.grid.shape()[1];

        let mut load = 0;
        for (idx, &elem) in self.grid.iter().enumerate() {
            if elem == b'O' {
                let row = idx / width;
                load += height - row
            }
        }

        u32::try_from(load).unwrap()
    }
}
