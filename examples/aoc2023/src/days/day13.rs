use std::{fmt::Display, ops::Range, str::FromStr};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::Rng;

use super::Day;

const DIMENSION: Range<usize> = 10..20;
const NUM_MIRRORS: usize = 100;

// Strategy is to pick a mirror with a random dimension, select a row in which
// to start the symmetry, insert duplicate rows to make the symmetry happen,
// smudge one cell, then append or pepend a perfect symmetry pair of rows to the
// top or bottom. From there, we can randomly opt to rotate the mirror.
//
// We want 100 valid mirrors for which there are only one solution, so we will
// then attempt to generate the solution for each mirror, rejecting ones that
// are ambiguous. This is going to be slow, relatively speaking.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day13;

impl Day for Day13 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day13.gen_input(rng)?.iter().join("\n\n"))
    }
}

impl InputGenerator for Day13 {
    type GeneratorError = anyhow::Error;
    type Output = Vec<String>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut mirrors = Vec::with_capacity(NUM_MIRRORS);

        while mirrors.len() < NUM_MIRRORS {
            let mut m = Mirror::random(rng);
            if rng.gen_bool(0.5) {
                m = m.rotate();
            }

            let s = m.to_string();

            let bm = BitMirror::from_str(&s)?;

            // this is going to be very slow since we're at the mercy of RNG.
            // I suspect the real input had hand-crafted mirrors and a selection
            // was made and transformed to randomize inputs.
            if bm.unique_solution() {
                mirrors.push(s);
            }
        }

        Ok(mirrors)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mirror {
    chars: Vec<Vec<char>>,
}

impl Mirror {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let width = rng.gen_range(DIMENSION);
        let height = rng.gen_range(DIMENSION);

        let mut chars = vec![vec![]; height];

        let mirror_row = rng.gen_range(0..(height - 1));

        let mut above = mirror_row as isize;
        let mut below = mirror_row + 1;

        let add_above = above as usize > height - below;

        let mut min_region = above as usize;
        let mut max_region = below;

        while above >= 0 || below < height {
            let row = make_row(rng, width);

            if above >= 0 {
                min_region = above as usize;
                chars[above as usize] = row.clone();
                above -= 1;
            }

            if below < height {
                max_region = below;
                chars[below] = row;
                below += 1;
            }
        }

        // smudge
        let r = rng.gen_range(min_region..=max_region);
        let c = rng.gen_range(0..width);

        if chars[r][c] == '.' {
            chars[r][c] = '#';
        } else {
            chars[r][c] = '.';
        }

        // This isn't great, I guess, since it means the p1 symmetry will always
        // be at the "top" or "bottom". Many of the mirrors in the real input
        // had this "feature," which makes me think it's the "reasonable"
        // approach to doing this.
        if add_above {
            let r = make_row(rng, width);
            chars.push(r.clone());
            chars.push(r);
        } else {
            let r = make_row(rng, width);
            chars.insert(0, r.clone());
            chars.insert(0, r);
        }

        Self { chars }
    }

    pub fn rotate(&self) -> Self {
        let n = self.chars.len();
        let m = self.chars[0].len();
        let mut chars = vec![vec!['.'; n]; m];

        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..m {
                chars[j][n - i - 1] = self.chars[i][j];
            }
        }

        Self { chars }
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.chars
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n")
            .fmt(f)
    }
}

fn make_row<R: Rng + Clone + ?Sized>(rng: &mut R, width: usize) -> Vec<char> {
    (0..width)
        .map(|_| if rng.gen_bool(0.5) { '#' } else { '.' })
        .collect()
}

// for checking. This is basically my real solution modified to ensure we only
// have one off by 1/symmetry line. Converting to string then to this is pretty
// much a waste, but it should be fast enough.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitMirror {
    horizontal: Vec<u32>,
    width: usize,
    height: usize,
}

impl FromStr for BitMirror {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.len()).unwrap_or_default();

        let mut mirror = Self {
            width,
            ..Default::default()
        };

        for line in s.lines() {
            mirror
                .horizontal
                .push(line.chars().enumerate().fold(0, |acc, (col, ch)| {
                    if ch == '#' {
                        acc | (1 << col)
                    } else {
                        acc
                    }
                }));
        }

        mirror.height = mirror.horizontal.len();

        Ok(mirror)
    }
}

impl Display for BitMirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.horizontal.iter() {
            writeln!(f, "{:018b}", row)?;
        }
        Ok(())
    }
}

impl BitMirror {
    pub fn unique_solution(&self) -> bool {
        self.reflect_vertical() + self.reflect_horizontal() == 1
            && self.reflect_vertical_one_off() + self.reflect_horizontal_one_off() == 1
    }

    pub fn reflect_horizontal(&self) -> usize {
        let mut count = 0;
        'outer: for i in 1..self.width {
            let limit = self.width - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for row in self.horizontal.iter() {
                let reversed = (row >> i).reverse_bits() >> adjust;
                let masked = (row >> shift) & mask;
                if masked != reversed {
                    continue 'outer;
                }
            }

            count += 1;
        }

        count
    }

    pub fn reflect_vertical(&self) -> usize {
        let mut count = 0;
        'outer: for i in 0..(self.height - 1) {
            let limit = self.height - i - 2;
            // expand outward
            for delta in 0..=i.min(limit) {
                if self.horizontal[i - delta] != self.horizontal[i + 1 + delta] {
                    continue 'outer;
                }
            }

            count += 1;
        }

        count
    }

    pub fn reflect_horizontal_one_off(&self) -> usize {
        let mut count = 0;
        'outer: for i in 1..self.width {
            let mut one_count = 0;
            let limit = self.width - i;
            let adjust = 32 - limit.min(i);
            let mask = u32::MAX >> adjust;
            let shift = if limit < i { i - limit } else { 0 };
            for row in self.horizontal.iter() {
                let reversed = (row >> i).reverse_bits() >> adjust;
                let masked = (row >> shift) & mask;
                one_count += (masked ^ reversed).count_ones();

                if one_count > 1 {
                    continue 'outer;
                }
            }

            if one_count == 1 {
                count += 1;
            }
        }

        count
    }

    pub fn reflect_vertical_one_off(&self) -> usize {
        let mut count = 0;
        'outer: for i in 0..(self.height - 1) {
            let mut one_count = 0;
            let limit = self.height - i - 2;
            // expand outward
            for delta in 0..=i.min(limit) {
                one_count +=
                    (self.horizontal[i - delta] ^ self.horizontal[i + 1 + delta]).count_ones();

                if one_count > 1 {
                    continue 'outer;
                }
            }

            if one_count == 1 {
                count += 1;
            }
        }

        count
    }
}
