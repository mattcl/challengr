use std::ops::Range;

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const INCR_RANGE: Range<i8> = 1..4;
const VALUE_RANGE: Range<i8> = 1..100;
const NUM_VALUES: Range<usize> = 5..9;
const MIN_NUM_VALID: Range<usize> = 300..750;
const NUM_REPORTS: usize = 1000;
const OFF_BY_ONE_PROB: f64 = 0.23;

/// Strategy will be to generate up to NUM_VALUES values in ascending/descending
/// order with a probability of invalidating the sequence.
///
/// we need to ensure we have at least one isntance of the edge-case where the
/// value you need to skip is the _first_ value in the report.
///
/// we also want a minimum number of valid reports
#[derive(Debug, Default, Clone, Copy)]
pub struct Day02;

impl Day for Day02 {
    fn generate<R: Rng + Clone>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day02 {}.gen_input(rng)?.join("\n"))
    }
}

impl InputGenerator for Day02 {
    type GeneratorError = anyhow::Error;
    type Output = Vec<String>;

    fn gen_input<R: Rng + Clone>(&self, rng: &mut R) -> Result<Self::Output, Self::GeneratorError> {
        let mut out = Vec::with_capacity(NUM_REPORTS);

        let valid = Uniform::from(MIN_NUM_VALID).sample(rng);

        let remaining = NUM_REPORTS - valid;

        for _ in 0..valid {
            let mut report = make_valid_report(rng);

            if rng.gen_bool(OFF_BY_ONE_PROB) {
                // pick an index to mutate
                let alteration_idx = rng.gen_range(0..report.len());

                // insert a value
                let value = if rng.gen_bool(0.5) {
                    rng.gen_range(VALUE_RANGE)
                } else {
                    report[alteration_idx]
                };
                report.insert(alteration_idx, value);
            }

            out.push(report.iter().join(" "));
        }

        for _ in 0..remaining {
            out.push(make_maybe_invalid(rng).iter().join(" "));
        }

        out.shuffle(rng);

        Ok(out)
    }
}

fn make_valid_report<R: Rng + Clone>(rng: &mut R) -> Vec<i8> {
    let start = rng.gen_range(VALUE_RANGE);
    let len = rng.gen_range(NUM_VALUES);

    let ascending = if start as i64 + (len as i64 * 3) > 99 {
        false
    } else if start as i64 - (len as i64 * 3) < 1 {
        true
    } else {
        rng.gen_bool(0.5)
    };

    let mut out = Vec::with_capacity(len);

    out.push(start);

    for i in 0..(len - 1) {
        if ascending {
            out.push(out[i] + rng.gen_range(INCR_RANGE));
        } else {
            out.push(out[i] - rng.gen_range(INCR_RANGE));
        }
    }

    out
}

fn make_maybe_invalid<R: Rng + Clone>(rng: &mut R) -> Vec<i8> {
    let len = rng.gen_range(NUM_VALUES);
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(rng.gen_range(VALUE_RANGE));
    }

    out
}
