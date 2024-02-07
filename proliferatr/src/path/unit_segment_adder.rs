use derive_builder::Builder;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rustc_hash::FxHashSet;

use crate::{bound::Bound2D, direction::Cardinal, point::Point};

use super::{PathMutator, PointPath};

/// Randomly adds segments to a path by inserting two additional points between
/// existing pairs of points, making a specified number of passes through the
/// points that comprise the path.
///
/// If the bounds are specified, this will skip adding segments that would lie
/// outside of those bounds.
///
/// In order to prevent generating paths that contain crossing segments, this
/// requires that the path being mutated are unit-paths, in that every segment
/// described by two points is one unit long. If the path is a unit-path, this
/// is guaranteed to produce paths that do not cross themselves.
///
/// Because this constructs an internal hash set of points to prevent
/// self-crossing, it is more expensive to call `.mutate(...)` multiple times
/// than it is to set the number of passes to a value greater than one.
///
/// Specific points can be marked as off-limits for the mutations by using the
/// [insert_avoided] method. The set of avoided points can be cleared with
/// [clear_avoided].
///
/// # Examples
/// ```
/// use rand::thread_rng;
/// use proliferatr::path::{Path, PathMutator, PointPath, UnitSegmentAdder};
///
/// let mut p = Path::from_iter([
///     (0, 0).into(),
///     (1, 0).into(),
///     (2, 0).into(),
///     (2, 1).into()
/// ]);
///
/// let mut adder = UnitSegmentAdder::builder()
///     .passes(3)
///     .rng(thread_rng())
///     .build()
///     .unwrap();
///
/// adder.mutate(&mut p);
/// ```
#[derive(Debug, Clone, PartialEq, Builder)]
pub struct UnitSegmentAdder<R>
where
    R: Rng + Clone + ?Sized,
{
    /// The optional bound for this mutator.
    #[builder(default, setter(into, strip_option))]
    bounds: Option<Bound2D>,

    /// The number of iterations through the list of points.
    #[builder(default = "1")]
    passes: usize,

    /// The probability that this will attempt to insert a segment between two
    /// existing points.
    #[builder(default = "0.5")]
    attempt: f64,

    /// The probability that the two added points will be shifted in the
    /// positive direction from the existing pair of points.
    #[builder(default = "0.5")]
    plus_bias: f64,

    /// The random number generator for internal use.
    rng: R,

    #[builder(default)]
    avoid: FxHashSet<Point>,
}

impl Default for UnitSegmentAdder<ThreadRng> {
    fn default() -> Self {
        Self {
            bounds: None,
            passes: 1,
            attempt: 0.5,
            plus_bias: 0.5,
            rng: thread_rng(),
            avoid: FxHashSet::default(),
        }
    }
}

impl<R> UnitSegmentAdder<R>
where
    R: Rng + Clone + ?Sized,
{
    pub fn builder() -> UnitSegmentAdderBuilder<R> {
        UnitSegmentAdderBuilder::default()
    }

    pub fn insert_avoided(&mut self, point: Point) {
        self.avoid.insert(point);
    }

    pub fn clear_avoided(&mut self) {
        self.avoid.clear();
    }
}

impl<R> PathMutator for UnitSegmentAdder<R>
where
    R: Rng + Clone + ?Sized,
{
    fn mutate<P: PointPath>(&mut self, path: &mut P) -> bool {
        // extend the avoid cache with the current set of points
        self.avoid.extend(path.points().copied());

        let mut any_mutations = false;

        for _ in 0..self.passes {
            // walk from the end of the path to the front so we can insert points
            // without messing up our index iterator.
            for i in (0..(path.len() - 1)).rev() {
                let mut p1 = path.get(i).copied().unwrap();
                let mut p2 = path.get(i + 1).copied().unwrap();

                if self.rng.gen_bool(self.attempt) {
                    if let Some(dir) = p1.cardinal_to(&p2) {
                        match dir {
                            Cardinal::East | Cardinal::West => {
                                if self.rng.gen_bool(self.plus_bias) {
                                    p1.y += 1;
                                    p2.y += 1;
                                } else {
                                    p1.y -= 1;
                                    p2.y -= 1;
                                }
                            }
                            Cardinal::North | Cardinal::South => {
                                if self.rng.gen_bool(self.plus_bias) {
                                    p1.x += 1;
                                    p2.x += 1;
                                } else {
                                    p1.x -= 1;
                                    p2.x -= 1;
                                }
                            }
                        }

                        if let Some(ref bounds) = self.bounds {
                            if !bounds.contains(&p1) || !bounds.contains(&p2) {
                                continue;
                            }
                        }

                        if !self.avoid.contains(&p1) && !self.avoid.contains(&p2) {
                            self.avoid.insert(p1);
                            self.avoid.insert(p2);
                            any_mutations = true;
                            // insert in reverse order because of the shifting
                            // behavior
                            path.insert(i + 1, p2);
                            path.insert(i + 1, p1);
                        }
                    }
                }
            }
        }

        any_mutations
    }
}
