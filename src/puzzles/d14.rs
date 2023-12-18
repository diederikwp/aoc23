use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};

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
