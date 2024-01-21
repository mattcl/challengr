use std::{collections::VecDeque, convert::Infallible};

use itertools::Itertools;
use proliferatr::{
    maze::{Direction, Grid, Location},
    InputGenerator,
};
use rand::{seq::SliceRandom, Rng};

use super::Day;

const DIMENSION: usize = 70;
// this doesn't divide the grid evenly, but it's good enough.
const TILE_WIDTH: usize = 11;
const NUM_ALTERATION_CYCLES: usize = 30;
const ALTERATIONS: &[Alt] = &[Alt::Nothing, Alt::Expand, Alt::Contract];

/// Divide the grid into 11x11 tiles, randomly selecting a point in each tile
/// to be the location of a junction. The upper right and lower left corners
/// will have a "junction" with only two neighbors, which will turn them into
/// non-junctions in the final grid.
///
/// For each junction, construct a shortest path to each neighbor for which a
/// joining path does not already exist.
///
/// Alter each path repeatedly within the confines of the grid (like we do for
/// the closed paths for days 10 and 18).
///
/// For part 1, we need to place arrows preventing travel back "up" the graph,
/// allowing movement only from one junction toward a junction that is "lower"
/// than it. This can be done by just putting arrows pointing at the lower right
/// corner.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day23;

impl Day for Day23 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Day23.gen_input(rng)
    }
}

impl InputGenerator for Day23 {
    type GeneratorError = Infallible;
    type Output = String;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut grid = Grid::new(DIMENSION, DIMENSION);

        let (mut paths, junctions, mut occupied) = 'outer: loop {
            let mut occupied = vec![vec![false; DIMENSION]; DIMENSION];
            let mut junctions = vec![vec![Location::default(); 6]; 6];

            // we can probably only include the south and east nodes.
            #[allow(clippy::needless_range_loop)]
            for tile_r in 0..6 {
                let max_r = TILE_WIDTH * (tile_r + 1) - 4;
                let min_r = TILE_WIDTH * tile_r + 4;

                for tile_c in 0..6 {
                    let max_c = TILE_WIDTH * (tile_c + 1) - 4;
                    let min_c = TILE_WIDTH * tile_c + 4;

                    let loc: Location =
                        (rng.gen_range(min_r..max_r), rng.gen_range(min_c..max_c)).into();
                    junctions[tile_r][tile_c] = loc;
                }
            }

            let mut paths = Vec::new();
            if let Some(p) = initial_path(
                &Location::default(),
                &junctions[0][0],
                Direction::East,
                &mut occupied,
            ) {
                paths.push(p);
            } else {
                continue 'outer;
            }

