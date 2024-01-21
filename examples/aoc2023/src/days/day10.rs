use std::{collections::VecDeque, convert::Infallible};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

const DIMENSION: usize = 140;
const CENTER: usize = DIMENSION / 2;
const CENTER_EXCLUSION: usize = 20;
const EXCLUSION_POINTS: usize = 30;
const ABSOLUTE_EXCLUSION: usize = 10;
const STARTING_SQUARE_OFFSET: usize = 20;
const NUM_ALTERATIONS: usize = 100;
const ALTERATIONS: &[Alt] = &[Alt::Nothing, Alt::Expand, Alt::Contract];
// the extra '.' is intentional
const FILLER_CHARS: &[u8] = b"7F|-JL..";

/// The strategy is to generate a rectangle of points spaced 1 unit away from
/// each other, centered in the grid. We will then iterate through each pair of
/// points, epanding or contracting the shape by adding segment that lies
/// outside or inside or the shape, joining that to the polgon by inserting the
/// endpoints of the segment between the pair of points we're looking at.
///
/// We do this N times then fill in the rest of the grid with nonsense. We can
/// guarantee that we leave a gap in the center where we are guaranteed to
/// contain cells, as well as creating "islands" in the initial rectangle that
/// we won't be able to fill in.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day10;

impl Day for Day10 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day10
            .gen_input(rng)?
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n"))
    }
}

impl InputGenerator for Day10 {
    type GeneratorError = Infallible;
    type Output = Vec<Vec<char>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut occupied = vec![vec![false; DIMENSION]; DIMENSION];
        let mut grid = vec![vec!['.'; DIMENSION]; DIMENSION];

        // VecDeque for better insert behavior
        let mut points: VecDeque<Point> = VecDeque::with_capacity(1000);

        // make a square
        let mut cur = Point {
            row: CENTER - STARTING_SQUARE_OFFSET,
            col: CENTER - STARTING_SQUARE_OFFSET,
        };
        points.push_back(cur);
        // top
        for c in (cur.col + 1)..(cur.col + STARTING_SQUARE_OFFSET * 2) {
            cur.col = c;
            points.push_back(cur);
        }
        // right
        for r in (cur.row + 1)..(cur.row + STARTING_SQUARE_OFFSET * 2) {
            cur.row = r;
            points.push_back(cur);
        }
        // bot
        for _ in 0..(STARTING_SQUARE_OFFSET * 2 - 1) {
            cur.col -= 1;
            points.push_back(cur);
        }
        // left
        for _ in 1..(STARTING_SQUARE_OFFSET * 2 - 1) {
            cur.row -= 1;
            points.push_back(cur);
        }

        cur.row -= 1;

        // sanity check we got back to the same spot as we started
        assert_eq!(cur, points[0]);

        let center = Point {
            row: CENTER,
            col: CENTER,
        };

        for p in points.iter() {
            occupied[p.row][p.col] = true;
        }

        // we're going to insert some random occupied locations into the
        // exclusion zone to disrupt the shape in the center
        for _ in 0..EXCLUSION_POINTS {
            let r = rng.gen_range((CENTER - CENTER_EXCLUSION)..(CENTER + CENTER_EXCLUSION));
            let c = rng.gen_range((CENTER - CENTER_EXCLUSION)..(CENTER + CENTER_EXCLUSION));

            occupied[r][c] = true;
            occupied[r + 1][c + 1] = true;
        }

