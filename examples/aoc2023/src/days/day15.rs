use std::{collections::HashSet, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::{
    generic::{token::LOWER_ALPHA_CHARS, StringToken},
    InputGenerator,
};
use rand::{seq::SliceRandom, Rng};

use super::Day;

const LENS_RANGE: Range<u8> = 1..10;
const NUM_UNIQUE_KEYS: Range<usize> = 500..601;
const NUM_OPERATIONS: Range<usize> = 4000..5000;
const KEY_LEN: Range<usize> = 2..7;

/// Strategy is going to be to generate a fixed number of keys, then perform
/// operations using all of those keys.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day15;

impl Day for Day15 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Day15.gen_input(rng)
    }
}

impl InputGenerator for Day15 {
    type GeneratorError = anyhow::Error;
    type Output = String;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let key_gen = StringToken::builder()
            .length(KEY_LEN)
            .charset(LOWER_ALPHA_CHARS)
            .build()
            .unwrap();

        let num_keys = rng.gen_range(NUM_UNIQUE_KEYS);
        let mut keys = HashSet::with_capacity(num_keys);

        while keys.len() < num_keys {
            let key = key_gen.gen_input(rng)?;
            if keys.contains(&key) {
                continue;
            }

            keys.insert(key);
        }

        let key_refs = keys.iter().collect::<Vec<_>>();

        let num_instructions = rng.gen_range(NUM_OPERATIONS);
        let instructions = (0..num_instructions)
            .map(|_| {
                let key = key_refs.choose(rng).unwrap();
                if rng.gen_bool(0.5) {
                    Instruction {
                        key: key.as_str(),
                        operation: Operation::Remove,
                    }
                } else {
                    Instruction {
                        key: key.as_str(),
                        operation: Operation::Add(rng.gen_range(LENS_RANGE)),
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(instructions.iter().join(","))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction<'a> {
    key: &'a str,
    operation: Operation,
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.key, self.operation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operation {
    Add(u8),
    Remove,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(v) => write!(f, "={}", v),
            Operation::Remove => write!(f, "-"),
        }
    }
}
