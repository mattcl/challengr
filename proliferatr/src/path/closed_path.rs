use std::collections::VecDeque;

use thiserror::Error;

use crate::point::Point;

use super::{PathError, PointPath};

#[derive(Debug, Clone, Error)]
pub enum ClosedPathError {
    #[error("Width ({width}) and height ({height}) must be greater than 1.")]
    InvalidDimension { width: usize, height: usize },
}

/// A closed (loop) latice path in a 2D coordinate plane.
///
/// For convenience, the first and last points in this path are identical.
///
/// Note: there is no internal mechanism for preventing you from manually
/// breaking the loop or manually altering the path such that is is no longer
/// a latice path.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClosedPath {
    points: VecDeque<Point>,
}

impl ClosedPath {
    /// Make a rectangular path of unit segments wiht (0, 0) as the lower left
    /// corner. Clockwise winding order.
    ///
    /// Width and height must each be greater than one.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::path::{ClosedPath, PointPath};
    ///
    /// let w = 10;
    /// let h = 15;
    /// let p = ClosedPath::rect_path(w, h).unwrap();
    ///
    /// // the width and height include the corners, so remove the overlap (4)
    /// // and add on the extra point at the end of the path that is a duplicate
    /// // of the start point
    /// assert_eq!(p.len(), w * 2 + h * 2 - 4 + 1);
    ///
    /// // as with all ClosedPath, the first and last point are the same.
    /// assert_eq!(p.get(0).unwrap(), p.get(p.len() - 1).unwrap());
    /// ```
    pub fn rect_path(width: usize, height: usize) -> Result<Self, PathError> {
        if width < 1 || height < 1 {
            return Err(ClosedPathError::InvalidDimension { width, height }.into());
        }

        // this is close enough
        let mut raw = VecDeque::with_capacity(width * 2 + height * 2 - 4 + 1);
        let mut cur = Point::default();

        raw.push_back(cur);

        for _ in 1..height {
            cur.y += 1;
            raw.push_back(cur);
        }

        for _ in 1..width {
            cur.x += 1;
            raw.push_back(cur);
        }

        for _ in 1..height {
            cur.y -= 1;
            raw.push_back(cur);
        }

        // this will end up placing a point on the list that's equal to the
        // start (0, 0)
        for _ in 1..width {
            cur.x -= 1;
            raw.push_back(cur);
        }

        assert_eq!(raw[0], raw[raw.len() - 1]);

        Ok(Self { points: raw })
    }
}

impl PointPath for ClosedPath {
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