        for _ in 0..NUM_ALTERATIONS {
            for i in (0..(points.len() - 1)).rev() {
                // we know `i` and `i + 1` exist
                let mut p1 = points[i];
                let mut p2 = points[i + 1];

                let orient = if p1.row == p2.row {
                    Orientation::Horizontal
                } else {
                    Orientation::Vertical
                };

                let alteration = ALTERATIONS.choose(rng).unwrap();
                match alteration {
                    Alt::Expand => match orient {
                        Orientation::Vertical => {
                            if p1.col < CENTER && p1.col > 0 {
                                p1.col -= 1;
                                p2.col -= 1;
                            } else if p1.col >= CENTER && p1.col < DIMENSION - 1 {
                                p1.col += 1;
                                p2.col += 1;
                            }
                        }
                        Orientation::Horizontal => {
                            if p1.row < CENTER && p1.row > 0 {
                                p1.row -= 1;
                                p2.row -= 1;
                            } else if p1.row >= CENTER && p1.row < DIMENSION - 1 {
                                p1.row += 1;
                                p2.row += 1;
                            }
                        }
                    },
                    Alt::Contract => {
                        match orient {
                            Orientation::Vertical => {
                                if p1.col < CENTER {
                                    p1.col += 1;
                                    p2.col += 1;
                                } else {
                                    p1.col -= 1;
                                    p2.col -= 1;
                                }
                            }
                            Orientation::Horizontal => {
                                if p1.row < CENTER {
                                    p1.row += 1;
                                    p2.row += 1;
                                } else {
                                    p1.row -= 1;
                                    p2.row -= 1;
                                }
                            }
                        }

                        if p1.dist(&center) < ABSOLUTE_EXCLUSION
                            || p2.dist(&center) < ABSOLUTE_EXCLUSION
                        {
                            continue;
                        }
                    }
                    Alt::Nothing => continue,
                }

                if !occupied[p1.row][p1.col] && !occupied[p2.row][p2.col] {
                    occupied[p1.row][p1.col] = true;
                    occupied[p2.row][p2.col] = true;
                    points.insert(i + 1, p2);
                    points.insert(i + 1, p1);
                }
            }
        }

        // pick a random spot for the S
        let s_idx = rng.gen_range(0..points.len());

        // duplicate the first and second elemnts onto the end of the list so we
        // "wrap" around.
        points.push_back(points[0]);
        points.push_back(points[1]);

        for (p1, p2, p3) in points.iter().tuple_windows() {
            let d1 = p1.dir_to(p2);
            let d2 = p2.dir_to(p3);

            let ch = match (d1, d2) {
                (Direction::East, Direction::East) | (Direction::West, Direction::West) => '-',
                (Direction::North, Direction::North) | (Direction::South, Direction::South) => '|',
                (Direction::East, Direction::South) | (Direction::North, Direction::West) => '7',
                (Direction::East, Direction::North) | (Direction::South, Direction::West) => 'J',
                (Direction::West, Direction::South) | (Direction::North, Direction::East) => 'F',
                (Direction::West, Direction::North) | (Direction::South, Direction::East) => 'L',
                _ => unreachable!("Unexpected combo ({:?}, {:?})", d1, d2),
            };

            grid[p2.row][p2.col] = ch;
        }

        let s = points[s_idx];
        grid[s.row][s.col] = 'S';

        // we now want to randomly fill the other characters to disguise the path
        #[allow(clippy::needless_range_loop)]
        for r in 0..DIMENSION {
            for c in 0..DIMENSION {
                let p = Point { row: r, col: c };

                // don't accidentally create a path leading into the S
                if p.dist(&s) < 3 {
                    continue;
                }

                if grid[r][c] == '.' {
                    grid[r][c] = *FILLER_CHARS.choose(rng).unwrap() as char;
                }
            }
        }

        Ok(grid)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn dir_to(&self, other: &Self) -> Direction {
        if self.row == other.row {
            if self.col < other.col {
                Direction::East
            } else {
                Direction::West
            }
        } else if self.col == other.col {
            if self.row < other.row {
                Direction::South
            } else {
                Direction::North
            }
        } else {
            unreachable!("Attempted to get dir for points that are not cardinal neighbors")
        }
    }

    pub fn dist(&self, other: &Self) -> usize {
        self.row.max(other.row) - self.row.min(other.row) + self.col.max(other.col)
            - self.col.min(other.col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Alt {
    Nothing,
    Contract,
    Expand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
