use std::convert::Infallible;

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::Day;

const DIMENSION: usize = 141;
const CENTER: usize = DIMENSION / 2;
const OUTER_DIST: usize = CENTER - 6;

/// It appears like the center of the real inputs have much higher numbers than
/// the edges
#[derive(Debug, Default, Clone, Copy)]
pub struct Day17;

impl Day for Day17 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Self
            .gen_input(rng)?
            .iter()
            .map(|r| {
                r.iter()
                    .map(|c| char::from_digit(*c as u32, 10).unwrap())
                    .collect::<String>()
            })
            .join("\n"))
    }
}

impl InputGenerator for Day17 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<u8>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut out = vec![vec![0; DIMENSION]; DIMENSION];
        let center = Location {
            row: CENTER,
            col: CENTER,
        };

        let inner = Uniform::from(4..10);
        let outer = Uniform::from(1..7);

        #[allow(clippy::needless_range_loop)]
        for row in 0..DIMENSION {
            for col in 0..DIMENSION {
                let loc = Location { row, col };
                if loc.manhattan_dist(&center) < OUTER_DIST {
                    out[row][col] = inner.sample(rng);
                } else {
                    out[row][col] = outer.sample(rng);
                }
            }
        }

        Ok(out)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    pub fn manhattan_dist(&self, other: &Self) -> usize {
        self.row.max(other.row) - self.row.min(other.row) + self.col.max(other.col)
            - self.col.min(other.col)
    }
}
