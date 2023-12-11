use std::{error::Error, str::FromStr};

pub struct Galaxies {
    xs: Vec<i32>,
    ys: Vec<i32>,
}

impl FromStr for Galaxies {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Gather coords
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    xs.push(x.try_into()?);
                    ys.push(y.try_into()?);
                }
            }
        }

        // Sort
        xs.sort();
        ys.sort();

        // Expand
        Self::expand(&mut xs);
        Self::expand(&mut ys);

        Ok(Galaxies { xs, ys })
    }
}

impl Galaxies {
    pub fn sum_pairwise_dist(&self) -> i32 {
        Self::sum_abs_diff(&self.xs) + Self::sum_abs_diff(&self.ys)
    }

    fn sum_abs_diff(nums: &[i32]) -> i32 {
        // Assuming xs and ys are sorted
        let n: i32 = nums.len().try_into().unwrap();
        nums.iter()
            .enumerate()
            .map(|(i, x)| {
                let i = i32::try_from(i).unwrap();
                x * (2 * i - n + 1)
            })
            .sum()
    }

    fn expand(coords: &mut [i32]) {
        if coords.is_empty() {
            return;
        }

        // Assuming coords are sorted
        let mut shift = 0;
        let mut last = coords[0];
        for num in coords {
            let step = *num - last;
            shift += i32::max(step - 1, 0);
            last = *num;
            *num += shift;
        }
    }
}
