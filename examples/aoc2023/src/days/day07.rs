use std::{collections::HashSet, convert::Infallible, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const NUM_HANDS: usize = 1000;
const HAND_SIZE: usize = 5;
const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const BID_RANGE: Range<i64> = 1..1000;

// simply generate a list of hands, then shuffle them, making sure to include
// at least the default set of hands. Hands need to be unique strings to avoid
// ordering issues.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day07;

impl Day for Day07 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Self.gen_input(rng)?.iter().join("\n"))
    }
}

impl InputGenerator for Day07 {
    type GeneratorError = Infallible;
    type Output = Vec<Hand>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut seen = HashSet::with_capacity(NUM_HANDS);
        let mut out = vec![
            Hand {
                cards: "AAAAA".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "KKKKK".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "JJJJJ".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "888TT".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "22QQ2".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "4444J".into(),
                bid: rng.gen_range(BID_RANGE),
            },
            Hand {
                cards: "53JJJ".into(),
                bid: rng.gen_range(BID_RANGE),
            },
        ];

        for h in out.iter() {
            seen.insert(h.cards.clone());
        }

        // we need to generate 1000 total hands
        for _ in 0..(NUM_HANDS - out.len()) {
            loop {
                let hand = Hand::random(rng);
                if seen.contains(&hand.cards) {
                    continue;
                }
                seen.insert(hand.cards.clone());
                out.push(hand);
                break;
            }
        }

        out.shuffle(rng);

        Ok(out)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    cards: String,
    bid: i64,
}

impl Hand {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let dist = Uniform::from(0..CARDS.len());
        let mut cards = ['A'; HAND_SIZE];

        #[allow(clippy::needless_range_loop)]
        for i in 0..HAND_SIZE {
            cards[i] = CARDS[dist.sample(rng)];
        }

        let bid = rng.gen_range(BID_RANGE);

        Self {
            cards: cards.iter().collect(),
            bid,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.cards, self.bid)
    }
}
