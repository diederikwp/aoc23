use std::{error::Error, str::FromStr};

pub struct InitSequence {
    steps: Vec<Instruction>,
}

impl FromStr for InitSequence {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .split(',')
            .map(|step| step.parse())
            .collect::<Result<Vec<Instruction>, _>>()?;

        Ok(InitSequence { steps })
    }
}

impl InitSequence {
    pub fn sum_hashes(&self) -> u32 {
        self.steps
            .iter()
            .map(|s| u32::from(s.hash_instruction()))
            .sum()
    }

    pub fn total_resulting_power(&self) -> u32 {
        let mut boxes = Boxes::new(256);
        for instruction in &self.steps {
            boxes.apply(instruction);
        }

        boxes.total_focusing_power()
    }
}

struct Instruction {
    txt: String,
    label_len: usize,
    operation: Operation,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let txt = s.chars().filter(|&c| c != '\n').collect();
        let label_len;
        let operation;

        if let Some(pos) = s.find('=') {
            let focal_length: u8 = s.as_bytes()[pos + 1] - b'0';
            operation = Operation::Place(focal_length);
            label_len = pos;
        } else if let Some(pos) = s.find('-') {
            operation = Operation::Remove;
            label_len = pos;
        } else {
            return Err("Invalid instruction".into());
        }

        Ok(Instruction {
            txt,
            label_len,
            operation,
        })
    }
}

impl Instruction {
    fn label(&self) -> &str {
        &self.txt[..self.label_len]
    }

    fn hash_instruction(&self) -> u8 {
        Self::hash_bytes(self.txt.as_bytes())
    }

    fn hash_label(&self) -> u8 {
        Self::hash_bytes(self.label().as_bytes())
    }

    fn hash_bytes(bytes: &[u8]) -> u8 {
        let mut val: u32 = 0;
        for &b in bytes {
            val += u32::from(b);
            val *= 17;
            val %= 256;
        }

        u8::try_from(val).unwrap()
    }
}

enum Operation {
    Place(u8),
    Remove,
}

struct Boxes(Vec<LensBox>);

impl Boxes {
    fn new(n: usize) -> Self {
        Self(vec![LensBox(Vec::new()); n])
    }

    fn apply(&mut self, instruction: &Instruction) {
        let this_box = &mut self.0[usize::from(instruction.hash_label())];

        match instruction.operation {
            Operation::Place(focal_length) => {
                if let Some(pos) = this_box
                    .0
                    .iter()
                    .position(|entry| entry.label == instruction.label())
                {
                    this_box.0[pos].focal_length = focal_length;
                } else {
                    this_box.0.push(LensBoxEntry {
                        label: instruction.label().to_string(),
                        focal_length,
                    })
                }
            }

            Operation::Remove => {
                if let Some(pos) = this_box
                    .0
                    .iter()
                    .position(|entry| entry.label == instruction.label())
                {
                    this_box.0.remove(pos);
                }
            }
        }
    }

    fn total_focusing_power(&self) -> u32 {
        let total: usize = self
            .0
            .iter()
            .enumerate()
            .map(|(idx_box, lens_box)| {
                let box_power: usize = lens_box
                    .0
                    .iter()
                    .enumerate()
                    .map(|(idx_lens, entry)| (idx_lens + 1) * usize::from(entry.focal_length))
                    .sum();
                box_power * (idx_box + 1)
            })
            .sum();

        u32::try_from(total).unwrap()
    }
}

#[derive(Clone)]
struct LensBox(Vec<LensBoxEntry>);

#[derive(Clone)]
struct LensBoxEntry {
    label: String,
    focal_length: u8,
}
