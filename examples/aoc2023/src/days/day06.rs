use std::{convert::Infallible, fmt::Display, ops::Range};

use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution};

use super::Day;

const NUM_RECORDS: usize = 4;
const TIME_RANGE: Range<i64> = 40..100;
const VEL_MIN: i64 = 5;
const VEL_OFFSET: i64 = 15;

/// The solution for this day is the range described by
/// (time - vel) * vel > dist for unkown of vel.
///
/// We need to pick a time and vel, where vel < time to determine the dist.
///
/// The additional constraint might be that joining the digits of the times and
/// dists must also yield a time/dist for which there is an integer velocity
/// solution, but the real inputs do not appear to have integer velocites for
/// the joined number, which make this much easier.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day06;

impl Day for Day06 {
    fn generate<R: rand::Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Self {}.gen_input(rng)?.to_string())
    }
}

impl InputGenerator for Day06 {
    type GeneratorError = Infallible;
    type Output = Records;

    fn gen_input<R: rand::Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        loop {
            let mut out = Records::default();
            let t_dist = Uniform::from(TIME_RANGE);

            for i in 0..NUM_RECORDS {
                loop {
                    let time = t_dist.sample(rng);

                    if out.times.contains(&time) {
                        continue;
                    }

                    let v = rng.gen_range(VEL_MIN..=(time - VEL_OFFSET));

                    let mid = time / 2;

                    if v == time / 2 || v.max(mid) - v.min(mid) < 7 {
                        continue;
                    }

                    let dist = (time - v) * v;

                    // idk if this is actually even possible to be larger, but
                    // I can't be bothered
                    if dist < 10_000 {
                        out.times[i] = time;
                        out.dists[i] = dist;
                        out.widths[i] = if dist >= 1000 {
                            4
                        } else if dist >= 100 {
                            3
                        } else {
                            2
                        };
                        break;
                    }
                }
            }

            if out.valid() {
                return Ok(out);
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Record {
    time: i64,
    dist: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Records {
    times: [i64; NUM_RECORDS],
    dists: [i64; NUM_RECORDS],
    widths: [usize; NUM_RECORDS],
}

impl Records {
    // can't be bothered to actually math my way out of this
    pub fn valid(&self) -> bool {
        let combined_time = self.times.iter().fold(0, |acc, v| acc * 100 + v);
        let combined_dist = self.dists.iter().enumerate().fold(0, |acc, (idx, v)| {
            acc * 10_i64.pow(self.widths[idx] as u32) + v
        });

        for i in 0..NUM_RECORDS {
            if !self.check(self.times[i], self.dists[i]) {
                return false;
            }
        }

        self.check(combined_time, combined_dist)
    }

    fn check(&self, time: i64, dist: i64) -> bool {
        let t = time as f64;
        let t2 = t * t;
        let r = dist as f64;
        let b = (t2 - 4.0 * r).sqrt();

        // solutions for (time - x) * x > dist
        let lower_raw = 0.5 * (t - b);
        let upper_raw = 0.5 * (t + b);

        let mut lower = lower_raw.ceil() as i64;
        let mut upper = upper_raw.floor() as i64;

        // correct for weird errors with small numbers
        let mut attempts = 0;
        while (time - lower) * lower <= dist {
            lower += 1;
            attempts += 1;
            if attempts > 2 {
                return false;
            }
        }

        let mut attempts = 0;
        while (time - upper) * upper <= dist {
            upper -= 1;
            attempts += 1;
            if attempts > 2 {
                return false;
            }
        }

        true
    }
}

impl Display for Records {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Time:       {: >w1$}   {: >w2$}   {: >w3$}   {: >w4$}",
            self.times[0],
            self.times[1],
            self.times[2],
            self.times[3],
            w1 = self.widths[0],
            w2 = self.widths[1],
            w3 = self.widths[2],
            w4 = self.widths[3]
        )?;
        write!(
            f,
            "Distance:   {: >w1$}   {: >w2$}   {: >w3$}   {: >w4$}",
            self.dists[0],
            self.dists[1],
            self.dists[2],
            self.dists[3],
            w1 = self.widths[0],
            w2 = self.widths[1],
            w3 = self.widths[2],
            w4 = self.widths[3]
        )
    }
}
