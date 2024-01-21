use std::{collections::VecDeque, ops::Range, str::FromStr};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use rustc_hash::FxHashMap;
use thiserror::Error;

use super::Day;

// in practice, we seem to have a high probability of finding a solution in just
// one attempt, so this pretty much guarantees we find a solution.
const NUM_ATTEMPTS: usize = 5000;
const DIMENSION: usize = 100;
const NUM_SQUARE_ROCKS: Range<usize> = 1600..1701;
const NUM_ROUND_ROCKS: Range<usize> = 1900..2101;

// these are "approximate" as we are just using them to bound the number of
// iterations
const CYCLE_START_LIMIT: usize = 180;
const CYCLE_LEN_LIMIT: usize = 100;

#[derive(Debug, Clone, Copy, Error)]
pub enum Day14Error {
    #[error("Failed to produce a valid input in {0} attempts.")]
    FailedToProduceInput(usize),
}

/// So we don't know if this is _actually_ valid, but the plan is to generate
/// random configurations of rocks, then run that configuration against my real
/// solution for day 14 with a much smaller number of steps. If the cycle is
/// detected within the allowed number of steps, we'll call it a valid input.
/// If not, we'll try again, up to 5000 times.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day14;

impl Day for Day14 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Day14.gen_input(rng)
    }
}

impl InputGenerator for Day14 {
    type GeneratorError = Day14Error;
    type Output = String;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let dist = Uniform::from(0..DIMENSION);

        for _ in 0..NUM_ATTEMPTS {
            let num_square_rocks = rng.gen_range(NUM_SQUARE_ROCKS);
            let num_round_rocks = rng.gen_range(NUM_ROUND_ROCKS);

            let mut grid = vec![vec!['.'; DIMENSION]; DIMENSION];

            // place squares
            let mut count = 0;
            while count < num_square_rocks {
                let r = dist.sample(rng);
                let c = dist.sample(rng);

                if grid[r][c] != '.' {
                    continue;
                }

                grid[r][c] = '#';
                count += 1;
            }

            // place rounds
            count = 0;
            while count < num_round_rocks {
                let r = dist.sample(rng);
                let c = dist.sample(rng);

                if grid[r][c] != '.' {
                    continue;
                }

                grid[r][c] = 'O';
                count += 1;
            }

            let output = grid.iter().map(|r| r.iter().collect::<String>()).join("\n");

            let mut dish = BitDish::from_str(&output).unwrap();
            if dish
                .cycle(CYCLE_START_LIMIT + CYCLE_LEN_LIMIT * 2)
                .is_some()
            {
                return Ok(output);
            }
        }

        Err(Day14Error::FailedToProduceInput(NUM_ATTEMPTS))
    }
}

// The following is modified from my actual solution modified to be more
// explicit about a cycle starting and being detected within a fixed number of
// tilting operations.
#[derive(Debug, Default, Clone)]
pub struct BitDish {
    rounds: Vec<u128>,
    cubes: Vec<u128>,
    height: usize,
    left_border_mask: u128,
    right_border_mask: u128,
}

impl BitDish {
    fn total_load(&self) -> u32 {
        self.rounds
            .iter()
            .enumerate()
            .map(|(i, r)| (self.height - i) as u32 * r.count_ones())
            .sum()
    }

    pub fn cycle(&mut self, count: usize) -> Option<u32> {
        let mut cache: FxHashMap<(u128, u128), usize> = FxHashMap::default();
        let mut loads: Vec<u32> = Vec::with_capacity(500);
        for cycle_idx in 0..count {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            let load = self.total_load();

            loads.push(load);

            // make a key with the last 8
            if cycle_idx > 8 {
                let key_a: u128 = loads[(loads.len() - 5)..]
                    .iter()
                    .fold(0, |acc, v| (acc << 32 | *v as u128));
                let key_b: u128 = loads[(loads.len() - 9)..(loads.len() - 4)]
                    .iter()
                    .fold(0, |acc, v| (acc << 32 | *v as u128));
                let key = (key_a, key_b);

                let e = cache.entry(key).or_insert(cycle_idx);

                if *e != cycle_idx {
                    let period = cycle_idx - *e;
                    // dbg!(period, cycle_idx);

                    let rem = (count - cycle_idx) % period;
                    // we need to advance by rem in loads from the last index - 1
                    return Some(loads[*e + rem - 1]);
                }
            }
        }

        None
    }

    fn tilt_north(&mut self) {
        let mut rows = VecDeque::from_iter(1..self.height);

        while let Some(row) = rows.pop_front() {
            let target_row = row - 1;
            let moves_available =
                self.rounds[row] & !self.rounds[target_row] & !self.cubes[target_row];

            if moves_available != 0 {
                self.rounds[row] &= !moves_available;
                self.rounds[target_row] |= moves_available;

                if target_row > 0 {
                    rows.push_front(target_row);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut rows = Vec::from_iter(0..(self.height - 1));
        while let Some(row) = rows.pop() {
            let target_row = row + 1;
            let moves_available =
                self.rounds[row] & !self.rounds[target_row] & !self.cubes[target_row];

            if moves_available != 0 {
                self.rounds[row] &= !moves_available;
                self.rounds[target_row] |= moves_available;

                if target_row < self.height - 1 {
                    rows.push(target_row);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut rows = Vec::from_iter(0..self.height);
        while let Some(row) = rows.pop() {
            let cubes = self.cubes[row];
            let rounds = self.rounds[row];
            let moves_available = rounds & !((rounds | cubes) >> 1) & self.left_border_mask;
            if moves_available != 0 {
                self.rounds[row] = rounds & !moves_available | moves_available << 1;
                rows.push(row);
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut rows = Vec::from_iter(0..self.height);
        while let Some(row) = rows.pop() {
            let cubes = self.cubes[row];
            let rounds = self.rounds[row];
            let moves_available = rounds & !((rounds | cubes) << 1) & self.right_border_mask;
            if moves_available != 0 {
                self.rounds[row] = rounds & !moves_available | moves_available >> 1;
                rows.push(row);
            }
        }
    }
}

impl FromStr for BitDish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let height = lines.len();
        let width = lines[0].len();

        let mut rounds = vec![0; height];
        let mut cubes = vec![0; height];

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        cubes[row] |= 1_u128 << (width - col - 1);
                    }
                    'O' => rounds[row] |= 1_u128 << (width - col - 1),
                    _ => {}
                }
            }
        }

        let left_border_mask = !(1_u128 << (width - 1));
        let right_border_mask = !1;

        Ok(Self {
            rounds,
            cubes,
            height,
            left_border_mask,
            right_border_mask,
        })
    }
}
