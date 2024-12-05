use std::{fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::seq::SliceRandom;
use rustc_hash::FxHashSet;

use super::Day;

const SIZE: usize = 140;
const XMAS_CHARS: &[u8] = b"XMAS";
const NUM_PART1: Range<usize> = 3500..4500;
const NUM_PART2: Range<usize> = 2500..3000;

const NORTH: &[(i64, i64, char)] = &[(0, 0, 'X'), (-1, 0, 'M'), (-2, 0, 'A'), (-3, 0, 'S')];
const SOUTH: &[(i64, i64, char)] = &[(0, 0, 'X'), (1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S')];
const EAST: &[(i64, i64, char)] = &[(0, 0, 'X'), (0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S')];
const WEST: &[(i64, i64, char)] = &[(0, 0, 'X'), (0, -1, 'M'), (0, -2, 'A'), (0, -3, 'S')];
const NORTH_EAST: &[(i64, i64, char)] = &[(0, 0, 'X'), (-1, 1, 'M'), (-2, 2, 'A'), (-3, 3, 'S')];
const SOUTH_EAST: &[(i64, i64, char)] = &[(0, 0, 'X'), (1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S')];
const NORTH_WEST: &[(i64, i64, char)] = &[(0, 0, 'X'), (-1, -1, 'M'), (-2, -2, 'A'), (-3, -3, 'S')];
const SOUTH_WEST: &[(i64, i64, char)] = &[(0, 0, 'X'), (1, -1, 'M'), (2, -2, 'A'), (3, -3, 'S')];

const DIRS: &[&[(i64, i64, char)]] = &[
    NORTH, SOUTH, EAST, WEST, NORTH_EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST,
];

const DIAG_UP: &[(i64, i64, char, i64, i64, char)] =
    &[(1, -1, 'M', -1, 1, 'S'), (1, -1, 'S', -1, 1, 'M')];

const DIAG_DN: &[(i64, i64, char, i64, i64, char)] =
    &[(-1, -1, 'M', 1, 1, 'S'), (-1, -1, 'S', 1, 1, 'M')];

#[derive(Debug, Default, Clone, Copy)]
pub struct Day04;

impl Day for Day04 {
    fn generate<R: rand::Rng + Clone>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day04 {}.gen_input(rng)?.to_string())
    }
}

impl InputGenerator for Day04 {
    type GeneratorError = anyhow::Error;
    type Output = Grid;

    fn gen_input<R: rand::Rng + Clone>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut grid = Grid::new(rng);

        let num_part1 = rng.gen_range(NUM_PART1);
        let num_part2 = rng.gen_range(NUM_PART2);

        for _ in 0..num_part1 {
            grid.insert(rng, Token::Xmas);
        }

        for _ in 0..num_part2 {
            grid.insert(rng, Token::Cross);
        }

        Ok(grid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    Xmas,
    Cross,
}

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<char>>,
    seen: FxHashSet<(usize, usize)>,
}

impl Grid {
    pub fn new<R: rand::Rng + Clone>(rng: &mut R) -> Self {
        let mut cells = vec![vec!['.'; SIZE]; SIZE];

        #[allow(clippy::needless_range_loop)]
        for r in 0..SIZE {
            for c in 0..SIZE {
                cells[r][c] = XMAS_CHARS.choose(rng).copied().unwrap().into();
            }
        }

        Self {
            cells,
            seen: FxHashSet::default(),
        }
    }

    fn insert<R: rand::Rng + Clone>(&mut self, rng: &mut R, token: Token) {
        let (row, col) = loop {
            let row = rng.gen_range(1..SIZE - 1);
            let col = rng.gen_range(1..SIZE - 1);

            if self.seen.contains(&(row, col)) {
                continue;
            }

            self.seen.insert((row, col));

            break (row, col);
        };

        let dir_offsets = *DIRS.choose(rng).unwrap();

        match token {
            Token::Xmas => {
                for (dr, dc, ch) in dir_offsets {
                    let cur_row = row as i64 + dr;
                    let cur_col = col as i64 + dc;

                    if cur_row < 0
                        || cur_row >= SIZE as i64
                        || cur_col < 0
                        || cur_col >= SIZE as i64
                    {
                        break;
                    }

                    self.cells[cur_row as usize][cur_col as usize] = *ch;
                }
            }
            Token::Cross => {
                // up diagonal
                let (dr1, dc1, ch1, dr2, dc2, ch2) = *DIAG_UP.choose(rng).unwrap();
                let cur_row = row as i64 + dr1;
                let cur_col = col as i64 + dc1;

                if cur_row < 0 || cur_row >= SIZE as i64 || cur_col < 0 || cur_col >= SIZE as i64 {
                    return;
                }

                self.cells[cur_row as usize][cur_col as usize] = ch1;

                let cur_row = row as i64 + dr2;
                let cur_col = col as i64 + dc2;

                if cur_row < 0 || cur_row >= SIZE as i64 || cur_col < 0 || cur_col >= SIZE as i64 {
                    return;
                }

                self.cells[cur_row as usize][cur_col as usize] = ch2;

                // down diagonal
                let (dr1, dc1, ch1, dr2, dc2, ch2) = *DIAG_DN.choose(rng).unwrap();
                let cur_row = row as i64 + dr1;
                let cur_col = col as i64 + dc1;

                if cur_row < 0 || cur_row >= SIZE as i64 || cur_col < 0 || cur_col >= SIZE as i64 {
                    return;
                }

                self.cells[cur_row as usize][cur_col as usize] = ch1;

                let cur_row = row as i64 + dr2;
                let cur_col = col as i64 + dc2;

                if cur_row < 0 || cur_row >= SIZE as i64 || cur_col < 0 || cur_col >= SIZE as i64 {
                    return;
                }

                self.cells[cur_row as usize][cur_col as usize] = ch2;

                self.cells[row][col] = 'A';
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.cells.iter().map(|row| row.iter().join("")).join("\n");

        std::fmt::Display::fmt(&out, f)
    }
}
