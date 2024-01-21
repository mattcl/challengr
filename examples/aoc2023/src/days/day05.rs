use std::{collections::HashMap, convert::Infallible, fmt::Display};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

// These are rough estimates given my input, but we're obviously just guessing.
const NUM_SEEDS: usize = 10;
const MIN_VAL: std::ops::Range<i64> = 5_000_000..6_000_001;
const SEED_RANGE_INCREMENT: std::ops::Range<i64> = 100_000_000..900_000_001;
const SEED_RANGE_LENGTH: std::ops::Range<i64> = 50_000_000..500_000_001;
const NUM_PARTITIONS: std::ops::Range<usize> = 6..37;
const KEY_ORDER: &[Category] = &[
    Category::SeedToSoil,
    Category::SoilToFertilzier,
    Category::FertilizerToWater,
    Category::WaterToLight,
    Category::LightToTemperature,
    Category::TemperatureToHumidity,
    Category::HumidityToLocation,
];

/// This is comprised of two parts. First, the seed ranges need to be selected
/// by picking 10 numbers, starting between 5 and 6 million, and adding between
/// 100 and 900 million to determine the next number. Then, for each number,
/// we will determine how many numbers are in the range by selecting a number
/// greater than 25 million and the start of the next range, or 500 million more
/// than the start (whiever comes first).
///
/// Next, we will add between 100 million and 200 million to get the overall max
/// value for determining the remapped ranges for the mapping portion of the
/// input.
///
/// For the remapped ranges, we will decide on a number times to split the
/// overall range. We can then shuffle the order of this these ranges and pair
/// them with ranges from the original list, creating the remapping logic. We
/// can then optionally shrink the range maps to create gaps.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day05;

impl Day for Day05 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        let (seeds, mapping) = Day05.gen_input(rng)?;
        let mut out = format!("seeds: {}", seeds.iter().join(" "));
        for k in KEY_ORDER.iter() {
            out.push('\n');
            out.push('\n');
            out.push_str(&format!("{} map:\n", k));
            // this is "safe" because we made this map to have all the keys
            out.push_str(&mapping.get(k).unwrap().iter().join("\n"));
        }

        Ok(out)
    }
}

impl InputGenerator for Day05 {
    type GeneratorError = Infallible;
    type Output = (Vec<Range>, HashMap<Category, Vec<RangeMap>>);

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut start = rng.gen_range(MIN_VAL);

        let mut seeds = Vec::with_capacity(10);
        for _ in 0..NUM_SEEDS {
            let seed = Range {
                start,
                length: rng.gen_range(SEED_RANGE_LENGTH),
            };
            let end = seed.start + seed.length;

            seeds.push(seed);
            start = end.max(start + rng.gen_range(SEED_RANGE_INCREMENT));
        }

        seeds.shuffle(rng);

        let end = start;

        let mut mapping = HashMap::default();
        for (i, k) in KEY_ORDER.iter().enumerate() {
            mapping.insert(*k, generate_category(rng, 0, end, i == KEY_ORDER.len() - 1));
        }

        Ok((seeds, mapping))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    SeedToSoil,
    SoilToFertilzier,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SeedToSoil => "seed-to-soil",
            Self::SoilToFertilzier => "soil-to-fertilizer",
            Self::FertilizerToWater => "fertilizer-to-water",
            Self::WaterToLight => "water-to-light",
            Self::LightToTemperature => "light-to-temperature",
            Self::TemperatureToHumidity => "temperature-to-humidity",
            Self::HumidityToLocation => "humidity-to-location",
        }
        .fmt(f)
    }
}

// these are going to be inclusive
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
    start: i64,
    length: i64,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.start, self.length)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeMap {
    origin: i64,
    dest: i64,
    length: i64,
}

impl Display for RangeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.dest, self.origin, self.length)
    }
}

fn generate_category<R: Rng + Clone + ?Sized>(
    rng: &mut R,
    start: i64,
    end: i64,
    last: bool,
) -> Vec<RangeMap> {
    let num_partitions = rng.gen_range(NUM_PARTITIONS) as i64;
    let width = (end - start + 1) / num_partitions;
    let origins = (0..(num_partitions))
        .map(|i| i * width + start)
        .collect::<Vec<_>>();
    let mut dests = origins.clone();
    // If we're the last, the zero destination must map to something impossible
    // The real inputs do not have this "feature," but this will prevent zero
    // from ever being returned as a solution without us having to think that
    // much about it.
    if last {
        dests[0] = dests[dests.len() - 1] + width;
    }
    dests.shuffle(rng);

    while origins.iter().zip(dests.iter()).any(|(a, b)| a == b) {
        dests.shuffle(rng);
    }

    let mut ranges = Vec::with_capacity(num_partitions as usize);
    for (origin, dest) in origins.iter().copied().zip(dests.iter().copied()) {
        // we want to use anywhere from roughly 90% to 100% of the width
        let lower = (width as f64 * 0.90).floor() as i64;
        let length = rng.gen_range(lower..width);
        ranges.push(RangeMap {
            origin,
            dest,
            length,
        });
    }

    ranges.shuffle(rng);

    ranges
}
