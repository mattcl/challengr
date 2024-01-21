use std::fmt::Display;

use itertools::Itertools;
use rand::{seq::IteratorRandom, Rng};

const LOC_CARD_NEIGHBOR_OFFSETS: [(Direction, i64, i64); 4] = [
    (Direction::North, -1, 0),
    (Direction::East, 0, 1),
    (Direction::South, 1, 0),
    (Direction::West, 0, -1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North = 1,
    South = 2,
    East = 4,
    West = 8,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

impl Location {
    pub fn cardinal_neighbors(&self) -> impl Iterator<Item = (Direction, Location)> {
        let row = self.row;
        let col = self.col;

        LOC_CARD_NEIGHBOR_OFFSETS
            .iter()
            .filter_map(move |(dir, dr, dc)| {
                if *dr < 0 && row == 0 {
                    return None;
                }

                if *dc < 0 && col == 0 {
                    return None;
                }

                Some((
                    *dir,
                    ((row as i64 + *dr) as usize, (col as i64 + *dc) as usize).into(),
                ))
            })
    }

    pub fn dir_to(&self, other: &Self) -> Direction {
        if self.row == other.row {
            if self.col < other.col {
                Direction::East
            } else {
                Direction::West
            }
        } else {
            #[allow(clippy::collapsible_if)]
            if self.row < other.row {
                Direction::South
            } else {
                Direction::North
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid {
    pub cells: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![0; width]; height],
            width,
            height,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, loc: &Location) -> bool {
        loc.row < self.height && loc.col < self.width
    }

    pub fn get(&self, loc: &Location) -> Option<u8> {
        if self.contains(loc) {
            Some(self.cells[loc.row][loc.col])
        } else {
            None
        }
    }

    pub fn set(&mut self, loc: &Location, value: u8) -> bool {
        if self.contains(loc) {
            self.cells[loc.row][loc.col] = value;
            true
        } else {
            false
        }
    }

    pub fn random_cell<R: Rng + Clone + ?Sized>(&self, rng: &mut R) -> Location {
        Location {
            row: rng.gen_range(0..self.height),
            col: rng.gen_range(0..self.width),
        }
    }

    pub fn neighbors(
        &self,
        loc: &Location,
    ) -> impl Iterator<Item = (Direction, Location, u8)> + '_ {
        loc.cardinal_neighbors()
            .filter(|(_, l)| self.contains(l))
            .map(|(d, l)| (d, l, self.cells[l.row][l.col]))
    }

    pub fn char_representation(&self) -> Vec<Vec<char>> {
        // the top and left edge isn't included
        let rendered_height = self.height * 2 + 1;
        let rendered_width = self.width * 2 + 1;
        let mut output = vec![vec!['#'; self.width * 2 + 1]; self.height * 2 + 1];

        // TODO: make configurable
        // corner goals
        output[0][1] = '.';
        output[rendered_height - 1][rendered_width - 2] = '.';

        for r in 0..self.height {
            for c in 0..self.width {
                let v = self.cells[r][c];
                if v == 0 {
                    continue;
                }

                output[r * 2 + 1][c * 2 + 1] = '.';

                if v & Direction::East as u8 != 0 {
                    output[r * 2 + 1][c * 2 + 2] = '.';
                }

                if v & Direction::South as u8 != 0 {
                    output[r * 2 + 2][c * 2 + 1] = '.';
                }
            }
        }

        output
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.char_representation()
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n")
            .fmt(f)
    }
}

// slow, but just to test out the grid rendering
pub fn aldos_broder<R: Rng + Clone + ?Sized>(rng: &mut R, grid: &mut Grid) {
    let mut unvisited = grid.size() - 1;
    let mut cell = grid.random_cell(rng);
    let mut cell_value = grid.get(&cell).unwrap();

    while unvisited > 0 {
        if let Some((dir, loc, mut v)) = grid.neighbors(&cell).choose(rng) {
            if v == 0 {
                v |= dir.opposite() as u8;
                grid.set(&loc, v);
                grid.set(&cell, cell_value | dir as u8);
                unvisited -= 1;
            }

            cell = loc;
            cell_value = v;
        }
    }
}
