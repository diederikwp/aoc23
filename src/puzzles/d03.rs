use std::collections::HashSet;

use self::tok::{TokenKind, Tokenizer};
use ndarray::Array2;

pub struct Schematic {
    grid: Array2<GridSlot>,
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

impl Schematic {
    pub fn new(text: &str) -> Self {
        let tokenizer = Tokenizer::new(text);

        // Assume square grid. This is not in the puzzle spec but true for the
        // example and my input.
        let mut grid = Array2::from_elem(
            (tokenizer.line_len(), tokenizer.line_len()),
            GridSlot::Empty,
        );
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();

        let mut idx_symbol = 0;
        let mut idx_number = 0;
        for token in tokenizer {
            match token.kind {
                TokenKind::SymTok => {
                    let symbol = Symbol {
                        idx: idx_symbol,
                        x: token.x,
                        y: token.y,
                        char: token.txt.chars().nth(0).unwrap(),
                    };
                    symbols.push(symbol);

                    grid[[token.x, token.y]] = GridSlot::Symbol(idx_symbol);
                    idx_symbol += 1;
                }

                TokenKind::NumTok => {
                    let number = Number {
                        idx: idx_number,
                        val: token.txt.parse().unwrap(),
                    };
                    numbers.push(number);

                    for x in token.x..(token.x + token.txt.len()) {
                        grid[[x, token.y]] = GridSlot::Number(idx_number);
                    }
                    idx_number += 1;
                }
            }
        }

        Schematic {
            grid,
            symbols,
            numbers,
        }
    }

    pub fn grid(&self) -> &Array2<GridSlot> {
        &self.grid
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    pub fn numbers(&self) -> &[Number] {
        &self.numbers
    }

    pub fn select_part_number_idxs(&self) -> Vec<bool> {
        let mut selected_nums = vec![false; self.numbers().len()];

        // Mark all numbers adjacent to a symbol in `selected_nums`
        for sym in self.symbols() {
            for (x, y) in Self::get_adjacent_coords(sym.x, sym.y) {
                if let Some(GridSlot::Number(i)) = self.grid.get([x, y]) {
                    selected_nums[*i] = true;
                }
            }
        }

        selected_nums
    }

    pub fn total_gear_ratio(&self) -> u32 {
        let mut total_gear_ratio = 0;
        let part_num_selection = self.select_part_number_idxs();

        for sym in self.symbols() {
            let mut adjacent_part_idxs = HashSet::new();

            for (x, y) in Self::get_adjacent_coords(sym.x, sym.y) {
                if let Some(GridSlot::Number(i)) = self.grid.get([x, y]) {
                    if part_num_selection[*i] {
                        adjacent_part_idxs.insert(i);
                    }
                }
            }

            if adjacent_part_idxs.len() == 2 {
                total_gear_ratio += adjacent_part_idxs
                    .iter()
                    .map(|&i| self.numbers()[*i].val)
                    .product::<u32>()
            }
        }

        total_gear_ratio
    }

    fn get_adjacent_coords(x: usize, y: usize) -> [(usize, usize); 8] {
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GridSlot {
    Empty,
    Symbol(usize),
    Number(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol {
    pub idx: usize,
    pub x: usize,
    pub y: usize,
    pub char: char,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Number {
    pub idx: usize,
    pub val: u32,
}

mod tok {
    use std::iter::Peekable;
    use std::str::CharIndices;

    pub struct Tokenizer<'a> {
        txt: &'a str,
        line_len: usize,
        char_idxs: Peekable<CharIndices<'a>>,
        x: usize,
        y: usize,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(txt: &'a str) -> Self {
            let line_len = txt.find('\n').unwrap();
            Tokenizer {
                txt,
                line_len,
                char_idxs: txt.char_indices().peekable(),
                x: 0,
                y: 0,
            }
        }

        pub fn line_len(&self) -> usize {
            self.line_len
        }

        // Any of the consume_* functions will panic if there is no next character.

        fn consume_period(&mut self) {
            self.char_idxs.next();
            self.x += 1;
        }

        fn consume_newline(&mut self) {
            self.char_idxs.next();
            self.y += 1;
            self.x = 0;
        }

        fn consume_numeric(&mut self) -> Token<'a> {
            let startpos = self.char_idxs.next().unwrap().0;
            let mut len = 1;

            while let Some((_, d)) = self.char_idxs.peek() {
                if d.is_numeric() {
                    len += 1;
                    self.char_idxs.next();
                } else {
                    break;
                }
            }

            let tok = Token {
                kind: TokenKind::NumTok,
                x: self.x,
                y: self.y,
                txt: &self.txt[startpos..(startpos + len)],
            };
            self.x += len;
            tok
        }

        fn consume_symbol(&mut self) -> Token<'a> {
            let (pos, _) = self.char_idxs.next().unwrap();
            let tok = Token {
                kind: TokenKind::SymTok,
                x: self.x,
                y: self.y,
                txt: &self.txt[pos..(pos + 1)],
            };
            self.x += 1;
            tok
        }
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some((_, c)) = self.char_idxs.peek() {
                match c {
                    '.' => self.consume_period(),
                    '\n' => self.consume_newline(),
                    d if d.is_numeric() => return Some(self.consume_numeric()),
                    _ => return Some(self.consume_symbol()),
                }
            }
            None
        }
    }

    pub struct Token<'a> {
        pub kind: TokenKind,
        pub x: usize,
        pub y: usize,
        pub txt: &'a str,
    }

    pub enum TokenKind {
        SymTok,
        NumTok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day;
    use crate::template::read_file;

    #[test]
    fn test_new_schematic() {
        let schematic = Schematic::new(&read_file("examples", day!(3)));

        assert_eq!(schematic.grid[[0, 0]], GridSlot::Number(0));
        assert_eq!(schematic.grid[[1, 0]], GridSlot::Number(0));
        assert_eq!(schematic.grid[[2, 0]], GridSlot::Number(0));
        assert_eq!(schematic.grid[[3, 0]], GridSlot::Empty);
        assert_eq!(schematic.grid[[4, 0]], GridSlot::Empty);
        assert_eq!(schematic.grid[[5, 0]], GridSlot::Number(1));
        assert_eq!(schematic.grid[[6, 3]], GridSlot::Symbol(1));

        let expected_numbers = vec![
            Number { idx: 0, val: 467 },
            Number { idx: 1, val: 114 },
            Number { idx: 2, val: 35 },
            Number { idx: 3, val: 633 },
            Number { idx: 4, val: 617 },
            Number { idx: 5, val: 58 },
            Number { idx: 6, val: 592 },
            Number { idx: 7, val: 755 },
            Number { idx: 8, val: 664 },
            Number { idx: 9, val: 598 },
        ];
        assert_eq!(schematic.numbers, expected_numbers);

        let expected_symbols = vec![
            Symbol {
                idx: 0,
                x: 3,
                y: 1,
                char: '*',
            },
            Symbol {
                idx: 1,
                x: 6,
                y: 3,
                char: '#',
            },
            Symbol {
                idx: 2,
                x: 3,
                y: 4,
                char: '*',
            },
            Symbol {
                idx: 3,
                x: 5,
                y: 5,
                char: '+',
            },
            Symbol {
                idx: 4,
                x: 3,
                y: 8,
                char: '$',
            },
            Symbol {
                idx: 5,
                x: 5,
                y: 8,
                char: '*',
            },
        ];
        assert_eq!(schematic.symbols, expected_symbols);
    }
}
