use std::{fmt::Display, ops::Range};

use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

const VALUE_RANGE: Range<i64> = 1..1000;
const SEPARATORS: &[u8] = b"~![]{}()<>:@#$%^&*'?;+-/, ";
const NUM_SEPARATORS: Range<usize> = 0..5;
const LEFT_DELIMITERS: &[u8] = b"([{<";
const RIGHT_DELIMITERS: &[u8] = b")]}>";
const NUM_LINES: usize = 6;
const LINE_LENGTH: Range<usize> = 80..101;

/// If we look at the real inputs, we notice that it is very _not_ random in the
/// sense that the other tokens are well-defined. We're going to mimic the real
/// inputs here
#[derive(Debug, Default, Clone, Copy)]
pub struct Day03;

impl Day for Day03 {
    fn generate<R: Rng + Clone>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day03 {}.gen_input(rng)?.join("\n"))
    }
}

impl InputGenerator for Day03 {
    type GeneratorError = anyhow::Error;
    type Output = Vec<String>;

    fn gen_input<R: Rng + Clone>(&self, rng: &mut R) -> Result<Self::Output, Self::GeneratorError> {
        let mut out = Vec::default();

        while out.len() < NUM_LINES {
            let len = rng.gen_range(LINE_LENGTH);
            let mut line = Vec::with_capacity(len * 2);
            let mut num_do = 0;
            let mut num_dont = 0;
            for _ in 0..len {
                let token = Token::new(rng);

                match token {
                    Token::Do => num_do += 1,
                    Token::Dont => num_dont += 1,
                    _ => {}
                }

                line.push(token.to_string());
                let num_sep = rng.gen_range(NUM_SEPARATORS);
                if num_sep > 0 {
                    let mut sep = String::new();
                    for _ in 0..num_sep {
                        sep.push(SEPARATORS.choose(rng).copied().unwrap().into());
                    }
                    line.push(sep);
                }
            }

            if num_do == 0 && num_dont == 0 {
                continue;
            }

            out.push(line.join(""));
        }

        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    Mul {
        left: i64,
        right: i64,
    },
    WrongDelim {
        left: i64,
        right: i64,
        left_delim: char,
        right_delim: char,
    },
    OnlyOneValue {
        val: i64,
    },
    Do,
    Dont,
    Who,
    What,
    When,
    Where,
    Why,
    Select,
}

impl Token {
    pub fn new<R: Rng + Clone>(rng: &mut R) -> Self {
        match rng.gen::<f64>() {
            x if x < 0.2 => Self::Mul {
                left: rng.gen_range(VALUE_RANGE),
                right: rng.gen_range(VALUE_RANGE),
            },
            x if x < 0.25 => Self::WrongDelim {
                left: rng.gen_range(VALUE_RANGE),
                right: rng.gen_range(VALUE_RANGE),
                left_delim: LEFT_DELIMITERS.choose(rng).copied().unwrap().into(),
                right_delim: RIGHT_DELIMITERS.choose(rng).copied().unwrap().into(),
            },
            x if x < 0.3 => Self::OnlyOneValue {
                val: rng.gen_range(VALUE_RANGE),
            },
            x if x < 0.35 => Self::Do,
            x if x < 0.4 => Self::Dont,
            x if x < 0.5 => Self::Who,
            x if x < 0.6 => Self::What,
            x if x < 0.7 => Self::When,
            x if x < 0.8 => Self::Where,
            x if x < 0.9 => Self::Why,
            _ => Self::Select,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: String = match self {
            Token::Mul { left, right } => format!("mul({},{})", left, right),
            Token::WrongDelim {
                left,
                right,
                left_delim,
                right_delim,
            } => format!("mul{}{},{}{}", left_delim, left, right, right_delim),
            Token::OnlyOneValue { val } => format!("mul({},", val),
            Token::Who => "who()".into(),
            Token::What => "what()".into(),
            Token::When => "when()".into(),
            Token::Where => "where()".into(),
            Token::Why => "why()".into(),
            Token::Select => "select()".into(),
            Token::Do => "do()".into(),
            Token::Dont => "don't()".into(),
        };

        std::fmt::Display::fmt(&out, f)
    }
}
