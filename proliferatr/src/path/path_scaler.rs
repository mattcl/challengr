use derive_builder::Builder;

use super::PathMutator;

/// A [PathMutator] that scales the points in a given [PointPath] using
/// independent `x` and `y` factors.
///
/// The configured x and y factors must be greater than zero.
///
/// # Examples
/// ```
/// use proliferatr::path::{ClosedPath, PathScaler, PathMutator, PointPath};
/// let mut p = ClosedPath::rect_path(2, 2).unwrap();
/// assert_eq!(p.len(), 5);
///
/// let mut scaler = PathScaler::builder()
///     .x_factor(2)
///     .y_factor(3)
///     .build()
///     .unwrap();
///
/// scaler.mutate(&mut p);
///
/// assert_eq!(p.get(0).copied(), Some((0, 0).into()));
/// assert_eq!(p.get(1).copied(), Some((0, 3).into()));
/// assert_eq!(p.get(2).copied(), Some((2, 3).into()));
/// assert_eq!(p.get(3).copied(), Some((2, 0).into()));
/// assert_eq!(p.get(4).copied(), Some((0, 0).into()));
///
/// // we can do it again
/// scaler.mutate(&mut p);
///
/// assert_eq!(p.get(0).copied(), Some((0, 0).into()));
/// assert_eq!(p.get(1).copied(), Some((0, 9).into()));
/// assert_eq!(p.get(2).copied(), Some((4, 9).into()));
/// assert_eq!(p.get(3).copied(), Some((4, 0).into()));
/// assert_eq!(p.get(4).copied(), Some((0, 0).into()));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct PathScaler {
    #[builder(default = "1")]
    x_factor: i64,
    #[builder(default = "1")]
    y_factor: i64,
}

impl Default for PathScaler {
    fn default() -> Self {
        Self {
            x_factor: 1,
            y_factor: 1,
        }
    }
}

impl PathScalerBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(x_factor) = self.x_factor {
            if x_factor < 1 {
                return Err("x_factor cannot be less than 1.".into());
            }
        }

        if let Some(y_factor) = self.y_factor {
            if y_factor < 1 {
                return Err("y_factor cannot be less than 1.".into());
            }
        }

        Ok(())
    }
}

impl PathScaler {
    pub fn builder() -> PathScalerBuilder {
        PathScalerBuilder::default()
    }
}

impl PathMutator for PathScaler {
    fn mutate<P: super::PointPath>(&mut self, path: &mut P) -> bool {
        // break early in this special case
        if self.x_factor == 1 && self.y_factor == 1 {
            return false;
        }

        for p in path.points_mut() {
            p.x *= self.x_factor;
            p.y *= self.y_factor;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        let s = PathScaler::builder().build().unwrap();
        assert_eq!(s, PathScaler::default());

        let s = PathScaler::builder()
            .x_factor(2)
            .y_factor(3)
            .build()
            .unwrap();

        assert_eq!(
            s,
            PathScaler {
                x_factor: 2,
                y_factor: 3
            }
        );
    }
}
