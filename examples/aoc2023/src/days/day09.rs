use std::{convert::Infallible, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::Rng;

use super::Day;

const NUM_ROWS: Range<usize> = 200..211;
const NUM_VALUES: usize = 21;
const STARTING_DEPTH: Range<usize> = 1..(NUM_VALUES - 1);
const STARTING_VALUE: Range<i64> = -5..15;

// We could do a pascal's triangle related math trick, probably, but growing the
// list from a starting depth is simple enough. We just need to pick the
// starting value for the above layer randomly.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day09;

impl Day for Day09 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Self
            .gen_input(rng)?
            .iter()
            .map(|r| r.iter().join(" "))
            .join("\n"))
    }
}

impl InputGenerator for Day09 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<i64>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let num_rows = rng.gen_range(NUM_ROWS);
        let mut out = Vec::with_capacity(num_rows);
        let mut next = Vec::with_capacity(NUM_VALUES);

        for _ in 0..num_rows {
            next.clear();
            let mut row = Vec::with_capacity(NUM_VALUES);
            let starting_depth = rng.gen_range(STARTING_DEPTH);
            // allow because want the alloc to be the full width
            #[allow(clippy::same_item_push)]
            for _ in 0..starting_depth {
                row.push(0);
            }

            while row.len() < 21 {
                let prev = rng.gen_range(STARTING_VALUE);
                next.push(prev);

                for (idx, v) in row.drain(..).enumerate() {
                    next.push(next[idx] + v);
                }

                std::mem::swap(&mut row, &mut next);
            }

            out.push(row.clone());
        }

        Ok(out)
    }
}
