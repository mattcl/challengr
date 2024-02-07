use std::ops::AddAssign;

use crate::direction::Cardinal;

/// A 2D coordinate representation of `(x, y)`.
///
/// # Examples
/// ```
/// use proliferatr::point::Point;
/// let p1 = Point::new(2, 3);
/// let p2: Point = (2, 3).into();
/// let p3 = Point { x: -4, y: 7 };
///
/// assert_eq!(p1, p2);
/// assert_ne!(p1, p3);
///
/// assert_eq!(p1.x, 2);
/// assert_eq!(p1.y, 3);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Construct a new [Point].
    ///
    /// # Examples
    /// ```
    /// use proliferatr::point::Point;
    /// let p = Point::new(4, -5);
    ///
    /// assert_eq!(p.x, 4);
    /// assert_eq!(p.y, -5);
    /// ```
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    /// Return the cardinal direction from `self` to `other`.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::direction::Cardinal;
    /// use proliferatr::point::Point;
    ///
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(3, 0);
    ///
    /// assert_eq!(p1.cardinal_to(&p2), Some(Cardinal::East));
    /// assert_eq!(p2.cardinal_to(&p1), Some(Cardinal::West));
    ///
    /// let p2 = Point::new(0, 3);
    /// assert_eq!(p1.cardinal_to(&p2), Some(Cardinal::North));
    /// assert_eq!(p2.cardinal_to(&p1), Some(Cardinal::South));
    ///
    /// // equal points have no relative direction
    /// assert_eq!(p1.cardinal_to(&p1), None);
    ///
    /// // ordinal points have no relative cardinal direction
    /// let p2 = Point::new(2, 3);
    /// assert_eq!(p1.cardinal_to(&p2), None);
    /// ```
    pub fn cardinal_to(&self, other: &Self) -> Option<Cardinal> {
        if self == other {
            None
        } else if self.x == other.x {
            if self.y < other.y {
                Some(Cardinal::North)
            } else {
                Some(Cardinal::South)
            }
        } else if self.y == other.y {
            if self.x < other.x {
                Some(Cardinal::East)
            } else {
                Some(Cardinal::West)
            }
        } else {
            None
        }
    }

    /// Return the reflection of this [Point] across the x-axis.
    ///
    /// This is equivalent to creating a new point with the sign of the `y`
    /// component changed.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::point::Point;
    ///
    /// let p1 = Point::new(2, 3);
    /// let p2 = p1.reflect_x();
    /// assert_eq!(p2, Point::new(2, -3));
    /// ```
    pub fn reflect_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    /// Reflect `self` across the x-axis.
    ///
    /// This is equivalent to changing the sign of the `y` component.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::point::Point;
    ///
    /// let mut p1 = Point::new(2, 3);
    /// p1.reflect_x_mut();
    /// assert_eq!(p1, Point::new(2, -3));
    /// ```
    pub fn reflect_x_mut(&mut self) {
        self.y = -self.y;
    }

    /// Return the reflection of this [Point] across the y-axis.
    ///
    /// This is equivalent to creating a new point with the sign of the `x`
    /// component changed.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::point::Point;
    ///
    /// let p1 = Point::new(2, 3);
    /// let p2 = p1.reflect_y();
    /// assert_eq!(p2, Point::new(-2, 3));
    /// ```
    pub fn reflect_y(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    /// Reflect `self` across the y-axis.
    ///
    /// This is equivalent to changing the sign of the `x` component.
    ///
    /// # Examples
    /// ```
    /// use proliferatr::point::Point;
    ///
    /// let mut p1 = Point::new(2, 3);
    /// p1.reflect_y_mut();
    /// assert_eq!(p1, Point::new(-2, 3));
    /// ```
    pub fn reflect_y_mut(&mut self) {
        self.x = -self.x;
    }

    /// Returns the Manhattan distance between `self` and `other`.
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
