use itertools::Itertools;
use proliferatr::{
    bound::Bound2D,
    direction::Cardinal,
    path::{ClosedPath, PathMutator, PointPath, UnitSegmentAdder},
    point::Point,
    InputGenerator,
};
use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};

use super::Day;

const DIMENSION: usize = 140;
const CENTER_EXCLUSION: i64 = 20;
const EXCLUSION_POINTS: usize = 200;
const STARTING_SQUARE_SIDE: usize = 60;
const NUM_ALTERATION_PASSES: usize = 100;
const CENTER: Point = Point { x: 70, y: 70 };
const INITIAL_OFFSET: Point = Point { x: 40, y: 40 };
const BOUNDS: Bound2D = Bound2D {
    min_x: 0,
    max_x: 139,
    min_y: 0,
    max_y: 139,
};
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
    type GeneratorError = anyhow::Error;
    type Output = Vec<Vec<char>>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let mut grid = vec![vec!['.'; DIMENSION]; DIMENSION];

        // create the initial square path and translate it to the center of the
        // grid
        let mut path = ClosedPath::rect_path(STARTING_SQUARE_SIDE, STARTING_SQUARE_SIDE)?;
        path.translate(INITIAL_OFFSET);

        let mut segment_adder = UnitSegmentAdder::builder()
            .rng(rng.clone())
            .passes(NUM_ALTERATION_PASSES)
            .bounds(BOUNDS)
            .build()?;

        // we're going to add some noise inside the exclusion zone so that there
        // is a region of cells contained by the path in the center of the grid
        let ex_dist = Uniform::from(-CENTER_EXCLUSION..CENTER_EXCLUSION);
        for _ in 0..EXCLUSION_POINTS {
            let mut p = Point::new(ex_dist.sample(rng), ex_dist.sample(rng));
            p += CENTER;
            segment_adder.insert_avoided(p);
        }

        // alter our starting path by addiing random segments
        segment_adder.mutate(&mut path);

        // pick a random spot for the S
        let s_idx = rng.gen_range(0..path.len());

        let mut points: Vec<_> = path.points().copied().collect();

        // we're going to extend the closed path, which already has the first
        // point duplicated onto the end, to have the first _and_ second points
        // duplicated.
        points.push(points[1]);

        for (p1, p2, p3) in points.iter().tuple_windows() {
            // these unwraps should be safe because the points should be
            // different
            let d1 = p1.cardinal_to(p2).unwrap();
            let d2 = p2.cardinal_to(p3).unwrap();

            let ch = match (d1, d2) {
                (Cardinal::East, Cardinal::East) | (Cardinal::West, Cardinal::West) => '-',
                (Cardinal::North, Cardinal::North) | (Cardinal::South, Cardinal::South) => '|',
                (Cardinal::East, Cardinal::South) | (Cardinal::North, Cardinal::West) => '7',
                (Cardinal::East, Cardinal::North) | (Cardinal::South, Cardinal::West) => 'J',
                (Cardinal::West, Cardinal::South) | (Cardinal::North, Cardinal::East) => 'F',
                (Cardinal::West, Cardinal::North) | (Cardinal::South, Cardinal::East) => 'L',
                _ => unreachable!("Unexpected combo ({:?}, {:?})", d1, d2),
            };

            grid[DIMENSION - 1 - p2.y as usize][p2.x as usize] = ch;
        }

        let s = points[s_idx];
        grid[DIMENSION - 1 - s.y as usize][s.x as usize] = 'S';

        // we now want to randomly fill the other characters to disguise the path
        #[allow(clippy::needless_range_loop)]
        for r in 0..DIMENSION {
            for c in 0..DIMENSION {
                let p = Point {
                    x: c as i64,
                    y: r as i64,
                };

                // don't accidentally create a path leading into the S
                if p.manhattan_distance(&s) < 3 {
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
