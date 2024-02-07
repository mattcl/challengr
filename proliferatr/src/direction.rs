//! This module contains several Enums representing different sets compass
//! directions.
//!
//! All of the enums can be cast to integer types that are bitmasks. The masks
//! for specific named directions are identical regardless of which Direction
//! type is being worked with (for consistency).
//!
//! The base 10 values of the masks are as follows:
//! ```text
//! North     = 1,
//! NorthEast = 2,
//! East      = 4,
//! SouthEast = 8,
//! South     = 16,
//! SouthWest = 32,
//! West      = 64,
//! NorthWest = 128,
//! ```
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
#[non_exhaustive]
pub enum DirectionError {
    #[error("Cannot make Direction from {0}")]
    DirectionParseError(String),

    #[error("Cannot make Cardinal from {0}")]
    CardinalParseError(String),

    #[error("Cannot make HorizHexDir from {0}")]
    HorizHexParseError(String),

    #[error("Cannot make VertHexDir from {0}")]
    VertHexParseError(String),

    #[error("Cannot make Relative from {0}")]
    RelativeParseError(String),
}

/// Driections is an enum of both the Cardinal and Ordinal directions.
///
/// It can be parsed from various string representations.
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use proliferatr::direction::Direction;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(Direction::from_str(v).unwrap(), Direction::North);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(Direction::from_str(v).unwrap(), Direction::NorthEast);
/// }
///
/// // etc..
/// ```
///
/// Additionally, this enum acts as the foundation for other direction enums
/// that are subsets of the combination of Cardinal and Ordinal directions. As
/// such, conversions exist from the subset types to this type.
///
/// i.e.:
///
/// ```
/// use proliferatr::direction::{Direction, HorizHexDir};
///
/// assert_eq!(Direction::NorthEast, HorizHexDir::NorthEast.into())
/// ```
///
/// Lastly, the direction enums can be cast as whatever integer type to form
/// bitmasks
///
/// ```
/// use proliferatr::direction::Direction;
///
/// let a = Direction::North as u8;
/// let b = Direction::East as u8;
/// assert_eq!(a | b, 0b101);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North = 1,
    NorthEast = 2,
    East = 4,
    SouthEast = 8,
    South = 16,
    SouthWest = 32,
    West = 64,
    NorthWest = 128,
}

impl Direction {
    /// Return the direction 180 degress opposite of ourself.
    ///
    /// Example:
    /// ```
    /// use proliferatr::direction::Direction;
    ///
    /// assert_eq!(Direction::North.opposite(), Direction::South);
    /// assert_eq!(Direction::South.opposite(), Direction::North);
    /// assert_eq!(Direction::East.opposite(), Direction::West);
    /// assert_eq!(Direction::West.opposite(), Direction::East);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::NorthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
            Self::NorthWest => Self::SouthEast,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "north" | "n" => Self::North,
            "northeast" | "ne" => Self::NorthEast,
            "east" | "e" => Self::East,
            "southeast" | "se" => Self::SouthEast,
            "south" | "s" => Self::South,
            "southwest" | "sw" => Self::SouthWest,
            "west" | "w" => Self::West,
            "northwest" | "nw" => Self::NorthWest,
            _ => return Err(DirectionError::DirectionParseError(s.to_string())),
        })
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::North => "North",
            Self::NorthEast => "NorthEast",
            Self::East => "East",
            Self::SouthEast => "SouthEast",
            Self::South => "South",
            Self::SouthWest => "SouthWest",
            Self::West => "West",
            Self::NorthWest => "NorthWest",
        };
        write!(f, "{}", out)
    }
}

impl From<Cardinal> for Direction {
    fn from(value: Cardinal) -> Self {
        Self::from(&value)
    }
}

impl From<&Cardinal> for Direction {
    fn from(value: &Cardinal) -> Self {
        match value {
            Cardinal::North => Self::North,
            Cardinal::South => Self::South,
            Cardinal::East => Self::East,
            Cardinal::West => Self::West,
        }
    }
}

