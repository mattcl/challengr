use derive_builder::Builder;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom};
use thiserror::Error;

use crate::InputGenerator;

pub const LOWER_ALPHA_CHARS: &[u8] = b"bcdfghjklmnpqrstvwxz";
pub const UPPER_ALPHA_CHARS: &[u8] = b"BCDFGHJKLMNPQRSTVWXZ";

#[derive(Debug, Clone, Error)]
pub enum TokenError {
    #[error("Failed to select a character.")]
    FailedToSelectCharacter,
}

/// A type that can generate a random token string.
///
/// # Examples
/// ```
/// use proliferatr::generic::StringToken;
/// let generator = StringToken::builder()
///     .length(2..=3)
///     .charset(b"bcdfghjklmnpqrstvwxz")
///     .build()
///     .expect("failed to build generator");
///
/// // the above configuration happens to be the default. Though using the
/// // LOWER_ALPHA_CHARS constant.
/// assert_eq!(generator, StringToken::default());
/// ```
#[derive(Debug, Clone, PartialEq, Builder)]
pub struct StringToken<'a> {
    #[builder(setter(into))]
    length: Uniform<usize>,
    charset: &'a [u8],
}

impl<'a> Default for StringToken<'a> {
    fn default() -> Self {
        Self {
            length: (2..4).into(),
            charset: LOWER_ALPHA_CHARS,
        }
    }
}

impl<'a> StringToken<'a> {
    pub fn builder() -> StringTokenBuilder<'a> {
        StringTokenBuilder::default()
    }
}

impl<'a> InputGenerator for StringToken<'a> {
    type GeneratorError = TokenError;
    type Output = String;

    fn gen_input<R: rand::Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let len = self.length.sample(rng);
        (0..len)
            .map(|_| {
                self.charset
                    .choose(rng)
                    .copied()
                    .map(|v| v as char)
                    .ok_or(TokenError::FailedToSelectCharacter)
            })
            .collect::<Result<String, _>>()
    }
}
