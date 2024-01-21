use std::{convert::Infallible, fmt::Display};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const DIMENSION: usize = 140;
const SYMBOLS: [char; 10] = ['@', '#', '$', '%', '&', '*', '+', '=', '/', '.'];
const FAILURES_PER_ROW_SLICE: u32 = 20;
const MAX_PER_ROW_SLICE: u32 = 20;
const VALUE_SELECTION_PROBABILITY: f64 = 0.6;

/// The strategy here is to randomly select coordinates, assign that coordinate
/// a symbol, variant, and optional values, then attempt to insert those chars
/// into the grid, marking the "visited" locations and their neighbors. This is
/// because there never seems to be the situation where a gear has 3 numbers,
/// and it appears that no one number is shared by two symbols in the inputs.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day03;

impl Day for Day03 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Day03 {}.gen_input(rng)?.to_string())
    }
}

impl InputGenerator for Day03 {
    type GeneratorError = Infallible;
    type Output = Grid;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut grid = Grid::default();
        grid.populate(rng);
        Ok(grid)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    chars: Vec<Vec<char>>,
    excluded: Vec<Vec<bool>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            chars: vec![vec!['.'; DIMENSION]; DIMENSION],
            excluded: vec![vec![false; DIMENSION]; DIMENSION],
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.chars
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n")
            .fmt(f)
    }
}

impl Grid {
    pub fn populate<R: Rng + Clone + ?Sized>(&mut self, rng: &mut R) {
        let offset_dist = Uniform::from(-1..=1);
        let v_dist = Uniform::from(1..1000);
        let c_dist = Uniform::from(1..(DIMENSION - 1));
        for r in 2..(DIMENSION - 1) {
            let r_dist = Uniform::from((r - 1)..(r + 1));

            let mut num_failures = 0;
            let mut num_successes = 0;
            while num_failures < FAILURES_PER_ROW_SLICE && num_successes < MAX_PER_ROW_SLICE {
                let row = r_dist.sample(rng);
                let col = c_dist.sample(rng);
                let loc = Location::new(row, col);
                let gear = Gear::random(rng, &v_dist, &offset_dist);

                if self.attempt_insert(loc, gear) {
                    num_successes += 1;
                } else {
                    num_failures += 1;
                }
            }
        }
    }

    fn attempt_insert(&mut self, location: Location, gear: Gear) -> bool {
        if !self.excluded[location.row][location.col] {
            let mut desired_insertions: Vec<(Location, char)> = vec![(location, gear.ch)];

            // compute each desired location and insertion char
            match gear.variant {
                Variant::NS => {
                    if let Some((mut offset, value)) = gear.left {
                        // this should always be safe because of the row range
                        // we sample from
                        let row = location.row - 1;
                        let mut working = value;
                        while working > 0 {
                            let digit = working % 10;

                            let col = location.col as i32 + offset;
                            if col < 0
                                || col as usize >= self.chars[0].len()
                                || self.excluded[row][col as usize]
                            {
                                return false;
                            }

                            desired_insertions.push((
                                Location::new(row, col as usize),
                                char::from_digit(digit, 10).unwrap(),
                            ));

                            working /= 10;
                            offset -= 1;
                        }
                    }

                    if let Some((mut offset, value)) = gear.right {
                        // this should always be safe because of the row range
                        // we sample from
                        let row = location.row + 1;
                        let mut working = value;
                        while working > 0 {
                            let digit = working % 10;

                            let col = location.col as i32 + offset;
                            if col < 0
                                || col as usize >= self.chars[0].len()
                                || self.excluded[row][col as usize]
                            {
                                return false;
                            }

                            desired_insertions.push((
                                Location::new(row, col as usize),
                                char::from_digit(digit, 10).unwrap(),
                            ));

                            working /= 10;
                            offset -= 1;
                        }
                    }
                }
                Variant::WE => {
                    if let Some((offset, value)) = gear.left {
                        // this will always be safe because of the row range
                        let row = (location.row as i32 + offset) as usize;

                        let mut col_offset = -1;
                        let mut working = value;
                        while working > 0 {
                            let digit = working % 10;

                            let col = location.col as i32 + col_offset;
                            if col < 0
                                || col as usize >= self.chars[0].len()
                                || self.excluded[row][col as usize]
                            {
                                return false;
                            }

                            desired_insertions.push((
                                Location::new(row, col as usize),
                                char::from_digit(digit, 10).unwrap(),
                            ));

                            working /= 10;
                            col_offset -= 1;
                        }
                    }
                    // the right side is harder in this case becase we need to
                    // know the first digit
                    if let Some((offset, value)) = gear.right {
                        let mut col = if value > 99 {
                            location.col + 3
                        } else if value > 9 {
                            location.col + 2
                        } else {
                            location.col + 1
                        };

                        // this will always be safe because of the row range
                        let row = (location.row as i32 + offset) as usize;

                        if col >= self.chars[0].len() || self.excluded[row][col] {
                            return false;
                        }

                        let mut working = value;
                        while working > 0 {
                            let digit = working % 10;
                            if self.excluded[row][col] {
                                return false;
                            }

                            desired_insertions.push((
                                Location::new(row, col),
                                char::from_digit(digit, 10).unwrap(),
                            ));

                            working /= 10;
                            col -= 1;
                        }
                    }
                }
            }

            // now that we're here, we can insert all of the locations and add
            // those locations AND their neighbors to the excluded map
            for (loc, ch) in desired_insertions {
                self.chars[loc.row][loc.col] = ch;
                self.exclude_loc_and_all_neighbors(&loc);
            }
        }
        false
    }

    fn exclude_loc_and_all_neighbors(&mut self, loc: &Location) {
        for dr in -1..=1 {
            let row = loc.row as i32 + dr;
            if row < 0 || row >= self.chars.len() as i32 {
                continue;
            }
            for dc in -1..=1 {
                let col = loc.col as i32 + dc;
                if col < 0 || col >= self.chars[0].len() as i32 {
                    continue;
                }

                self.excluded[row as usize][col as usize] = true;
            }
        }
    }
}

// It looks like there are only a few kinds of gear candidates
// (mirrors and rotations not called out):
//
// NGN
//
// N N
//  G
//
// N
//  G
//   N
//
//  N
//  G
// N
//
//  N
//  G
//   N
//
// NG
//   N
// this is dumb. I'll think of a better way later
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Variant {
    NS,
    WE,
}

const VARIANTS: [Variant; 2] = [Variant::NS, Variant::WE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gear {
    ch: char,
    variant: Variant,
    left: Option<(i32, u32)>,
    right: Option<(i32, u32)>,
}

impl Gear {
    pub fn random<R: Rng + Clone + ?Sized>(
        rng: &mut R,
        dist: &Uniform<u32>,
        offset: &Uniform<i32>,
    ) -> Self {
        let ch = SYMBOLS.choose(rng).copied().unwrap();
        let variant = VARIANTS.choose(rng).copied().unwrap();

        let left = rng
            .gen_bool(VALUE_SELECTION_PROBABILITY)
            .then(|| (offset.sample(rng), dist.sample(rng)));
        let right = rng
            .gen_bool(VALUE_SELECTION_PROBABILITY)
            .then(|| (offset.sample(rng), dist.sample(rng)));

        Self {
            ch,
            variant,
            left,
            right,
        }
    }
}
