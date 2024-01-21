use proliferatr::InputGenerator;
use rand::Rng;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day07::Day07;
pub use day08::Day08;
pub use day09::Day09;
pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;
pub use day23::Day23;
pub use day24::Day24;
pub use day25::Day25;

pub trait Day: Default + InputGenerator {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError>;
}
