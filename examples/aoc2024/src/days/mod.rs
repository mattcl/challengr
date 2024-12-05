use proliferatr::InputGenerator;
use rand::Rng;

mod day01;
mod day02;
mod day03;
mod day04;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;

pub trait Day: Default + InputGenerator {
    fn generate<R: Rng + Clone>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError>;
}
