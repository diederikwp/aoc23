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

pub struct BoatRace {
    time: u64,
    distance: u64,
}

impl FromStr for BoatRace {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let time_str: String = lines
            .next()
            .ok_or("expected 2 lines")?
            .split_whitespace()
            .skip(1)
            .collect();

        let dist_str: String = lines
            .next()
            .ok_or("expected 2 lines")?
            .split_whitespace()
            .skip(1)
            .collect();

        let distance: u64 = dist_str.parse()?;
        let time: u64 = time_str.parse()?;

        Ok(BoatRace { time, distance })
    }
}

impl BoatRace {
    pub fn n_ways_to_win(&self) -> u64 {
        // Exponential search past lowest possible button hold
        let mut max = 1;
        while max * (self.time - max) <= self.distance {
            max *= 2;
        }

        // Binary search for lowest possible button hold
        let (mut low, mut high) = (0, max);

        while high - low > 1 {
            let mid = (high - low) / 2 + low;

            if mid * (self.time - mid) > self.distance {
                high = mid;
            } else {
                low = mid;
            }
        }

        self.time - 2 * high + 1
    }
}
