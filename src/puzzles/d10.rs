use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};

use self::tile_set::TileSet;

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
    pub fn loop_len(&self) -> u32 {
        self.iter_loop().count().try_into().unwrap()
    }

    pub fn enclosed_area(&self) -> u32 {
        // Idea: First expand the maze into 4x the area, to make sure that the
        // "squeezed" part of the outside consists of tiles in stead of space
        // between tiles. Then perform the flood fill algorithm to find
        // connected components. Any component containing an edge of the field
        // is outside, other components are inside.

        let enlarged = self.cleaned().enlarged();
        let n = enlarged.tiles.shape()[0];
        let mut n_enclosed = 0;

        let mut unvisited_tiles = TileSet::new((n, n));
        for (pos, c) in enlarged.tiles.indexed_iter() {
            if *c == '.' || *c == '*' {
                unvisited_tiles.push(pos);
            }
        }

        while unvisited_tiles.len() > 0 {
            // Use any unvisited tile to seed the next flood fill.
            let mut frontier = vec![unvisited_tiles.pop()]; // Tiles in this connected component
            let mut inside = true; // Whether this component is inside or outside
            let mut n_dots = 0; // Number of '.' tiles in this component

            while let Some((y, x)) = frontier.pop() {
                let (y, x) = (y as i32, x as i32);
                let neighbours = [(y - 1, x), (y + 1, x), (y, x + 1), (y, x - 1)];
                for pos in neighbours {
                    if !enlarged.pos_is_in_field(pos) {
                        continue;
                    }
                    let pos = (pos.0 as usize, pos.1 as usize);

                    if unvisited_tiles.contains(&pos) && ".*".contains(enlarged.tiles[pos]) {
                        frontier.push(pos);
                        unvisited_tiles.remove(&pos);

                        if enlarged.tiles[pos] == '.' {
                            n_dots += 1;
                        }
                        if enlarged.pos_is_on_edge(pos) {
                            inside = false
                        }
                    }
                }
            }

            if inside {
                n_enclosed += n_dots;
            }
        }

        n_enclosed
    }

    fn pos_is_in_field(&self, pos: (i32, i32)) -> bool {
        let n = self.tiles.shape()[0] as i32;
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < n && pos.1 < n
    }

    fn pos_is_on_edge(&self, pos: (usize, usize)) -> bool {
        let n = self.tiles.shape()[0];
        pos.0 == 0 || pos.1 == 0 || pos.0 == n - 1 || pos.1 == n - 1
    }

    fn iter_loop(&self) -> MazeIter<'_> {
        MazeIter::new(self)
    }

    fn cleaned(&self) -> Self {
        let shape = self.tiles.raw_dim();
        let mut cleaned = Array::from_elem(shape, '.');

        for (pos, tile) in self.iter_loop() {
            cleaned[pos] = tile
        }

        Maze {
            tiles: cleaned,
            s_pos: self.s_pos,
        }
    }

    fn enlarged(&self) -> Self {
        // Replace every tile by four tiles, using '*' as filler. E.g.
        // F becomes
        // F-
        // |*
        let n = self.tiles.shape()[0];
        let mut enlarged = Array2::from_elem((n * 2, n * 2), '*');

        for ((y, x), c) in self.tiles.indexed_iter() {
            match c {
                '|' => {
                    enlarged[(y * 2, x * 2)] = '|';
                    enlarged[(y * 2 + 1, x * 2)] = '|';
                }
                '-' => {
                    enlarged[(y * 2, x * 2)] = '-';
                    enlarged[(y * 2, x * 2 + 1)] = '|';
                }
                'L' => {
                    enlarged[(y * 2, x * 2)] = 'L';
                    enlarged[(y * 2, x * 2 + 1)] = '-';
                }
                'J' => {
                    enlarged[(y * 2, x * 2)] = 'J';
                }
                '7' => {
                    enlarged[(y * 2, x * 2)] = '7';
                    enlarged[(y * 2 + 1, x * 2)] = '|';
                }
                'F' => {
                    enlarged[(y * 2, x * 2)] = 'F';
                    enlarged[(y * 2 + 1, x * 2)] = '|';
                    enlarged[(y * 2, x * 2 + 1)] = '-';
                }
                '.' => {
                    enlarged[(y * 2, x * 2)] = '.';
                }
                'S' => {
                    enlarged[(y * 2, x * 2)] = 'S';
                    if x < n - 1 && "-J7".contains(self.tiles[(y, x + 1)]) {
                        enlarged[(y * 2, x * 2 + 1)] = '-';
                    }
                    if y < n - 1 && "|LJ".contains(self.tiles[(y + 1, x)]) {
                        enlarged[(y * 2 + 1, x * 2)] = '|';
                    }
                    if x > 0 && "-LF".contains(self.tiles[(y, x - 1)]) {
                        enlarged[(y * 2, x * 2 - 1)] = '-';
                    }
                    if y > 0 && "|7F".contains(self.tiles[(y - 1, x)]) {
                        enlarged[(y * 2 - 1, x * 2)] = '|';
                    }
                }
                _ => panic!("Invalid character"),
            }
        }

        Maze {
            tiles: enlarged,
            s_pos: (self.s_pos.0 * 2, self.s_pos.1 * 2),
        }
    }
}