impl From<HorizHexDir> for Direction {
    fn from(value: HorizHexDir) -> Self {
        Self::from(&value)
    }
}

impl From<&HorizHexDir> for Direction {
    fn from(value: &HorizHexDir) -> Self {
        match value {
            HorizHexDir::North => Self::North,
            HorizHexDir::NorthEast => Self::NorthEast,
            HorizHexDir::NorthWest => Self::NorthWest,
            HorizHexDir::South => Self::South,
            HorizHexDir::SouthEast => Self::SouthEast,
            HorizHexDir::SouthWest => Self::SouthWest,
        }
    }
}

impl From<VertHexDir> for Direction {
    fn from(value: VertHexDir) -> Self {
        Self::from(&value)
    }
}

impl From<&VertHexDir> for Direction {
    fn from(value: &VertHexDir) -> Self {
        match value {
            VertHexDir::East => Self::East,
            VertHexDir::NorthEast => Self::NorthEast,
            VertHexDir::SouthEast => Self::SouthEast,
            VertHexDir::West => Self::West,
            VertHexDir::NorthWest => Self::NorthWest,
            VertHexDir::SouthWest => Self::SouthWest,
        }
    }
}

/// Cardinal driections are North, South, East, and West. It can be pasrsed
/// much like the [Direction] enum.
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use proliferatr::direction::Cardinal;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::North);
/// }
/// ```
///
/// Additionally, because these directions can be represented by a single char,
/// the Cardinal enum can also be made from chars, irrespective of case.
///
/// Example:
/// ```
/// use std::convert::TryFrom;
/// use proliferatr::direction::Cardinal;
///
/// assert_eq!(Cardinal::try_from('n').unwrap(), Cardinal::North);
/// assert_eq!(Cardinal::try_from('N').unwrap(), Cardinal::North);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Cardinal {
    North = 1,
    East = 4,
    South = 16,
    West = 64,
}

impl Cardinal {
    /// Return the cardinal direction 90 degress to the right of ourself.
    ///
    /// Example:
    /// ```
    /// use proliferatr::direction::Cardinal;
    ///
    /// assert_eq!(Cardinal::North.right(), Cardinal::East);
    /// assert_eq!(Cardinal::South.right(), Cardinal::West);
    /// assert_eq!(Cardinal::East.right(), Cardinal::South);
    /// assert_eq!(Cardinal::West.right(), Cardinal::North);
    /// ```
    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    /// Return the cardinal direction 90 degress to the left of ourself.
    ///
    /// Example:
    /// ```
    /// use proliferatr::direction::Cardinal;
    ///
    /// assert_eq!(Cardinal::North.left(), Cardinal::West);
    /// assert_eq!(Cardinal::South.left(), Cardinal::East);
    /// assert_eq!(Cardinal::East.left(), Cardinal::North);
    /// assert_eq!(Cardinal::West.left(), Cardinal::South);
    /// ```
    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    /// Return the cardinal direction 180 degress opposite of ourself.
    ///
    /// Example:
    /// ```
    /// use proliferatr::direction::Cardinal;
    ///
    /// assert_eq!(Cardinal::North.opposite(), Cardinal::South);
    /// assert_eq!(Cardinal::South.opposite(), Cardinal::North);
    /// assert_eq!(Cardinal::East.opposite(), Cardinal::West);
    /// assert_eq!(Cardinal::West.opposite(), Cardinal::East);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

impl FromStr for Cardinal {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::North => Self::North,
            Direction::South => Self::South,
            Direction::East => Self::East,
            Direction::West => Self::West,
            _ => return Err(DirectionError::CardinalParseError(s.to_string())),
        })
    }
}

impl TryFrom<char> for Cardinal {
    type Error = DirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'n' => Ok(Self::North),
            's' => Ok(Self::South),
            'e' => Ok(Self::East),
            'w' => Ok(Self::West),
            _ => Err(DirectionError::CardinalParseError(value.to_string())),
        }
    }
}

