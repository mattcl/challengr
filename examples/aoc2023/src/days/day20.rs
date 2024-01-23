use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use proliferatr::{
    generic::{token::LOWER_ALPHA_CHARS, StringToken},
    InputGenerator,
};
use rand::{seq::SliceRandom, Rng};

use super::Day;

// Yeah, just going to hardcode these. These are all the primes between 2048 to
// 4095 (to ensure we have a 1 in bit 12).
const PRIME_CHOICES: &[u32] = &[
    2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153,
    2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293,
    2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399,
    2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549,
    2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683,
    2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731, 2741, 2749, 2753, 2767, 2777, 2789,
    2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897, 2903, 2909,
    2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049,
    3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169, 3181, 3187, 3191, 3203,
    3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259, 3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329,
    3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407, 3413, 3433, 3449, 3457, 3461, 3463,
    3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539, 3541, 3547, 3557, 3559, 3571, 3581,
    3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637, 3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701,
    3709, 3719, 3727, 3733, 3739, 3761, 3767, 3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847,
    3851, 3853, 3863, 3877, 3881, 3889, 3907, 3911, 3917, 3919, 3923, 3929, 3931, 3943, 3947, 3967,
    3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027, 4049, 4051, 4057, 4073, 4079, 4091, 4093,
];
const NUM_ADDERS: usize = 4;
const NUM_BITS: usize = 12;
const KEY_LEN: usize = 2;

/// We have 4, 12-bit adders that we're going to configure such that when they
/// reach a particular 12-bit prime, will cause their conjunction to emit a low
/// pulse.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day20;

impl Day for Day20 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Day20.gen_input(rng)
    }
}

impl InputGenerator for Day20 {
    type GeneratorError = anyhow::Error;
    type Output = String;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let key_gen = StringToken::builder()
            .length(KEY_LEN..=KEY_LEN)
            .charset(LOWER_ALPHA_CHARS)
            .build()
            .unwrap();

        let desired_keys = (NUM_BITS + 2) * 4;
        let mut keys: Vec<String> = Vec::with_capacity(desired_keys);
        let mut seen_keys: HashSet<String> = HashSet::with_capacity(desired_keys + 3);

        seen_keys.insert("rx".into());
        let final_key = key_gen.gen_input(rng)?;
        seen_keys.insert(final_key.clone());

        let mut final_conjuction = Component::new(ComponentKind::Conjunction, &final_key);
        final_conjuction.links.push("rx");

        let primes = PRIME_CHOICES
            .choose_multiple(rng, NUM_ADDERS)
            .copied()
            .collect::<Vec<_>>();
        let mut adders = Vec::with_capacity(NUM_ADDERS);

        while keys.len() < desired_keys {
            let key = key_gen.gen_input(rng)?;
            if seen_keys.contains(&key) {
                continue;
            }

            seen_keys.insert(key.clone());
            keys.push(key);
        }

        for (idx, key_group) in keys.chunks(NUM_BITS + 2).enumerate() {
            adders.push(Adder::new(rng, primes[idx], &final_key, key_group))
        }

        let mut broadcaster = Component::new(ComponentKind::Broadcaster, "broadcaster");
        for adder in adders.iter() {
            broadcaster.links.push(adder.bits[0].key);
        }

        let mut out = Vec::with_capacity(desired_keys + 2);
        out.push(final_conjuction.to_string());
        out.push(broadcaster.to_string());

        for adder in adders.iter() {
            adder.populate(&mut out);
        }

        out.shuffle(rng);

        Ok(out.join("\n"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentKind {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

impl Display for ComponentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FlipFlop => "%",
            Self::Conjunction => "&",
            Self::Broadcaster => "",
        }
        .fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Component<'a> {
    kind: ComponentKind,
    key: &'a str,
    links: Vec<&'a str>,
}

impl<'a> Component<'a> {
    pub fn new(kind: ComponentKind, key: &'a str) -> Self {
        Self {
            kind,
            key,
            links: Vec::default(),
        }
    }
}

impl<'a> Display for Component<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            self.kind,
            self.key,
            self.links.iter().join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Adder<'a> {
    bits: Vec<Component<'a>>,
    conjunction: Component<'a>,
    nand: Component<'a>,
}

impl<'a> Adder<'a> {
    pub fn new<R: Rng + Clone + ?Sized>(
        rng: &mut R,
        prime: u32,
        final_conjuction: &'a str,
        keys: &'a [String],
    ) -> Self {
        let mut bits: Vec<Component> = (0..NUM_BITS)
            .map(|i| Component::new(ComponentKind::FlipFlop, keys[i].as_str()))
            .collect();
        let mut conjunction = Component::new(ComponentKind::Conjunction, keys[12].as_str());
        let mut nand = Component::new(ComponentKind::Conjunction, keys[13].as_str());
        nand.links.push(final_conjuction);

        for i in 0..NUM_BITS {
            let bit = 1_u32 << i;
            if prime & bit != 0 {
                if i < NUM_BITS - 1 {
                    bits[i].links.push(keys[12].as_str());
                    bits[i].links.push(keys[i + 1].as_str());
                } else {
                    bits[i].links.push(keys[12].as_str());
                }
            } else if i < NUM_BITS - 1 {
                bits[i].links.push(keys[i + 1].as_str());
            }

            if prime & bit == 0 || i == 0 {
                conjunction.links.push(keys[i].as_str());
            }
        }

        // insert the NAND node into a random position after the first index
        // we probably don't have to handle the edge case where this is empty,
        // since it should not be possible
        conjunction
            .links
            .insert(rng.gen_range(1..conjunction.links.len()), keys[13].as_str());

        Self {
            bits,
            conjunction,
            nand,
        }
    }

    pub fn populate(&self, out: &mut Vec<String>) {
        for bit in self.bits.iter() {
            out.push(bit.to_string());
        }

        out.push(self.conjunction.to_string());
        out.push(self.nand.to_string());
    }
}
