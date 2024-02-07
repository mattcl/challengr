use derive_builder::Builder;

use crate::point::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
pub struct Bound2D {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
}

impl Bound2D {
    pub fn builder() -> Bound2DBuilder {
        Bound2DBuilder::default()
    }

    /// Initialize the bound by using the minimum and maximum values from the
    /// supplied points.
    pub fn derive_from<'a, T: Iterator<Item = &'a Point>>(iter: T) -> Self {
        let mut bounds = Self {
            min_x: i64::MAX,
            max_x: i64::MIN,
            min_y: i64::MAX,
            max_y: i64::MIN,
        };

        for p in iter {
            if p.x < bounds.min_x {
                bounds.min_x = p.x;
            }

            if p.x > bounds.max_x {
                bounds.max_x = p.x;
            }

            if p.y < bounds.min_y {
                bounds.min_y = p.y;
            }

            if p.y > bounds.max_y {
                bounds.max_y = p.y;
            }
        }

        bounds
    }

    /// Return `true` if the specified point is contained within the bound.
    pub fn contains(&self, point: &Point) -> bool {
        self.min_x <= point.x
            && point.x <= self.max_x
            && self.min_y <= point.y
            && point.y <= self.max_y
    }

    /// Normalize the point by translating it into corrdinates relative to the
    /// bound where `min_x` and `min_y` is equivalent to `(0, 0)`.
    pub fn normalize(&self, point: &Point) -> Point {
        Point {
            x: point.x - self.min_x,
            y: point.y - self.min_y,
        }
    }
}
