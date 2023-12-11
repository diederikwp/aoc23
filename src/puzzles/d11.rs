use std::{error::Error, str::FromStr};

pub struct Galaxies {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

impl FromStr for Galaxies {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        xs.sort();
        ys.sort();

        Ok(Galaxies { xs, ys })
    }
}

impl Galaxies {
    pub fn sum_pairwise_dist(&self) -> i64 {
        Self::sum_abs_diff(&self.xs) + Self::sum_abs_diff(&self.ys)
    }

    pub fn expand(&mut self, multiplier: u32) {
        Self::expand_arr(&mut self.xs, multiplier);
        Self::expand_arr(&mut self.ys, multiplier);
    }

    fn sum_abs_diff(nums: &[i64]) -> i64 {
        // Assuming xs and ys are sorted
        let n: i64 = nums.len().try_into().unwrap();
        nums.iter()
            .enumerate()
            .map(|(i, x)| {
                let i = i64::try_from(i).unwrap();
                x * (2 * i - n + 1)
            })
            .sum()
    }

    fn expand_arr(coords: &mut [i64], multiplier: u32) {
        if coords.is_empty() {
            return;
        }

        // Assuming coords are sorted
        let mut shift = 0;
        let mut last = coords[0];
        for num in coords {
            let step = *num - last;
            shift += i64::max(step - 1, 0) * (i64::try_from(multiplier).unwrap() - 1);
            last = *num;
            *num += shift;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{template::read_file, Day};

    use super::*;

    #[test]
    fn test_expansion_multipliers() {
        let input = &read_file("examples", Day::new(11).unwrap());

        let mut galaxies: Galaxies = input.parse().unwrap();
        galaxies.expand(10);
        assert_eq!(galaxies.sum_pairwise_dist(), 1030);

        let mut galaxies: Galaxies = input.parse().unwrap();
        galaxies.expand(100);
        assert_eq!(galaxies.sum_pairwise_dist(), 8410);
    }
}
