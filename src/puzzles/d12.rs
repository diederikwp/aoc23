use std::{error::Error, str::FromStr};

use ndarray::Array2;

pub struct Field {
    springs: Vec<Springs>,
}

impl FromStr for Field {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Springs>, _>>()?;
        Ok(Field { springs })
    }
}

impl Field {
    pub fn total_arrangement_count(&self) -> u64 {
        self.springs.iter().map(|s| s.arrangement_count()).sum()
    }

    pub fn total_arrangement_count_extended(&self) -> u64 {
        self.springs
            .iter()
            .map(|s| s.extend().arrangement_count())
            .sum()
    }
}

struct Springs {
    row: Vec<u8>,       // e.g. "???.###"
    groups: Vec<usize>, // e.g. [1, 1, 3]
}

impl FromStr for Springs {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row_str, groups_str) = s.split_once(' ').ok_or("Expected 2 parts")?;

        // Prepend a '.' to the row
        let mut row = Vec::with_capacity(row_str.len() + 1);
        row.push(b'.');
        row.extend_from_slice(row_str.as_bytes());

        let groups = groups_str
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Springs { row, groups })
    }
}

impl Springs {
    fn arrangement_count(&self) -> u64 {
        // counts[(i, j)] holds the number of possible arrangements for groups[..i] and row[..j]
        let n = self.groups.len() + 1;
        let m = self.row.len() + 1;
        let mut counts = Array2::zeros((n, m));

        // Init first row
        let first_spring = self
            .row
            .iter()
            .enumerate()
            .find(|&(_, c)| *c == b'#')
            .map(|(idx, _)| idx)
            .unwrap_or(m - 1);
        for j in 0..=first_spring {
            counts[(0, j)] = 1;
        }

        // Fill the remainder recursively
        for i in 1..n {
            for j in 1..m {
                counts[(i, j)] = self.calc_single_count(i, j, &counts);
            }
        }

        counts[(n - 1, m - 1)]
    }

    fn calc_single_count(&self, i: usize, j: usize, counts: &Array2<u64>) -> u64 {
        let row_char = self.row[j - 1];
        let group_len = self.groups[i - 1];

        let mut answer = 0;

        if row_char == b'.' || row_char == b'?' {
            answer += counts[(i, j - 1)];
        }

        if j > group_len && (row_char == b'#' || row_char == b'?') {
            let n_springs =
                // Last group_len chars should be springs
                (self.row[(j - group_len)..j]
                .iter()
                .all(|c| [b'?', b'#'].contains(c)))
                // the one before that should not
                && [b'?', b'.'].contains(&self.row[j - group_len - 1]);

            if n_springs {
                answer += counts[(i - 1, j - group_len - 1)]
            }
        }

        answer
    }

    fn extend(&self) -> Self {
        let mut extended_row = Vec::with_capacity(self.row.len() * 5);
        extended_row.extend_from_slice(&self.row);
        for _ in 0..4 {
            extended_row.push(b'?');
            extended_row.extend_from_slice(&self.row[1..]); // skip the prepended '.'
        }

        let mut extended_groups = Vec::with_capacity(self.groups.len() * 5);
        for _ in 0..5 {
            extended_groups.extend_from_slice(&self.groups)
        }

        Springs {
            row: extended_row,
            groups: extended_groups,
        }
    }
}
