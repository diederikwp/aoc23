use std::{error::Error, str::FromStr};

pub struct Report {
    histories: Vec<History>,
}

impl FromStr for Report {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut histories = Vec::new();
        for line in s.lines() {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()?;
            if nums.is_empty() {
                return Err("Each history should have > 0 numbers".into());
            }
            histories.push(History { nums })
        }
        Ok(Report { histories })
    }
}

impl Report {
    pub fn sum_extrapolated(&self) -> i32 {
        self.histories.iter().map(|h| h.pred_next()).sum()
    }
}

struct History {
    nums: Vec<i32>,
}

impl History {
    fn pred_next(&self) -> i32 {
        let mut next = *self.nums.last().unwrap();
        let mut diffs = Vec::with_capacity(self.nums.len() - 1);
        let mut prev_diffs = Vec::with_capacity(self.nums.len() - 2);

        let mut curr_series = &self.nums;
        loop {
            diffs.clear();
            let mut all_zeroes = true;
            for i in 1..curr_series.len() {
                let diff = curr_series[i] - curr_series[i - 1];
                diffs.push(diff);
                all_zeroes = all_zeroes && (diff == 0);
            }

            if all_zeroes {
                break;
            }

            next += diffs.last().unwrap();
            std::mem::swap(&mut diffs, &mut prev_diffs);
            curr_series = &prev_diffs;
        }

        next
    }
}