/// HorizHexDir is an enum of compass directions that represent valid faces of a
/// hexagon with flat edges north and south.
///
/// See the following diagram:
/// ```text
///        n
///      +---+
/// nw  /     \  ne
///    +       +
/// sw  \     /  se
///      +---+
///        s
/// ```
/// It can be parsed from the standard set of direction strings
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use proliferatr::direction::HorizHexDir;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(HorizHexDir::from_str(v).unwrap(), HorizHexDir::North);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(HorizHexDir::from_str(v).unwrap(), HorizHexDir::NorthEast);
/// }
///
/// // etc..
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum HorizHexDir {
    North = 1,
    NorthEast = 2,
    NorthWest = 32,
    South = 16,
    SouthEast = 8,
    SouthWest = 128,
}

impl FromStr for HorizHexDir {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::North => Self::North,
            Direction::NorthEast => Self::NorthEast,
            Direction::NorthWest => Self::NorthWest,
            Direction::South => Self::South,
            Direction::SouthEast => Self::SouthEast,
            Direction::SouthWest => Self::SouthWest,
            _ => return Err(DirectionError::HorizHexParseError(s.to_string())),
        })
    }
}

impl fmt::Display for HorizHexDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

/// VertHexDir is an enum of compass directions that represent valid faces of a
/// hexagon with flat edges west and east.
///
/// See the following diagram:
/// ```text
///       +
///      / \
/// nw  /   \  ne
///    /     \
///   +       +
///   |       |
/// w |       | e
///   |       |
///   +       +
///    \     /
/// sw  \   /  se
///      \ /
///       +
/// ```
/// It can be parsed from the standard set of direction strings
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use proliferatr::direction::VertHexDir;
///
/// for v in ["East", "east", "E", "e"] {
///     assert_eq!(VertHexDir::from_str(v).unwrap(), VertHexDir::East);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(VertHexDir::from_str(v).unwrap(), VertHexDir::NorthEast);
/// }
///
/// // etc..
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum VertHexDir {
    East = 4,
    NorthEast = 2,
    SouthEast = 8,
    West = 64,
    NorthWest = 128,
    SouthWest = 32,
}

impl FromStr for VertHexDir {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::East => Self::East,
            Direction::NorthEast => Self::NorthEast,
            Direction::SouthEast => Self::SouthEast,
            Direction::West => Self::West,
            Direction::NorthWest => Self::NorthWest,
            Direction::SouthWest => Self::SouthWest,
            _ => return Err(DirectionError::VertHexParseError(s.to_string())),
        })
    }
}

impl fmt::Display for VertHexDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

/// Relative directions are directions like 'left' and 'right'.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Relative {
    Left,
    Right,
    Up,
    Down,
}

impl Relative {
    /// Returns the relative direction opposite to `self`.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl Display for Relative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Up => "Up",
            Self::Down => "Down",
        };

        s.fmt(f)
    }
}

impl TryFrom<char> for Relative {
    type Error = DirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' | 'l' => Ok(Self::Left),
            'R' | 'r' => Ok(Self::Right),
            'U' | 'u' => Ok(Self::Up),
            'D' | 'd' => Ok(Self::Down),
            _ => Err(DirectionError::RelativeParseError(value.to_string())),
        }
    }
}

impl FromStr for Relative {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" | "left" | "L" | "l" => Ok(Self::Left),
            "Right" | "right" | "R" | "r" => Ok(Self::Right),
            "Up" | "up" | "U" | "u" => Ok(Self::Up),
            "Down" | "down" | "D" | "d" => Ok(Self::Down),
            _ => Err(DirectionError::RelativeParseError(s.into())),
        }
    }
}

impl From<Cardinal> for Relative {
    fn from(value: Cardinal) -> Self {
        match value {
            Cardinal::North => Self::Up,
            Cardinal::East => Self::Right,
            Cardinal::South => Self::Down,
            Cardinal::West => Self::Left,
        }
    }
}

impl From<Relative> for Cardinal {
    fn from(value: Relative) -> Self {
        match value {
            Relative::Up => Self::North,
            Relative::Right => Self::East,
            Relative::Down => Self::South,
            Relative::Left => Self::West,
        }
    }
}

