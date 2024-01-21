use std::{collections::HashSet, convert::Infallible, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::Rng;

use super::Day;

const NUM_HAIL: usize = 300;
const MIN: i64 = 200_000_000_000_000;
const MAX: i64 = 400_000_000_000_000;
const VELOCITY: Range<i64> = -256..257;
const IMPACT_TIME: Range<i64> = 100_000_000_000..500_000_000_000;

/// Pick a collision location within MIN..MAX. Generate 300 hailstones that
/// all converge on the selected location. Pick a random velocity for the thrown
/// stone. and add that velocity to the velocity of all the other stones.
///
/// We're going to make sure we don't have any duplicate velocities in the final
/// output, and that we have no zero velocities in any direction.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day24;

impl Day for Day24 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day24.gen_input(rng)?.iter().join("\n"))
    }
}

impl InputGenerator for Day24 {
    type GeneratorError = Infallible;
    type Output = Vec<Hail>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let target = Point {
            x: rng.gen_range(MIN..=MAX),
            y: rng.gen_range(MIN..=MAX),
            z: rng.gen_range(MIN..=MAX),
        };

        let thrown_velocity = Point::random_velocity(rng);

        let mut seen_velocities = HashSet::with_capacity(NUM_HAIL);
        let mut seen_times = HashSet::with_capacity(NUM_HAIL);
        let mut hail = Vec::with_capacity(NUM_HAIL);

        while hail.len() < NUM_HAIL {
            let vel = Point::random_velocity(rng);

            let time = loop {
                let t = rng.gen_range(IMPACT_TIME);
                if !seen_times.contains(&t) {
                    seen_times.insert(t);
                    break t;
                }
            };

            // calculate the origin of this hailstone by back-tracking the time
            let origin = Point {
                x: target.x - vel.x * time,
                y: target.y - vel.y * time,
                z: target.z - vel.z * time,
            };

            // we now can move the hailstone out of the frame of the thrown stone
            let adjusted_vel = Point {
                x: vel.x + thrown_velocity.x,
                y: vel.y + thrown_velocity.y,
                z: vel.z + thrown_velocity.z,
            };

            // don't duplicate velocities, and ensure none of the velocity
            // components are zero
            if seen_velocities.contains(&adjusted_vel)
                || adjusted_vel.x == 0
                || adjusted_vel.y == 0
                || adjusted_vel.z == 0
            {
                continue;
            }

            seen_velocities.insert(adjusted_vel);

            hail.push(Hail {
                pos: origin,
                vel: adjusted_vel,
            });
        }

        // dbg!(target, target.x + target.y + target.z);

        Ok(hail)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hail {
    pos: Point,
    vel: Point,
}

impl Display for Hail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.pos, self.vel)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn random_velocity<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let x = loop {
            let v = rng.gen_range(VELOCITY);
            if v != 0 {
                break v;
            }
        };
        let y = loop {
            let v = rng.gen_range(VELOCITY);
            if v != 0 {
                break v;
            }
        };
        let z = loop {
            let v = rng.gen_range(VELOCITY);
            if v != 0 {
                break v;
            }
        };

        Point { x, y, z }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}
