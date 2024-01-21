use std::{convert::Infallible, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::Day;

const DIMENSION: usize = 131;
const CENTER: usize = 65;
const NUM_POINTS: Range<usize> = 1800..2401;

/// Strategy is going to be to generate a cluster of random points in the grid
/// staying away from the border and center row/colum.
///
/// The real inputs have a diamond shape in them, so we're going to replicate
/// that here, even though it shouldn't be necessary to solve the problem.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day21;

impl Day for Day21 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day21
            .gen_input(rng)?
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n"))
    }
}

impl InputGenerator for Day21 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<char>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut out = vec![vec!['.'; DIMENSION]; DIMENSION];
        // starting location
        out[CENTER][CENTER] = 'S';

        let dist = Uniform::from(1..(DIMENSION - 1));

        let num_points = rng.gen_range(NUM_POINTS);

        let mut count = 0;
        while count < num_points {
            let r = dist.sample(rng);
            if r == 0 || r == DIMENSION - 1 || r == CENTER {
                continue;
            }

            let c = dist.sample(rng);
            if c == 0 || c == DIMENSION - 1 || c == CENTER {
                continue;
            }

            if out[r][c] != '.' {
                continue;
            }

            let m_dist = r.max(CENTER) - r.min(CENTER) + c.max(CENTER) - c.min(CENTER);
            if (m_dist as i32 - CENTER as i32).abs() < 4 {
                continue;
            }

            out[r][c] = '#';

            count += 1;
        }

        Ok(out)
    }
}
