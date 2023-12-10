use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};

pub struct Maze {
    tiles: Array2<char>,
    s_pos: (usize, usize),
}

impl FromStr for Maze {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Assume square
        let n = s.find('\n').ok_or("No newline")?;
        let mut s_pos = (0, 0);
        let tiles = Array::from_iter(s.chars().filter(|c| !c.is_whitespace()).enumerate().map(
            |(i, c)| {
                if c == 'S' {
                    s_pos = (i % n, i / n);
                }
                c
            },
        ))
        .into_shape((n, n))?;

        Ok(Maze { tiles, s_pos })
    }
}

impl Maze {
    pub fn loop_len(&self) -> i32 {
        let n = self.tiles.shape()[0];
        let (mut x, mut y) = self.s_pos;

        // Determine starting direction: pick any valid direction. If can go up,
        // go up, if can go right, go right, etc.
        let (mut dx, mut dy): (i32, i32);
        if y > 0 && "|7F".contains(self.tiles[(y - 1, x)]) {
            dx = 0;
            dy = -1;
        } else if x < n - 1 && "-J7".contains(self.tiles[(y, x + 1)]) {
            dx = 1;
            dy = 0;
        } else if y < n - 1 && "|LJ".contains(self.tiles[(y + 1, x)]) {
            dx = 0;
            dy = 1;
        } else {
            dx = -1;
            dy = 0;
        }

        // Walk until back at S
        let mut len = 1;
        while self.tiles[((y as i32 + dy) as usize, (x as i32 + dx) as usize)] != 'S' {
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;

            match self.tiles[(y, x)] {
                '|' => (),
                '-' => (),
                'L' => {
                    dy -= 1;
                    dx += 1;
                }
                'J' => {
                    dy -= 1;
                    dx -= 1;
                }
                '7' => {
                    dy += 1;
                    dx -= 1;
                }
                'F' => {
                    dy += 1;
                    dx += 1;
                }
                '.' => panic!("Ended up outside of loop"),
                _ => panic!("Invalid character"),
            }
            len += 1;
        }

        len
    }
}
