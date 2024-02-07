use std::{convert::Infallible, hash::BuildHasherDefault, ops::Range};

use derive_builder::Builder;
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use rustc_hash::FxHashSet;

use crate::{point::Point, InputGenerator};

/// A type that can generate a unique list of random 2D Points.
///
/// # Examples
/// ```
/// use proliferatr::generic::Point2List;
/// // Ranges are i64 and exclusive of the max value,
/// // so maximum possible value for x/y in this example
/// // is 4999
/// let generator = Point2List::builder()
///     .x_range(0..5000)
///     .y_range(0..5000)
///     .num_points(500..600)
///     .build()
///     .expect("failed to build generator");
///
/// // the above configuration happens to be the default
/// assert_eq!(generator, Point2List::default());
/// ```
#[derive(Debug, Clone, Builder, PartialEq, Eq, Hash)]
pub struct Point2List {
    x_range: Range<i64>,
    y_range: Range<i64>,
    num_points: Range<usize>,
}

impl Default for Point2List {
    fn default() -> Self {
        Self {
            x_range: 0..5000,
            y_range: 0..5000,
            num_points: 500..600,
        }
    }
}

impl Point2List {
    pub fn builder() -> Point2ListBuilder {
        Point2ListBuilder::default()
    }

    pub fn gen_points<R: Rng + Clone + ?Sized>(&self, rng: &mut R) -> Vec<Point> {
        let num_points = rng.gen_range(self.num_points.clone());
        let x_dist = Uniform::from(self.x_range.clone());
        let y_dist = Uniform::from(self.y_range.clone());

        let mut seen: FxHashSet<Point> =
            FxHashSet::with_capacity_and_hasher(num_points, BuildHasherDefault::default());

        while seen.len() < num_points {
            let x = x_dist.sample(rng);
            let y = y_dist.sample(rng);

            let p = Point::new(x, y);
            if seen.contains(&p) {
                continue;
            }

            seen.insert(p);
        }

        Vec::from_iter(seen)
    }
}

impl InputGenerator for Point2List {
    type GeneratorError = Infallible;
    type Output = Vec<Point>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        Ok(self.gen_points(rng))
    }
}

/// A Point in three dimensions denoted by x, y and z.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

/// A type that can generate a unique list of random 3D Points.
///
/// # Examples
/// ```
/// use proliferatr::generic::Point3List;
/// // Ranges are i64 and exclusive of the max value,
/// // so maximum possible value for x/y in this example
/// // is 4999
/// let generator = Point3List::builder()
///     .x_range(0..5000)
///     .y_range(0..5000)
///     .z_range(0..5000)
///     .num_points(500..600)
///     .build()
///     .expect("failed to build generator");
///
/// // the above configuration happens to be the default
/// assert_eq!(generator, Point3List::default());
/// ```
#[derive(Debug, Clone, Builder, PartialEq, Eq, Hash)]
pub struct Point3List {
    x_range: Range<i64>,
    y_range: Range<i64>,
    z_range: Range<i64>,
    num_points: Range<usize>,
}

impl Default for Point3List {
    fn default() -> Self {
        Self {
            x_range: 0..5000,
            y_range: 0..5000,
            z_range: 0..5000,
            num_points: 500..600,
        }
    }
}

impl Point3List {
    pub fn builder() -> Point3ListBuilder {
        Point3ListBuilder::default()
    }

    pub fn gen_points<R: Rng + Clone + ?Sized>(&self, rng: &mut R) -> Vec<Point3> {
        let num_points = rng.gen_range(self.num_points.clone());
        let x_dist = Uniform::from(self.x_range.clone());
        let y_dist = Uniform::from(self.y_range.clone());
        let z_dist = Uniform::from(self.z_range.clone());

        let mut seen: FxHashSet<Point3> =
            FxHashSet::with_capacity_and_hasher(num_points, BuildHasherDefault::default());

        while seen.len() < num_points {
            let x = x_dist.sample(rng);
            let y = y_dist.sample(rng);
            let z = z_dist.sample(rng);

            let p = Point3 { x, y, z };
            if seen.contains(&p) {
                continue;
            }

            seen.insert(p);
        }

        Vec::from_iter(seen)
    }
}

impl InputGenerator for Point3List {
    type GeneratorError = Infallible;
    type Output = Vec<Point3>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        Ok(self.gen_points(rng))
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn point2() {
        let mut rng = thread_rng();
        let g = Point2List::default();
        let r = g.gen_input(&mut rng).unwrap();
        assert!(r.len() >= 500);
        assert!(r.len() < 600);
    }

    #[test]
    fn point3() {
        let mut rng = thread_rng();
        let g = Point3List::default();
        let r = g.gen_input(&mut rng).unwrap();
        assert!(r.len() >= 500);
        assert!(r.len() < 600);
    }
}