impl From<Relative> for Direction {
    fn from(value: Relative) -> Self {
        match value {
            Relative::Up => Self::North,
            Relative::Right => Self::East,
            Relative::Down => Self::South,
            Relative::Left => Self::West,
        }
    }
}

/// Indicates that this type has cardinal neighbors
pub trait CardinalNeighbors: Sized {
    /// Get a thing north of us.
    fn north(&self) -> Self;

    /// Get a thing south of us.
    fn south(&self) -> Self;

    /// Get a thing east of us.
    fn east(&self) -> Self;

    /// Get a thing west of us.
    fn west(&self) -> Self;

    /// Get a thing in `Cardinal` dir relative to us.
    fn cardinal_neighbor(&self, dir: Cardinal) -> Self {
        match dir {
            Cardinal::North => <Self as CardinalNeighbors>::north(self),
            Cardinal::East => <Self as CardinalNeighbors>::east(self),
            Cardinal::South => <Self as CardinalNeighbors>::south(self),
            Cardinal::West => <Self as CardinalNeighbors>::west(self),
        }
    }
}

/// Indicates that this type has ordinal neighbors
pub trait OrdinalNeighbors: Sized {
    /// Get a thing north east of us.
    fn north_east(&self) -> Self;

    /// Get a thing north west of us.
    fn north_west(&self) -> Self;

    /// Get a thing south east of us.
    fn south_east(&self) -> Self;

    /// Get a thing south west of us.
    fn south_west(&self) -> Self;
}

/// Indicates that this type has cardinal neighbors but some do not exist
pub trait BoundedCardinalNeighbors: Sized {
    /// Get a thing north of us.
    fn north(&self) -> Option<Self>;

    /// Get a thing south of us.
    fn south(&self) -> Option<Self>;

    /// Get a thing east of us.
    fn east(&self) -> Option<Self>;

    /// Get a thing west of us.
    fn west(&self) -> Option<Self>;

    /// Get a thing in `Cardinal` dir relative to us.
    fn cardinal_neighbor(&self, dir: Cardinal) -> Option<Self> {
        match dir {
            Cardinal::North => <Self as BoundedCardinalNeighbors>::north(self),
            Cardinal::East => <Self as BoundedCardinalNeighbors>::east(self),
            Cardinal::South => <Self as BoundedCardinalNeighbors>::south(self),
            Cardinal::West => <Self as BoundedCardinalNeighbors>::west(self),
        }
    }
}

/// Indicates that this type has ordinal neighbors, but some do not exist
pub trait BoundedOrdinalNeighbors: Sized {
    /// Get a thing north east of us.
    fn north_east(&self) -> Option<Self>;

    /// Get a thing north west of us.
    fn north_west(&self) -> Option<Self>;

    /// Get a thing south east of us.
    fn south_east(&self) -> Option<Self>;

    /// Get a thing south west of us.
    fn south_west(&self) -> Option<Self>;
}

#[cfg(test)]
mod tests {
    mod cardinal {
        use super::super::*;

        #[test]
        fn parsing() {
            for v in ["North", "north", "N", "n"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::North);
            }

            for v in ["South", "south", "S", "s"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::South);
            }

            for v in ["East", "east", "E", "e"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::East);
            }

            for v in ["West", "west", "W", "w"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::West);
            }
        }
    }

    mod relative {
        use super::super::*;

        #[test]
        fn parsing() {
            for v in ["Right", "right", "R", "r"] {
                assert_eq!(Relative::from_str(v).unwrap(), Relative::Right);
            }

            for v in ["Left", "left", "L", "l"] {
                assert_eq!(Relative::from_str(v).unwrap(), Relative::Left);
            }

            for v in ["Up", "up", "U", "u"] {
                assert_eq!(Relative::from_str(v).unwrap(), Relative::Up);
            }

            for v in ["Down", "down", "D", "d"] {
                assert_eq!(Relative::from_str(v).unwrap(), Relative::Down);
            }
        }
    }
}
