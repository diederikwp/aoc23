use std::{error::Error, iter::zip, str::FromStr};

pub struct BoatTable {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl FromStr for BoatTable {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .ok_or("expected 2 lines")?
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse())
            .collect::<Result<Vec<u32>, _>>()?;
        let distances = lines
            .next()
            .ok_or("expected 2 lines")?
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse())
            .collect::<Result<Vec<u32>, _>>()?;

        Ok(BoatTable { times, distances })
    }
}

impl BoatTable {
    pub fn n_ways_to_win(&self) -> u32 {
        zip(&self.times, &self.distances)
            .map(|(t, d)| {
                let (t_f, d_f) = (f64::from(*t), f64::from(*d));
                let lower = ((t_f - f64::sqrt(t_f * t_f - 4.0 * d_f)) / 2.0).floor() as u32 + 1;
                let upper = t - lower;
                upper - lower + 1
            })
            .product()
    }
}
