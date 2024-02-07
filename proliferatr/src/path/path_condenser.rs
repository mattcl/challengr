use derive_builder::Builder;
use itertools::Itertools;

use super::{PathMutator, PointPath};

/// A [PathMutator] that removes non-critical points from the [PointPath].
///
/// # Examples
/// ```
/// use proliferatr::path::{ClosedPath, PathCondenser, PathMutator, PointPath};
///
/// let mut p = ClosedPath::rect_path(10, 15).unwrap();
/// assert_eq!(p.len(), 47);
///
/// let mut condenser = PathCondenser::builder()
///     .build()
///     .unwrap();
///
/// condenser.mutate(&mut p);
///
/// // we expect to just be left witht he corners and the duplicated starting
/// // point for the closed path
/// assert_eq!(p.len(), 5);
/// assert_eq!(p.get(0).copied(), Some((0, 0).into()));
/// assert_eq!(p.get(1).copied(), Some((0, 14).into()));
/// assert_eq!(p.get(2).copied(), Some((9, 14).into()));
/// assert_eq!(p.get(3).copied(), Some((9, 0).into()));
/// assert_eq!(p.get(4).copied(), Some((0, 0).into()));
///
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
pub struct PathCondenser {
    #[builder(default)]
    limit: usize,
}

impl PathCondenser {
    pub fn builder() -> PathCondenserBuilder {
        PathCondenserBuilder::default()
    }

    /// Return the first index that could be removed, if it exists.
    ///
    /// This is mainly for convenience if, for some reason, we want to find a
    /// point to remove but don't want to remove it immediately.
    pub fn first_candidate<T: PointPath>(&self, start: usize, path: T) -> Option<usize> {
        for (i1, i2, i3) in (start..path.len()).tuple_windows() {
            let p1 = path.get(i1).unwrap();
            let p2 = path.get(i2).unwrap();
            let p3 = path.get(i3).unwrap();

            if let (Some(d1), Some(d2)) = (p1.cardinal_to(p2), p2.cardinal_to(p3)) {
                if d1 == d2 {
                    return Some(i2);
                }
            }
        }

        None
    }
}

impl PathMutator for PathCondenser {
    fn mutate<P: PointPath>(&mut self, path: &mut P) -> bool {
        let mut removals = 0;

        let mut len = path.len();
        let mut start = 1;

        if len < 3 {
            return false;
        }

        while (self.limit == 0 || removals < self.limit) && start < len - 1 {
            let p1 = path.get(start - 1).unwrap();
            let p2 = path.get(start).unwrap();
            let p3 = path.get(start + 1).unwrap();

            if let (Some(d1), Some(d2)) = (p1.cardinal_to(p2), p2.cardinal_to(p3)) {
                if d1 == d2 {
                    path.remove(start);
                    removals += 1;
                    len -= 1;
                    continue;
                }
            }

            start += 1;
        }

        removals > 0
    }
}
