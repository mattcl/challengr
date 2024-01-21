use std::{convert::Infallible, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

const JOINING_CHARS: &[u8] = b".?";
const GROUP_CHARS: &[u8] = b"#?";
const NUM_GROUPS: Range<usize> = 1..6;
const GROUP_SIZE: Range<usize> = 1..7;
const GROUP_SEPARATION: Range<usize> = 1..4;
const NUM_LINES: usize = 1000;

/// We're just going to randomly generate these strings and hope we don't
/// overflow our integer container. The real inputs look like they have some
/// hand-selected lines, but we're not going to bother.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day12;

impl Day for Day12 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day12.gen_input(rng)?.join("\n"))
    }
}

impl InputGenerator for Day12 {
    type GeneratorError = Infallible;
    type Output = Vec<String>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        Ok((0..NUM_LINES).map(|_| make_line(rng)).collect())
    }
}

fn make_group<R: Rng + Clone + ?Sized>(rng: &mut R, size: usize) -> String {
    (0..size)
        .map(|_| *GROUP_CHARS.choose(rng).unwrap() as char)
        .collect()
}

fn make_separator<R: Rng + Clone + ?Sized>(rng: &mut R, size: usize) -> String {
    (0..size)
        .map(|_| *JOINING_CHARS.choose(rng).unwrap() as char)
        .collect()
}

fn make_line<R: Rng + Clone + ?Sized>(rng: &mut R) -> String {
    let num_groups = rng.gen_range(NUM_GROUPS);
    let mut out = Vec::with_capacity(num_groups * 2 - 1);
    let mut groups = Vec::with_capacity(num_groups);

    for i in 0..num_groups {
        let group_size = rng.gen_range(GROUP_SIZE);
        out.push(make_group(rng, group_size));
        groups.push(char::from_digit(group_size as u32, 10).unwrap());

        let sep_size = rng.gen_range(GROUP_SEPARATION);

        // 50% with trailing separator
        if i != num_groups - 1 || rng.gen_bool(0.5) {
            out.push(make_separator(rng, sep_size));
        }
    }

    // not the most efficient thing with the string allocs
    format!("{} {}", out.join(""), groups.iter().join(","))
}
