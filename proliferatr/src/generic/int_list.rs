use std::{convert::Infallible, ops::Range};

use derive_builder::Builder;
use rand::{distributions::Uniform, prelude::Distribution};
use thiserror::Error;

use crate::InputGenerator;

#[derive(Debug, Clone, Error)]
pub enum IntListError {
    #[error("Invalid min..max range: {min}..{max}")]
    InvalidValueRange { min: i64, max: i64 },

    #[error("Invalid min_length..max_length range: {min_length}..{max_length}")]
    InvalidLengthRange {
        min_length: usize,
        max_length: usize,
    },
}

/// A type that can generate inputs comprised of a list of random integers.
///
/// # Examples
/// ```
/// use proliferatr::generic::IntList;
///
/// // Ranges are i64 and exclusive of the max value,
/// // so maximum possible value in this example is 99,999
/// let generator = IntList::builder()
///     .value_range(1000..100_000)
///     .num_ints(1000..1201)
///     .build()
///     .expect("failed to build generator");
///
/// // the above configuration happens to be the default
/// assert_eq!(generator, IntList::default());
/// ```
#[derive(Debug, Clone, Builder, PartialEq, Eq, Hash)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct IntList {
    value_range: Range<i64>,
    num_ints: Range<usize>,
}

impl Default for IntList {
    fn default() -> Self {
        Self {
            value_range: 1000..100_000,
            num_ints: 1000..1201,
        }
    }
}

impl IntListBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref value_range) = self.value_range {
            if value_range.start >= value_range.end {
                return Err(IntListError::InvalidValueRange {
                    min: value_range.start,
                    max: value_range.end,
                }
                .to_string());
            }
        }

        if let Some(ref num_ints) = self.num_ints {
            if num_ints.start >= num_ints.end {
                return Err(IntListError::InvalidLengthRange {
                    min_length: num_ints.start,
                    max_length: num_ints.end,
                }
                .to_string());
            }
        }

        Ok(())
    }
}

impl IntList {
    pub fn builder() -> IntListBuilder {
        IntListBuilder::default()
    }

    /// Generate a random vec of ints using the configuration of this `IntList`.
    ///
    /// The reason this method exists is to allow the underlying `IntList`
    /// functionality to be used by another input generator.
    ///
    /// This allocates, but means we don't have to deal with the lifetime of the
    /// RNG. This _should_ be an okay-enough tradeoff.
    pub fn gen_ints<R: rand::Rng + Clone + ?Sized>(&self, rng: &mut R) -> Vec<i64> {
        let num_ints = rng.gen_range(self.num_ints.clone());
        let uniform = Uniform::from(self.value_range.clone());
        uniform.sample_iter(rng).take(num_ints).collect()
    }
}

impl InputGenerator for IntList {
    type GeneratorError = Infallible;
    type Output = Vec<i64>;

    fn gen_input<R: rand::Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        Ok(self.gen_ints(rng))
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn int_generator() {
        let mut rng = thread_rng();
        let g = IntList::builder()
            .value_range(1000..10_000)
            .num_ints(100..151)
            .build()
            .unwrap();

        let r = g.gen_input(&mut rng).unwrap();
        let num_ints = r.len();
        assert!(num_ints < 151 && num_ints >= 100);
    }

    #[test]
    fn range_validation() {
        let g = IntList::builder()
            .value_range(1000..100)
            .num_ints(100..151)
            .build();

        assert!(g.is_err());

        let g = IntList::builder()
            .value_range(1000..10000)
            .num_ints(100..20)
            .build();

        assert!(g.is_err());
    }
}