            for row in 0..6 {
                // row 0
                for col in 0..6 {
                    if col < 5 {
                        if let Some(p) = initial_path(
                            &junctions[row][col],
                            &junctions[row][col + 1],
                            Direction::East,
                            &mut occupied,
                        ) {
                            paths.push(p);
                        } else {
                            continue 'outer;
                        }
                    }

                    if row < 5 {
                        if let Some(p) = initial_path(
                            &junctions[row][col],
                            &junctions[row + 1][col],
                            Direction::South,
                            &mut occupied,
                        ) {
                            paths.push(p);
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }

            // path to the last exit
            if let Some(p) = initial_path(
                &junctions[5][5],
                &Location {
                    row: DIMENSION - 1,
                    col: DIMENSION - 1,
                },
                Direction::South,
                &mut occupied,
            ) {
                paths.push(p);
            } else {
                continue 'outer;
            }

            break (paths, junctions, occupied);
        };

        // mutate the paths
        for _ in 0..NUM_ALTERATION_CYCLES {
            for p in paths.iter_mut() {
                p.alter(rng, &mut occupied);
            }
        }

        // translate the paths into the grid
        for path in paths.iter() {
            for (p1, p2) in path.locations.iter().tuple_windows() {
                let d = p1.dir_to(p2);
                let v1 = grid.get(p1).unwrap();
                grid.set(p1, v1 | d as u8);

                let v2 = grid.get(p2).unwrap();
                grid.set(p2, v2 | d.opposite() as u8);
            }
        }

        let mut chars = grid.char_representation();
        // place arrows at the junctions
        #[allow(clippy::needless_range_loop)]
        for r in 0..6 {
            for c in 0..6 {
                let row = junctions[r][c].row * 2 + 1;
                let col = junctions[r][c].col * 2 + 1;

                // these are the "empty" corners
                if (r == 0 && c == 5) || (r == 5 && c == 0) {
                    continue;
                }

                if r > 0 {
                    chars[row - 1][col] = 'v';
                }

                if r < 5 {
                    chars[row + 1][col] = 'v';
                }

                if c > 0 {
                    chars[row][col - 1] = '>';
                }

                if c < 5 {
                    chars[row][col + 1] = '>';
                }
            }
        }

        Ok(chars
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n"))
    }
}

// because of how we pick junctions, it should be unlikely that lines will
// cross, but, if they do, we'll just take the lazy way out and regen the whole
// junctions
fn initial_path(
    start: &Location,
    end: &Location,
    dir: Direction,
    occupied: &mut [Vec<bool>],
) -> Option<PointPath> {
    let mut path = VecDeque::default();
    path.push_front(*start);
    occupied[start.row][start.col] = true;

    // we know because of our bounds for the junction locations, that these
    // operations are safe
    let (mut cur, end_target) = match dir {
        Direction::East => (
            Location {
                row: start.row,
                col: start.col + 1,
            },
            Location {
                row: end.row,
                col: end.col - 1,
            },
        ),
        Direction::South => (
            Location {
                row: start.row + 1,
                col: start.col,
            },
            Location {
                row: end.row - 1,
                col: end.col,
            },
        ),
        _ => unreachable!(
            "Should not have started initial path in direction other than east or south"
        ),
    };

    occupied[cur.row][cur.col] = true;

    path.push_back(cur);

    match dir {
        Direction::East => {
            // move two units east
            for _ in 0..2 {
                cur.col += 1;
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }

            // align the rows
            while cur.row != end_target.row {
                if cur.row < end_target.row {
                    cur.row += 1;
                } else {
                    cur.row -= 1;
                }
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }

            // now move east until we hit the target
            while cur.col != end_target.col {
                cur.col += 1;
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }
        }
        Direction::South => {
            // move two units south
            for _ in 0..2 {
                cur.row += 1;
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }

            // align the cols
            while cur.col != end_target.col {
                if cur.col < end_target.col {
                    cur.col += 1;
                } else {
                    cur.col -= 1;
                }
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }

            // now move south until we hit the target
            while cur.row != end_target.row {
                cur.row += 1;
                path.push_back(cur);
                if occupied[cur.row][cur.col] {
                    return None;
                }
                occupied[cur.row][cur.col] = true;
            }
        }
        _ => unreachable!(
            "Should not have started initial path in direction other than east or south"
        ),
    }

    path.push_back(*end);
    Some(PointPath { locations: path })
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PointPath {
    locations: VecDeque<Location>,
}

impl PointPath {
    pub fn alter<R: Rng + Clone + ?Sized>(&mut self, rng: &mut R, occupied: &mut [Vec<bool>]) {
        for i in (1..(self.locations.len() - 2)).rev() {
            let mut p1 = self.locations[i];
            let mut p2 = self.locations[i + 1];

            let orient = if p1.row == p2.row {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            };

            let alteration = ALTERATIONS.choose(rng).unwrap();
            match alteration {
                Alt::Expand => match orient {
                    Orientation::Vertical if p1.col > 1 => {
                        p1.col -= 1;
                        p2.col -= 1;
                    }
                    Orientation::Horizontal if p1.row > 1 => {
                        p1.row -= 1;
                        p2.row -= 1;
                    }
                    _ => continue,
                },
                Alt::Contract => match orient {
                    Orientation::Vertical if p1.col < DIMENSION - 1 => {
                        p1.col += 1;
                        p2.col += 1;
                    }
                    Orientation::Horizontal if p1.row < DIMENSION - 1 => {
                        p1.row += 1;
                        p2.row += 1;
                    }
                    _ => continue,
                },
                Alt::Nothing => continue,
            }

            if !occupied[p1.row][p1.col] && !occupied[p2.row][p2.col] {
                occupied[p1.row][p1.col] = true;
                occupied[p2.row][p2.col] = true;
                self.locations.insert(i + 1, p2);
                self.locations.insert(i + 1, p1);
            }
        }
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
