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
    pub fn sum_extrapolated(self) -> i32 {
        self.histories.into_iter().map(|h| h.pred_next()).sum()
    }

    pub fn sum_extrapolated_backwards(self) -> i32 {
        self.histories.into_iter().map(|h| h.pred_prev()).sum()
    }
}

struct History {
    nums: Vec<i32>,
}

impl History {
    fn pred_next(self) -> i32 {
        let mut next = *self.nums.last().unwrap();
        let mut hist_diffs = HistoryDiffs::new(self);
        while let Some(diffs) = hist_diffs.next_diffs() {
            next += diffs.last().unwrap();
        }

        next
    }

    fn pred_prev(self) -> i32 {
        let mut prev = *self.nums.first().unwrap();
        let mut mult = 1;
        let mut hist_diffs = HistoryDiffs::new(self);
        while let Some(diffs) = hist_diffs.next_diffs() {
            mult *= -1;
            prev += mult * diffs.first().unwrap();
        }

        prev
    }
}

struct HistoryDiffs {
    diffs: Vec<i32>,
    prev_diffs: Vec<i32>,
}

impl HistoryDiffs {
    fn new(history: History) -> Self {
        let diffs = Vec::with_capacity(history.nums.len() - 1);
        let prev_diffs = history.nums;

        HistoryDiffs { diffs, prev_diffs }
    }

    fn next_diffs(&mut self) -> Option<&[i32]> {
        self.diffs.clear();
        let mut all_zeroes = true;
        for i in 1..self.prev_diffs.len() {
            let diff = self.prev_diffs[i] - self.prev_diffs[i - 1];
            self.diffs.push(diff);
            all_zeroes = all_zeroes && (diff == 0);
        }

        if all_zeroes {
            return None;
        }

        std::mem::swap(&mut self.diffs, &mut self.prev_diffs);

        Some(&self.prev_diffs)
    }
}
