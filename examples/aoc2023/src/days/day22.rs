use std::{convert::Infallible, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use super::Day;

const NUM_BRICKS: Range<usize> = 1250..1301;
const DIMENSION: usize = 10;
const HEIGHT: usize = 300;
// we want almost 100% of all generated lines to be horizontal
const XY_BIAS: f64 = 0.95;
const X_BIAS: f64 = 0.65;

/// Strategy is to randomly generate line segments that can be oriented along
/// any axis, with a bias toward x/y and the particular edge of he chosen
/// direction. We need to ensure that bricks do not get generated such that
/// they are inside of each other.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day22;

impl Day for Day22 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError> {
        Ok(Day22.gen_input(rng)?.iter().join("\n"))
    }
}

impl InputGenerator for Day22 {
    type GeneratorError = Infallible;
    type Output = Vec<Line>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let num_bricks = rng.gen_range(NUM_BRICKS);
        let mut out = Vec::with_capacity(num_bricks);
        // memory is cheap, right?. This isn't actually that large
        let mut seen = vec![vec![vec![false; HEIGHT]; DIMENSION]; DIMENSION];

        let xy_coord_dist = Uniform::from(0..DIMENSION);
        let xy_edge_dist = Uniform::from(0..DIMENSION / 2);
        let z_coord_dist = Uniform::from(1..HEIGHT);

        'outer: while out.len() < num_bricks {
            let z = z_coord_dist.sample(rng);

            let candidate = if !rng.gen_bool(XY_BIAS) {
                let x = xy_coord_dist.sample(rng);
                let y = xy_coord_dist.sample(rng);

                let start = Point { x, y, z };

                // z
                Line {
                    left: start,
                    right: Point {
                        x: start.x,
                        y: start.y,
                        z: rng.gen_range(start.z..HEIGHT),
                    },
                }
            } else if rng.gen_bool(X_BIAS) {
                // x
                let x = xy_edge_dist.sample(rng);
                let y = xy_coord_dist.sample(rng);

                let start = Point { x, y, z };
                Line {
                    left: start,
                    right: Point {
                        x: rng.gen_range(start.x..DIMENSION),
                        y: start.y,
                        z: start.z,
                    },
                }
            } else {
                // y
                let x = xy_coord_dist.sample(rng);
                let y = xy_edge_dist.sample(rng);

                let start = Point { x, y, z };

                Line {
                    left: start,
                    right: Point {
                        x: start.x,
                        y: rng.gen_range(start.y..DIMENSION),
                        z: start.z,
                    },
                }
            };

            for (cx, cy, cz) in candidate.points() {
                if seen[cx][cy][cz] {
                    continue 'outer;
                }
            }

            for (cx, cy, cz) in candidate.points() {
                seen[cx][cy][cz] = true;
            }

            out.push(candidate);
        }

        Ok(out)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    left: Point,
    right: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}~{}", self.left, self.right)
    }
}

impl Line {
    pub fn points(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (self.left.x..=self.right.x).flat_map(move |x| {
            (self.left.y..=self.right.y)
                .flat_map(move |y| (self.left.z..=self.right.z).map(move |z| (x, y, z)))
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
