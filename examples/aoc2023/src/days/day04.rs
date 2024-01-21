use std::{convert::Infallible, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

const NUM_CARDS: Range<usize> = 190..211;
const RUN_LENGTH: Range<usize> = 15..30;
const NUM_LEFT: usize = 10;
const NUM_RIGHT: usize = 25;

/// The main concern is making sure the part 2 sum will not be too large. It
/// appears that the real inputs constrain this value to fit within a u32, so
/// we're going to attempt to do that by splitting the input into "runs" of
/// length MIN..MAX, where a "run" starts with a card with 10 winning numbers,
/// and ends with a card with 0 winning numbers. "Runs" are self-contained in
/// that they cannot "spill" beyond the end of the "run," which would have the
/// result of compounding the next "run," etc. In doing so, we should be able to
/// limit the maximum number of duplicates for any one card.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day04;

impl Day for Day04 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Day04 {}
            .gen_input(rng)?
            .iter()
            .enumerate()
            .map(|(i, c)| format!("Card {: >3}: {}", i + 1, c))
            .join("\n"))
    }
}

impl InputGenerator for Day04 {
    type GeneratorError = Infallible;
    type Output = Vec<Card>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let num_cards = rng.gen_range(NUM_CARDS);
        let pool = (1..100).collect::<Vec<_>>();
        let mut out = Vec::with_capacity(num_cards);

        while out.len() < num_cards {
            let mut remaining = rng.gen_range(RUN_LENGTH);
            // insert a 10
            out.push(Card::random(rng, &pool, NUM_LEFT));
            while remaining > 0 {
                // using the remaining as a guide, pick the winning count such
                // that the card propagation will not continue beyond the zero
                // at the end of the run
                let num_winning = rng.gen_range(0..remaining).min(NUM_LEFT);
                out.push(Card::random(rng, &pool, num_winning));
                remaining -= 1;
            }
            // insert a 0
            out.push(Card::random(rng, &pool, 0));
        }

        Ok(out)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Card {
    left: Vec<u8>,
    right: Vec<u8>,
}

impl Card {
    /// Make a random card with the desired number of winning numbers.
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R, pool: &[u8], num_winning: usize) -> Self {
        match num_winning {
            0 => {
                let mut left = pool
                    .choose_multiple(rng, NUM_LEFT + NUM_RIGHT)
                    .copied()
                    .collect::<Vec<_>>();
                let right = left.split_off(NUM_LEFT);
                Self { left, right }
            }
            10 => {
                let right: Vec<_> = pool.choose_multiple(rng, NUM_RIGHT).copied().collect();
                let left = right.choose_multiple(rng, NUM_LEFT).copied().collect();
                Self { left, right }
            }
            x => {
                // these are disjoint
                let mut left = pool
                    .choose_multiple(rng, NUM_LEFT + NUM_RIGHT)
                    .copied()
                    .collect::<Vec<_>>();
                let right = left.split_off(NUM_LEFT);

                // select x from the right side
                for (idx, v) in right.choose_multiple(rng, x).enumerate() {
                    left[idx] = *v;
                }

                // re-shuffle the left side, since we replaced in order
                left.shuffle(rng);

                Self { left, right }
            }
        }
    }

    pub fn num_dupes(&self) -> usize {
        let mut count = 0;
        for v in self.right.iter() {
            if self.left.contains(v) {
                count += 1;
            }
        }
        count
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = self.left.iter().map(|v| format!("{: >2}", v)).join(" ");
        let right = self.right.iter().map(|v| format!("{: >2}", v)).join(" ");
        write!(f, "{} | {}", left, right)
    }
}
