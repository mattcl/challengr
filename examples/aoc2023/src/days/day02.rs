use std::{convert::Infallible, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const NUM_GAMES: Range<usize> = 100..111;
const NUM_DRAWS: Range<usize> = 1..6;
const COLORS: &[&str] = &["red", "green", "blue"];

/// The strategy here will be to generate a random number of draws in
/// random draws in random orders a random number of times between 100 and 110.
///
/// The "real" inputs seem to only have 100 games, but there's no reason we
/// can't have more.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day02;

impl Day for Day02 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Day02 {}.gen_input(rng)?.to_string())
    }
}

impl InputGenerator for Day02 {
    type GeneratorError = Infallible;
    type Output = Games;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let num_games = rng.gen_range(NUM_GAMES);
        Ok(Games::random(rng, num_games))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Games {
    games: Vec<Game>,
}

impl Games {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R, num: usize) -> Self {
        Self {
            games: (0..num).map(|_| Game::random(rng)).collect(),
        }
    }
}

impl Display for Games {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.games
            .iter()
            .enumerate()
            .map(|(idx, g)| format!("Game {}: {}", idx + 1, g))
            .join("\n")
            .fmt(f)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Game {
    draws: Vec<Draw>,
}

impl Game {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let num_draws = rng.gen_range(NUM_DRAWS);

        let mut draws = Vec::with_capacity(num_draws);

        for _ in 0..num_draws {
            draws.push(Draw::random(rng));
        }

        Self { draws }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.draws.iter().map(|d| d.to_string()).join("; ").fmt(f)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Draw {
    values: [u8; 3],
    order: [usize; 3],
}

impl Draw {
    /// Make a random draw, ensuring at least one value
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let v_dist = Uniform::from(0_u8..21);
        let mut values = [0_u8; 3];

        let mut sum = 0;
        while sum == 0 {
            #[allow(clippy::needless_range_loop)]
            for i in 0..3 {
                values[i] = v_dist.sample(rng);
                sum += values[i];
            }
        }

        let mut order = [0, 1, 2];
        order.shuffle(rng);

        Self { values, order }
    }
}

impl Display for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.order
            .into_iter()
            .filter(|idx| self.values[*idx] > 0)
            .map(|idx| format!("{} {}", self.values[idx], COLORS[idx]))
            .join(", ")
            .fmt(f)
    }
}
