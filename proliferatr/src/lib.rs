#![doc = include_str!("../README.md")]
use rand::Rng;

pub mod bound;
pub mod direction;
pub mod generic;
pub mod grid;
pub mod maze;
pub mod path;
pub mod point;

/// Indicates that the implementing type can act as an input generator.
///
/// Input generators produce inputs. The intent is to provide a way to generate
/// unique-enough inputs for answer-oriented programming challenges, though they
/// could be used in other ways. The trait simply allows a uniforn interface for
/// operating across a set of generators of different types.
pub trait InputGenerator {
    type GeneratorError;
    type Output;

    /// Attempt to generate an input, optionally using the provided RNG.
    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError>;
}

/// Indicates that the implementing type can act as an input validator
///
/// Input validators take a given input and use their `validate` method to
/// indicate of an input is valid or not. The intent is for the validator to be
/// used internally by a generator to ensure generated inputs that would be
/// returned are valid inputs. For that reason, this takes input as a `&str`.
pub trait InputValidator {
    type ValidatorError;

    fn validate(&self, input: &str) -> Result<bool, Self::ValidatorError>;
}
