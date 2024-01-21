use std::{convert::Infallible, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const DIMENSION: usize = 110;
const SYMBOLS: &[u8] = b"\\/|-";
const NUM_SYMBOLS: Range<usize> = 1100..1300;

// We're just going to completely random this, with maybe an intentional loop
// included. The only thing we really have to to is to make sure that the upper
// left corner is a backslash.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day16;

impl Day for Day16 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day16
            .gen_input(rng)?
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n"))
    }
}

impl InputGenerator for Day16 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<char>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut grid = vec![vec!['.'; DIMENSION]; DIMENSION];
        let mut count = 1;
        grid[0][0] = '\\';

        let desired = rng.gen_range(NUM_SYMBOLS);
        let dist = Uniform::from(0..DIMENSION);

        while count < desired {
            let r = dist.sample(rng);
            let c = dist.sample(rng);

            if grid[r][c] != '.' {
                continue;
            }

            let s = *SYMBOLS.choose(rng).unwrap() as char;
            grid[r][c] = s;

            count += 1;
        }

        Ok(grid)
    }
}
