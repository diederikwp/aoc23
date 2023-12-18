use std::{error::Error, str::FromStr};

pub struct InitSequence {
    steps: Vec<Vec<u8>>,
}

impl FromStr for InitSequence {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .split(',')
            .map(|step| step.bytes().filter(|&c| c != b'\n').collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();

        Ok(InitSequence { steps })
    }
}

impl InitSequence {
    pub fn sum_hashes(&self) -> u32 {
        self.steps.iter().map(|s| Self::hash(s)).sum()
    }

    fn hash(bytes: &[u8]) -> u32 {
        let mut val: u32 = 0;
        for &b in bytes {
            val += u32::from(b);
            val *= 17;
            val %= 256;
        }

        val
    }
}
