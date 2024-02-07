use std::collections::VecDeque;

use thiserror::Error;

use crate::point::Point;

mod closed_path;
mod path_condenser;
mod path_reflector;
mod path_scaler;
mod unit_segment_adder;

pub use closed_path::{ClosedPath, ClosedPathError};
pub use path_condenser::{PathCondenser, PathCondenserBuilder, PathCondenserBuilderError};
pub use path_reflector::{BothAxisReflector, XAxisReflector, YAxisReflector};
pub use path_scaler::{PathScaler, PathScalerBuilder, PathScalerBuilderError};
pub use unit_segment_adder::{
    UnitSegmentAdder, UnitSegmentAdderBuilder, UnitSegmentAdderBuilderError,
};

/// Indicates that this type describes a 2D path formed by the traversal of a
/// collection of [Point].
pub trait PointPath {
    /// The number of points that describe this path.
    fn len(&self) -> usize;

    /// Get the [Point] at the specified `idx`, if it exists.
    fn get(&self, idx: usize) -> Option<&Point>;

    /// Get an iterator through the points that describe this path.
    fn points(&self) -> impl Iterator<Item = &Point>;

    /// Get an iterator of the mutable [Point] references that make up this path.
    fn points_mut(&mut self) -> impl Iterator<Item = &mut Point>;

    /// Insert the specified point at `idx`.
    fn insert(&mut self, idx: usize, point: Point);

    /// Insert the specified points between the points at `idx - 1` and `idx`.
    fn insert_many<I: Iterator<Item = Point>>(&mut self, idx: usize, points: I);

    /// Remove the [Point] at `idx`, if it exists.
    ///
    /// Returns the [Point] if it did exist.
    fn remove(&mut self, idx: usize) -> Option<Point>;

    /// Translates all the points of `self` by `dxdy` by adding `dxdy` to every
    /// [Point] in the path.
    fn translate(&mut self, dxdy: Point);

    /// Returns `true` if this path is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Path mutators mutate a given [PointPath], by optionally adding, removing,
/// and/or altering the points of that path.
///
/// An example would be the [PathCondenser], that removes non-critial points
/// from a path.
pub trait PathMutator {
    /// Attempt to mutate the given path.
    ///
    /// Returns `true` if the path was mutated.
    fn mutate<P: PointPath>(&mut self, path: &mut P) -> bool;
}

#[derive(Debug, Clone, Error)]
pub enum PathError {
    #[error(transparent)]
    ClosedPath(#[from] ClosedPathError),
}

/// A sequence of [Point] describing a 2D path.
///
/// # Examples
/// ```
/// use proliferatr::path::{Path, PointPath};
///
/// let mut p = Path::default();
/// p.append((0, 0).into());
/// p.append((0, 2).into());
/// p.append((3, 2).into());
///
/// assert_eq!(p.len(), 3);
/// assert_eq!(p.get(2).copied(), Some((3, 2).into()));
///
/// p.prepend((-1, 0).into());
/// assert_eq!(p.len(), 4);
/// assert_eq!(p.get(0).copied(), Some((-1, 0).into()));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    points: VecDeque<Point>,
}

// We could probably DerefMut to give access to the underlying VecDeque, but
// this would make changing the internal storage a breaking change if that ever
// happened.
impl Path {
    /// Append this [Point] to the path.
    ///
    /// This does not validate that the [Point] does not already exist in the
    /// path, nor does it validate that the path is non-self-intersecting.
    pub fn append(&mut self, point: Point) {
        self.points.push_back(point);
    }

    /// Prepend this [Point] to the path.
    ///
    /// This does not validate that the [Point] does not already exist in the
    /// path, nor does it validate that the path is non-self-intersecting.
    pub fn prepend(&mut self, point: Point) {
        self.points.push_front(point);
    }
}

impl FromIterator<Point> for Path {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Self {
            points: VecDeque::from_iter(iter),
        }
    }
}

impl PointPath for Path {
    fn len(&self) -> usize {
        self.points.len()
    }

    fn get(&self, idx: usize) -> Option<&Point> {
        self.points.get(idx)
    }

    fn points(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    fn points_mut(&mut self) -> impl Iterator<Item = &mut Point> {
        self.points.iter_mut()
    }

    fn insert(&mut self, idx: usize, point: Point) {
        self.points.insert(idx, point);
    }

    fn insert_many<I: Iterator<Item = Point>>(&mut self, idx: usize, points: I) {
        for (offset, p) in points.enumerate() {
            self.points.insert(idx + offset, p);
        }
    }

    fn remove(&mut self, idx: usize) -> Option<Point> {
        self.points.remove(idx)
    }

    fn translate(&mut self, dxdy: Point) {
        for p in self.points.iter_mut() {
            *p += dxdy;
        }
    }
}
