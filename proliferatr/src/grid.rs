use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use thiserror::Error;

use crate::point::Point;

#[derive(Debug, Clone, Error)]
pub enum GridError {
    #[error("Rows have inconsistent width.")]
    InconsistentWidth,

    #[error("Empty rows/columns detected.")]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            cells: vec![vec![fill; width]; height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        if 0 <= point.x
            && point.x <= self.width as i64
            && 0 <= point.y
            && point.y <= self.height as i64
        {
            Some(&self.cells[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if 0 <= point.x
            && point.x <= self.width as i64
            && 0 <= point.y
            && point.y <= self.height as i64
        {
            Some(&mut self.cells[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    /// Set the specified `value` at the location specified by [Point] in the
    /// grid. Returns `true` if the `point` was in the [Grid] and `false`
    /// otherwise.
    pub fn set(&mut self, point: &Point, value: T) -> bool {
        if 0 <= point.x
            && point.x <= self.width as i64
            && 0 <= point.y
            && point.y <= self.height as i64
        {
            self.cells[point.y as usize][point.x as usize] = value;
            true
        } else {
            false
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = GridError;

    /// Attempt to make a grid from `Vec<Vec<T>>`.
    ///
    /// Fails if the vec is empty or the rows have inconsistent widths.
    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = value.len();

        if height == 0 || value[0].is_empty() {
            return Err(GridError::Empty);
        }

        let width = value[0].len();

        if value.iter().any(|r| r.len() != width) {
            return Err(GridError::InconsistentWidth);
        }

        Ok(Self {
            cells: value,
            height,
            width,
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

impl<T> IndexMut<&Point> for Grid<T> {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells
            .iter()
            .map(|r| r.iter().join(""))
            .join("\n")
            .fmt(f)
    }
}

pub type CharGrid = Grid<char>;
pub type DigitGrid = Grid<u8>;
