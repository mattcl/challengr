use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Range,
};

use itertools::Itertools;
use proliferatr::{
    generic::{token::LOWER_ALPHA_CHARS, StringToken},
    InputGenerator,
};
use rand::{seq::SliceRandom, Rng};

use super::Day;

const LAYER_SIZES: &[Range<usize>] = &[2..6, 9..15, 40..51, 100..110, 225..250];
const KEY_SIZE: Range<usize> = 2..4;
const XMAS: &[u8] = b"xmas";
const VALUES: Range<u16> = 1..4001;
const RULE_VALUES: Range<u16> = 1000..3001;
const LAST_ROW_RULES: Range<usize> = 1..3;
const NUM_RATINGS: usize = 200;

/// We're going to generate several "layers" of nodes under a layer containing
/// the single "in" node. We're then going to randomly link each node in a
/// particular layer to one or more nodes in the layer below it via Rules. The
/// nodes in the last layer will only use rules that end in Accept or Reject.
/// Because of explicit ordering for rules, this _should_ produce unique
/// solutions.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day19;

impl Day for Day19 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Day19.gen_input(rng)
    }
}

impl InputGenerator for Day19 {
    type GeneratorError = anyhow::Error;
    type Output = String;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let key_gen = StringToken::builder()
            .length(KEY_SIZE)
            .charset(LOWER_ALPHA_CHARS)
            .build()
            .unwrap();

        let sizes = LAYER_SIZES
            .iter()
            .map(|r| rng.gen_range(r.clone()))
            .collect::<Vec<_>>();

        let total: usize = sizes.iter().sum::<usize>() + 1;

        let mut raw_keys: HashSet<String> = HashSet::with_capacity(total);
        raw_keys.insert("in".into());

        while raw_keys.len() < total {
            let candidate = key_gen.gen_input(rng)?;
            if raw_keys.contains(&candidate) {
                continue;
            }

            raw_keys.insert(candidate);
        }

        raw_keys.remove("in");

        let mut keys = Vec::from_iter(raw_keys);
        keys.shuffle(rng);

        let mut key_iter = keys.iter();

        // generate nodes at each layer
        let mut workflows = vec![vec![Workflow {
            name: "in",
            ..Default::default()
        }]];
        workflows.extend(sizes.iter().map(|s| {
            (0..*s)
                .map(|_| Workflow {
                    name: key_iter.next().unwrap(),
                    ..Default::default()
                })
                .collect::<Vec<_>>()
        }));

        // handle everything but the last layer
        for i in 1..workflows.len() {
            let mut remaining = (0..workflows[i].len()).collect::<Vec<_>>();
            remaining.shuffle(rng);

            let mut cur = 0;
            // set the fallthroughs
            while let Some(idx) = remaining.pop() {
                let k = workflows[i][idx].name;

                workflows[i - 1][cur].fallthrough = k;

                cur += 1;
                if cur >= workflows[i - 1].len() {
                    break;
                }
            }

            cur = 0;

            // randomly assign rules for the remaining indexes
            while let Some(idx) = remaining.pop() {
                let k = workflows[i][idx].name;

                workflows[i - 1][cur].rules.insert(k, Rule::random(rng));

                cur += 1;
                cur %= workflows[i - 1].len();
            }
        }

        // For the last layer, all rules and fallthroughs need to be accept or
        // reject.
        let last = workflows.len() - 1;

        for i in 0..workflows[last].len() {
            workflows[last][i].fallthrough = if rng.gen_bool(0.5) { "A" } else { "R" };

            // yeah, we're just going to gen these even though we might just end
            // up overwriting the same key over and over again
            for _ in 0..(rng.gen_range(LAST_ROW_RULES)) {
                let k = if rng.gen_bool(0.5) { "A" } else { "R" };
                workflows[last][i].rules.insert(k, Rule::random(rng));
            }
        }

        let ratings = (0..NUM_RATINGS)
            .map(|_| Rating::random(rng))
            .collect::<Vec<_>>();

        // this is inefficient because of the allocations
        let mut workflow_refs = workflows
            .iter()
            .flat_map(|layer| layer.iter())
            .collect::<Vec<_>>();
        workflow_refs.shuffle(rng);

        Ok(format!(
            "{}\n\n{}",
            workflow_refs.iter().join("\n"),
            ratings.iter().join("\n"),
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Workflow<'a> {
    name: &'a str,
    rules: HashMap<&'a str, Rule>,
    fallthrough: &'a str,
}

impl<'a> Display for Workflow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rules.is_empty() {
            write!(f, "{}{{{}}}", self.name, self.fallthrough)
        } else {
            write!(
                f,
                "{}{{{},{}}}",
                self.name,
                self.rules
                    .iter()
                    .map(|(k, v)| format!("{}{}", v, k))
                    .join(","),
                self.fallthrough
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule {
    Greater { key: char, value: u16 },
    Less { key: char, value: u16 },
}

impl Rule {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        let key = *XMAS.choose(rng).unwrap() as char;
        let value = rng.gen_range(RULE_VALUES);

        if rng.gen_bool(0.5) {
            Self::Greater { key, value }
        } else {
            Self::Less { key, value }
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greater { key, value } => write!(f, "{}>{}:", key, value),
            Self::Less { key, value } => write!(f, "{}<{}:", key, value),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rating {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Rating {
    pub fn random<R: Rng + Clone + ?Sized>(rng: &mut R) -> Self {
        Self {
            x: rng.gen_range(VALUES),
            m: rng.gen_range(VALUES),
            a: rng.gen_range(VALUES),
            s: rng.gen_range(VALUES),
        }
    }
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}
