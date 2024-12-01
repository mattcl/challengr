use proliferatr::{generic::IntList, InputGenerator};
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const NUM_VALUES: usize = 1000;
const REPEAT_PROBABILITY: f64 = 0.7;

/// Strategy will be to generate two lists of random numbers, using the first
/// list as optional candidates for generating the second list.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day01;

impl Day for Day01 {
    fn generate<R: Rng + Clone>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day01 {}.gen_input(rng)?.join("\n"))
    }
}

impl InputGenerator for Day01 {
    type GeneratorError = anyhow::Error;
    type Output = Vec<String>;

    fn gen_input<R: Rng + Clone>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let left = IntList::builder()
            .value_range(10_000..100_000)
            .num_ints(NUM_VALUES..(NUM_VALUES + 1))
            .build()?
            .gen_input(rng)?;

        let mut right = Vec::with_capacity(left.len());

        let distr = Uniform::from(10_000..100_000);
        for _ in 0..left.len() {
            if rng.gen_bool(REPEAT_PROBABILITY) {
                right.push(*left.choose(rng).unwrap());
            } else {
                right.push(distr.sample(rng));
            }
        }

        let mut out = Vec::with_capacity(left.len());

        for (left, right) in left.iter().zip(right.iter()) {
            out.push(format!("{}   {}", left, right));
        }

        Ok(out)
    }
}