struct MazeIter<'a> {
    maze: &'a Maze,
    pos: (usize, usize),
    dx: i32,
    dy: i32,
    finished: bool,
}

impl<'a> MazeIter<'a> {
    fn new(maze: &'a Maze) -> Self {
        let n = maze.tiles.shape()[0];
        let (x, y) = maze.s_pos;

        // Determine starting direction: pick any valid direction. If can go up,
        // go up, if can go right, go right, etc.
        let (dx, dy): (i32, i32);
        if y > 0 && "|7F".contains(maze.tiles[(y - 1, x)]) {
            dx = 0;
            dy = -1;
        } else if x < n - 1 && "-J7".contains(maze.tiles[(y, x + 1)]) {
            dx = 1;
            dy = 0;
        } else if y < n - 1 && "|LJ".contains(maze.tiles[(y + 1, x)]) {
            dx = 0;
            dy = 1;
        } else {
            dx = -1;
            dy = 0;
        }

        MazeIter {
            maze,
            pos: (y, x),
            dx,
            dy,
            finished: false,
        }
    }
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = ((usize, usize), char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let orig_pos = self.pos;

        self.pos.0 = (self.pos.0 as i32 + self.dy) as usize;
        self.pos.1 = (self.pos.1 as i32 + self.dx) as usize;

        match self.maze.tiles[self.pos] {
            '|' => (),
            '-' => (),
            'L' => {
                self.dy -= 1;
                self.dx += 1;
            }
            'J' => {
                self.dy -= 1;
                self.dx -= 1;
            }
            '7' => {
                self.dy += 1;
                self.dx -= 1;
            }
            'F' => {
                self.dy += 1;
                self.dx += 1;
            }
            '.' => panic!("Ended up outside of loop"),
            'S' => {
                self.finished = true;
            }
            _ => panic!("Invalid character"),
        }

        Some((orig_pos, self.maze.tiles[orig_pos]))
    }
}

mod tile_set {
    use ndarray::{Array, Array2};

    pub struct TileSet {
        tiles: Array2<bool>,
        len: usize,
    }

    impl TileSet {
        pub fn new(shape: (usize, usize)) -> Self {
            let tiles = Array::from_elem(shape, false);
            TileSet { tiles, len: 0 }
        }

        pub fn contains(&self, pos: &(usize, usize)) -> bool {
            self.tiles[*pos]
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn push(&mut self, pos: (usize, usize)) {
            if !self.contains(&pos) {
                self.tiles[pos] = true;
                self.len += 1;
            }
        }

        pub fn pop(&mut self) -> (usize, usize) {
            let pos = self
                .tiles
                .indexed_iter()
                .filter(|(_, &present)| present)
                .map(|(pos, _)| pos)
                .next()
                .unwrap();
            self.tiles[pos] = false;
            self.len -= 1;

            pos
        }

        pub fn remove(&mut self, pos: &(usize, usize)) {
            if self.contains(pos) {
                self.len -= 1;
                self.tiles[*pos] = false;
            }
        }
    }
}
