use std::io::Write;

use anyhow::{bail, Result};
use clap::Parser;
use rand::thread_rng;

use crate::days::{
    Day, Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10, Day11, Day12, Day13,
    Day14, Day15, Day16, Day17, Day18, Day19, Day20, Day21, Day22, Day23, Day24, Day25,
};

/// Generate an unofficial input for a given day for advent of code 2023.
#[derive(Debug, Clone, Parser)]
#[command(author, version)]
pub struct Cli {
    /// A day from 1-25, inclusive.
    day: usize,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();

        let mut rng = thread_rng();

        let output = match cli.day {
            1 => Day01::generate(&mut rng)?,
            2 => Day02::generate(&mut rng)?,
            3 => Day03::generate(&mut rng)?,
            4 => Day04::generate(&mut rng)?,
            5 => Day05::generate(&mut rng)?,
            6 => Day06::generate(&mut rng)?,
            7 => Day07::generate(&mut rng)?,
            8 => Day08::generate(&mut rng)?,
            9 => Day09::generate(&mut rng)?,
            10 => Day10::generate(&mut rng)?,
            11 => Day11::generate(&mut rng)?,
            12 => Day12::generate(&mut rng)?,
            13 => Day13::generate(&mut rng)?,
            14 => Day14::generate(&mut rng)?,
            15 => Day15::generate(&mut rng)?,
            16 => Day16::generate(&mut rng)?,
            17 => Day17::generate(&mut rng)?,
            18 => Day18::generate(&mut rng)?,
            19 => Day19::generate(&mut rng)?,
            20 => Day20::generate(&mut rng)?,
            21 => Day21::generate(&mut rng)?,
            22 => Day22::generate(&mut rng)?,
            23 => Day23::generate(&mut rng)?,
            24 => Day24::generate(&mut rng)?,
            25 => Day25::generate(&mut rng)?,
            _ => bail!("Unsupported day: {}", cli.day),
        };

        // dumb way ensure newline at end of file
        if !output.ends_with('\n') {
            writeln!(std::io::stdout(), "{}", output)?;
        } else {
            std::io::stdout().write_all(output.as_bytes())?;
        }

        Ok(())
    }
}
