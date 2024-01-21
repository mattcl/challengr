use std::{collections::VecDeque, convert::Infallible, fmt::Display, hash::BuildHasherDefault};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};
use rustc_hash::FxHashSet;

use super::Day;

const INITIAL_SPACING: i64 = 1;
const INITIAL_EDGE_LENGTH: i64 = INITIAL_SPACING * 30;
const NUM_ALTERATIONS: usize = 30;
const ALTERATIONS: &[Alt] = &[Alt::Nothing, Alt::Expand, Alt::Contract];
// const MAX_5_DIGIT_HEX: i64 = 1048575;

/// Strategy here is to use a similar growing algorithm to day 10 for making two
/// shapes with an equal number of verticies. We then condense each shape the
/// same number of times to remove verticies that aren't corners. From here, we
/// can scale these shapes in x and y to produce a small and large shape before
/// translating the shapes into digging instructions.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day18;

impl Day for Day18 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day18.gen_input(rng)?.iter().join("\n"))
    }
}

impl InputGenerator for Day18 {
    type GeneratorError = Infallible;
    type Output = Vec<InstructionPair>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut shape1 = make_polygon(rng, 0);
        let mut shape2 = make_polygon(rng, shape1.len());

        // condense both shapes' points until one cannot be shrunk more
        let mut s1_start = 0;
        let mut s2_start = 0;
        while let (Some(r1), Some(r2)) = (
            removal_candiate(&shape1, s1_start),
            removal_candiate(&shape2, s2_start),
        ) {
            shape1.remove(r1);
            shape2.remove(r2);
            s1_start = r1;
            s2_start = r2;
        }

        // we're going to scale shape1 to make it a little bigger
        let x_factor = 2;
        let y_factor = 3;
        for p in shape1.iter_mut() {
            p.scale_x(x_factor);
            p.scale_y(y_factor);
        }

        // we're going to scale shape2 to make it much bigger
        let x_factor = rng.gen_range(10000..27100);
        let y_factor = rng.gen_range(10000..27100);
        for p in shape2.iter_mut() {
            p.scale_x(x_factor);
            p.scale_y(y_factor);
        }

        let mut instructions = Vec::with_capacity(shape1.len() - 1);

        for i in 0..(shape1.len() - 1) {
            let dir = shape1[i].dir_to(&shape1[i + 1]);
            let dist = shape1[i].dist(&shape1[i + 1]);
            let hex_dir = shape2[i].dir_to(&shape2[i + 1]);
            let hex_dist = shape2[i].dist(&shape2[i + 1]);

            instructions.push(InstructionPair {
                dir,
                dist,
                hex_dir,
                hex_dist,
            });
        }

        Ok(instructions)
    }
}

fn make_polygon<R: Rng + Clone + ?Sized>(rng: &mut R, point_constraint: usize) -> VecDeque<Point> {
    // VecDeque for better insert behavior
    let mut points: VecDeque<Point> = VecDeque::with_capacity(1000);
    let mut occupied: FxHashSet<Point> =
        FxHashSet::with_capacity_and_hasher(1000, BuildHasherDefault::default());

    // make a square like we did for 10
    let mut cur = Point::default();

    points.push_back(cur);
    occupied.insert(cur);

    // left
    for _ in 1..INITIAL_EDGE_LENGTH {
        cur.y += INITIAL_SPACING;
        points.push_back(cur);
        occupied.insert(cur);
    }

    // top
    for _ in 1..INITIAL_EDGE_LENGTH {
        cur.x += INITIAL_SPACING;
        points.push_back(cur);
        occupied.insert(cur);
    }

    // right
    for _ in 1..INITIAL_EDGE_LENGTH {
        cur.y -= INITIAL_SPACING;
        points.push_back(cur);
        occupied.insert(cur);
    }

    // bot
    for _ in 1..INITIAL_EDGE_LENGTH {
        cur.x -= INITIAL_SPACING;
        points.push_back(cur);
        occupied.insert(cur);
    }

    // Sanity check we got back to the same spot as we started. In this case,
    // we're going to include a duplicate of the starting point at the end of
    // the list.
    assert_eq!(points[0], points[points.len() - 1]);

    let cycles = if point_constraint > 0 {
        NUM_ALTERATIONS * 100
    } else {
        NUM_ALTERATIONS
    };

    for _ in 0..cycles {
        for i in (0..(points.len() - 1)).rev() {
            // we know `i` and `i + 1` exist
            let mut p1 = points[i];
            let mut p2 = points[i + 1];

            let orient = if p1.y == p2.y {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            };

            let alteration = ALTERATIONS.choose(rng).unwrap();
            match alteration {
                Alt::Expand => match orient {
                    Orientation::Vertical => {
                        p1.x -= 1;
                        p2.x -= 1;
                    }
                    Orientation::Horizontal => {
                        p1.y -= 1;
                        p2.y -= 1;
                    }
                },
                Alt::Contract => match orient {
                    Orientation::Vertical => {
                        p1.x += 1;
                        p2.x += 1;
                    }
                    Orientation::Horizontal => {
                        p1.y += 1;
                        p2.y += 1;
                    }
                },
                Alt::Nothing => continue,
            }

            if !occupied.contains(&p1) && !occupied.contains(&p2) {
                occupied.insert(p1);
                occupied.insert(p2);
                points.insert(i + 1, p2);
                points.insert(i + 1, p1);
            }

            if point_constraint > 0 && points.len() == point_constraint {
                return points;
            }
        }
    }

    points
}

pub fn removal_candiate(points: &VecDeque<Point>, start: usize) -> Option<usize> {
    let start = if start > 0 { start - 1 } else { 0 };

    for (i1, i2, i3) in (start..points.len()).tuple_windows() {
        let p1 = points[i1];
        let p2 = points[i2];
        let p3 = points[i3];

        let d1 = p1.dir_to(&p2);
        let d2 = p2.dir_to(&p3);

        if d1 == d2 {
            return Some(i2);
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstructionPair {
    dir: Relative,
    dist: i64,
    hex_dir: Relative,
    hex_dist: i64,
}

impl Display for InstructionPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} (#{:05x}{})",
            self.dir,
            self.dist,
            self.hex_dist,
            self.hex_dir.hex_representation()
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn dir_to(&self, other: &Self) -> Relative {
        if self.y == other.y {
            if self.x < other.x {
                Relative::Right
            } else {
                Relative::Left
            }
        } else if self.x == other.x {
            if self.y < other.y {
                Relative::Up
            } else {
                Relative::Down
            }
        } else {
            unreachable!("Attempted to get dir for points that are not cardinal neighbors")
        }
    }

    pub fn scale_x(&mut self, factor: i64) {
        self.x *= factor;
    }

    pub fn scale_y(&mut self, factor: i64) {
        self.y *= factor;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Alt {
    Nothing,
    Contract,
    Expand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Relative {
    Up,
    Down,
    Right,
    Left,
}

impl Relative {
    pub fn hex_representation(&self) -> u8 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
}

impl Display for Relative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => 'U',
            Self::Down => 'D',
            Self::Right => 'R',
            Self::Left => 'L',
        }
        .fmt(f)
    }
}
