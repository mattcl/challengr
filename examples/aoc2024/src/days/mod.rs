use proliferatr::InputGenerator;
use rand::Rng;

mod day01;

pub use day01::Day01;

pub trait Day: Default + InputGenerator {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as InputGenerator>::GeneratorError>;
}
