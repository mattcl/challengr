use std::{convert::Infallible, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::Day;

const DIMENSION: usize = 140;
const NUM_GALAXIES: Range<usize> = 400..451;

/// Strategy is just to generate N points in a 140x140 grid where no 2 points
/// are within 2 units of each other
#[derive(Debug, Default, Clone, Copy)]
pub struct Day11;

impl Day for Day11 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Self
            .gen_input(rng)?
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n"))
    }
}

impl InputGenerator for Day11 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<char>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut out = vec![vec!['.'; DIMENSION]; DIMENSION];

        let r_dist = Uniform::from(0..DIMENSION);
        let c_dist = Uniform::from(0..DIMENSION);

        for _ in 0..rng.gen_range(NUM_GALAXIES) {
            loop {
                let r = r_dist.sample(rng);
                let c = c_dist.sample(rng);

                if any_around(r, c, &out) {
                    continue;
                }

                out[r][c] = '#';
                break;
            }
        }

        Ok(out)
    }
}

fn any_around(row: usize, col: usize, grid: &[Vec<char>]) -> bool {
    for dr in -1..=1 {
        let r = row as i32 + dr;
        if r < 0 || r >= DIMENSION as i32 {
            continue;
        }

        for dc in -1..=1 {
            let c = col as i32 + dc;

            if c < 0 || c >= DIMENSION as i32 {
                continue;
            }

            if grid[r as usize][c as usize] == '#' {
                return true;
            }
        }
    }

    false
}
