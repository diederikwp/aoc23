use std::{error::Error, str::FromStr};

pub struct CardGame {
    winning_nums: Vec<u8>,
    nums: Vec<u8>,
}

impl CardGame {
    pub fn points(&self) -> u32 {
        2_u32.pow(self.n_matching()) / 2
    }

    pub fn n_matching(&self) -> u32 {
        self.nums
            .iter()
            // alternatives to `contains`:
            // - hashset (probably slower for such a short vec)
            // - sorting winning_nums upon creation and using binary search
            .filter(|x| self.winning_nums.contains(x))
            .count()
            .try_into()
            .unwrap()
    }
}

impl FromStr for CardGame {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, str_nums) = s.split_once(": ").ok_or("Invalid syntax")?;
        let (winning_nums_part, nums_part) = str_nums.split_once(" | ").ok_or("Invalid syntax")?;

        let mut winning_nums = Vec::new();
        for num in winning_nums_part.split_whitespace() {
            winning_nums.push(num.parse()?);
        }

        let mut nums = Vec::new();
        for num in nums_part.split_whitespace() {
            nums.push(num.parse()?);
        }

        Ok(CardGame { winning_nums, nums })
    }
}
