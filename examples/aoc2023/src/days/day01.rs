use std::{collections::HashSet, convert::Infallible, ops::Range};

use proliferatr::InputGenerator;
use rand::{
    distributions::{Alphanumeric, Uniform},
    prelude::Distribution,
    seq::SliceRandom,
    Rng,
};

use super::Day;

const NUM_LINES: Range<usize> = 1000..1100;
const BASE_LINE_LENGTH: Range<usize> = 1..65;
const KEYWORDS: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "oneight",
    "twone",
    "threeight",
    "fiveight",
    "sevenine",
    "eightwo",
    "eighthree",
];
const KEYWORD_PROBABILITY: f64 = 0.7;

/// The strategy will be to generate random alphanumeric strings, injecting the
/// special keywords at a random number of random locations. The real inputs
/// have 1000 lines, we're going to gen at least that many.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day01;

impl Day for Day01 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Day01 {}.gen_input(rng)?.join("\n"))
    }
}

impl InputGenerator for Day01 {
    type GeneratorError = Infallible;
    type Output = Vec<String>;

    /// Assumptions:
    ///     1. Every line MUST have at least one digit
    ///     2. We don't want 0's
    fn gen_input<R: Rng + ?Sized + Clone>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let num_lines = rng.gen_range(NUM_LINES);
        let len_dist = Uniform::from(BASE_LINE_LENGTH);

        let mut out = Vec::with_capacity(num_lines);

        for _ in 0..num_lines {
            let len = len_dist.sample(rng);

            // decide if we want to insert any keywords
            let keyword_pos: HashSet<usize> = if rng.gen_bool(KEYWORD_PROBABILITY) {
                // decide how many keywords
                let num_keywords = rng.gen_range(1..6);
                // decide where to inject those
                let pos_dist = Uniform::from(0..len);
                pos_dist
                    .sample_iter(rng.clone())
                    .take(num_keywords)
                    .collect()
            } else {
                HashSet::default()
            };

            let mut seen_digit = false;
            // this is approximate
            let mut s = String::with_capacity(len + 100);
            for (idx, ch) in Alphanumeric.sample_iter(rng.clone()).take(len).enumerate() {
                let mut ch = ch as char;
                if ch.is_ascii_digit() {
                    seen_digit = true;
                    // this isn't great, but let's just avoid the 0 stupidly
                    if ch == '0' {
                        ch = '1';
                    }
                }
                s.push(ch);
                if keyword_pos.contains(&idx) {
                    // get a keyword
                    let kw = KEYWORDS.choose(rng).unwrap();
                    s.push_str(kw);
                }
            }

            // guarantee we have a digit in the dumbest way
            if !seen_digit {
                s.push(char::from_digit(rng.gen_range(1..10), 10).unwrap_or('8'));
            }

            out.push(s.to_lowercase());
        }

        Ok(out)
    }
}
