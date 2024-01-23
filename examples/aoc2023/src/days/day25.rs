use std::{collections::HashSet, fmt::Display, ops::Range};

use itertools::Itertools;
use proliferatr::{
    generic::{token::LOWER_ALPHA_CHARS, StringToken},
    InputGenerator,
};
use rand::{
    distributions::Uniform,
    prelude::Distribution,
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use super::Day;

const NUM_NODES: Range<usize> = 750..821;
const NAME_LEN: usize = 3;
const NUM_NEIGHBORS: Range<usize> = 4..7;
const BI_DIRECTIONAL_PROB: f64 = 0.25;

/// Generate two graphs of about 800 nodes each. Pick three nodes from each
/// graph and join the graphs via those nodes. We're going to disguise the fact
/// that every node has at least 4 connections (so I don't have to worry about
/// a second, smaller cut) by omitting some of the neighbors when we transform
/// the graph into into a string. We can do this as long as the edge is
/// described by another line in the code
#[derive(Debug, Default, Clone, Copy)]
pub struct Day25;

impl Day for Day25 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        Ok(Day25.gen_input(rng)?.iter().join("\n"))
    }
}

impl InputGenerator for Day25 {
    type GeneratorError = anyhow::Error;
    type Output = Vec<Node>;

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        let key_gen = StringToken::builder()
            .length(NAME_LEN..=NAME_LEN)
            .charset(LOWER_ALPHA_CHARS)
            .build()
            .unwrap();

        let left_count = rng.gen_range(NUM_NODES);
        let right_count = rng.gen_range(NUM_NODES);
        let mut seen = HashSet::with_capacity(2000);
        let mut raw_graph = Vec::with_capacity(left_count + right_count);
        gen_graph(rng, &key_gen, left_count, 0, &mut seen, &mut raw_graph)?;
        gen_graph(
            rng,
            &key_gen,
            right_count,
            left_count,
            &mut seen,
            &mut raw_graph,
        )?;
        let mut seen_edges: HashSet<(usize, usize)> = HashSet::default();
        let mut graph = Vec::with_capacity(left_count + right_count);

        // pick three nodes for each
        let mut left_bridges = (0..left_count).choose_multiple(rng, 3);
        let mut right_bridges = (left_count..(left_count + right_count)).choose_multiple(rng, 3);

        left_bridges.shuffle(rng);
        right_bridges.shuffle(rng);

        // join the two groups of nodes via the selected nodes
        for (left, right) in left_bridges.into_iter().zip(right_bridges.into_iter()) {
            raw_graph[left].neighbors.insert(right);
            raw_graph[right].neighbors.insert(left);
        }

        // transform the raw nodes to real nodes
        for (idx, rn) in raw_graph.iter().enumerate() {
            let mut node = Node {
                name: rn.name.clone(),
                ..Default::default()
            };

            for n in rn.neighbors.iter().copied() {
                // we want to hide the fact that all nodes have at least 4
                // edges, so we're going to sometimes avoid recording the edge
                // in the other direction
                let key = (idx.min(n), idx.max(n));

                if !seen_edges.contains(&key)
                    || rng.gen_bool(BI_DIRECTIONAL_PROB)
                    || node.neighbors.is_empty()
                {
                    // fetch the name of that neighbor
                    node.neighbors.push(raw_graph[n].name.clone());
                    seen_edges.insert(key);
                }
            }

            graph.push(node);
        }

        graph.shuffle(rng);

        Ok(graph)
    }
}

fn gen_graph<R: Rng + Clone + ?Sized>(
    rng: &mut R,
    key_gen: &StringToken,
    count: usize,
    start_offset: usize,
    seen: &mut HashSet<String>,
    graph: &mut Vec<RawNode>,
) -> anyhow::Result<()> {
    // make all the nodes
    while graph.len() < count + start_offset {
        let name = key_gen.gen_input(rng)?;

        if seen.contains(&name) {
            continue;
        }

        seen.insert(name.clone());

        graph.push(RawNode {
            name,
            ..Default::default()
        });
    }

    // the indexes valid for this group of the graph
    let indexes = Uniform::from(start_offset..(start_offset + count));

    // assign neighbors
    for i in start_offset..(start_offset + count) {
        let desired_neighbors = rng.gen_range(NUM_NEIGHBORS);
        if graph[i].neighbors.len() >= desired_neighbors {
            // we don't need to do anything because other nodes already did
            continue;
        }

        while graph[i].neighbors.len() < desired_neighbors {
            let nidx = indexes.sample(rng);

            if nidx == i {
                continue;
            }

            if graph[i].neighbors.contains(&nidx) {
                continue;
            }

            graph[i].neighbors.insert(nidx);
            graph[nidx].neighbors.insert(i);
        }
    }

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub struct RawNode {
    name: String,
    neighbors: HashSet<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    name: String,
    neighbors: Vec<String>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.name, self.neighbors.join(" "))
    }
}
