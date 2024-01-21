use std::{collections::HashSet, convert::Infallible, fmt::Display};

use itertools::Itertools;
use proliferatr::InputGenerator;
use rand::{seq::SliceRandom, Rng};

use super::Day;

// I don't know the actual ranges of these things, so we're going to do a best
// guess based on solutions that were posted.
const NUM_LOOPS: usize = 6;
const LOOP_PRIMES: &[usize] = &[
    23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 97,
];
const INST_PRIMES: &[usize] = &[
    211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313,
    317, 331, 337, 347,
];
// anything goes but A and Z
const CHARSET: &[u8] = b"BCDEFGHIJKLMNOPQRSTUVWXY";
const P_CONTINUE_RUN: f64 = 0.70;
const MAX_RUN: usize = 4;

/// So the real inputs are very special, in that they describe six separate
/// "loops" of nodes, where the cycle length in any loop from Z -> Z and A -> Z
/// is identical, where that length is 2-digit prime number N, multplied by a
/// prime K, where K is the length of the left/right instructions as is probably
/// between 200 and 400. Eash node having a left/right (though some have the
/// same destination for both left and right), allows for the number of actual
/// nodes in each loop to be smaller than K * N. The expected solution to the
/// problem is therefore K * N1 * N2 * N3 * N4 * N5 * N6.
///
/// The inputs are designed so that there's a "shunt" of nodes near the end of
/// a cycle whose left and right pointers both point at the "left" node. Once
/// you enter this shunt, you will bypass the Z node no matter which other
/// directions you take. The length of this shunt is such that you need 4
/// sequential right moves to reach the Z Node. The real inputs L/R strings end
/// with RRRR, and the rest of the input is not allowed to have another sequence
/// like that in it. This probably ensures that you don't have to take input
/// length into account because any variation to RRRR will cause you to miss the
/// Z node.
///
/// No node other than an entrypoint into a loop may start with A. No node other
/// than then end of a cycle may end with Z
///
/// To make part 1 work, only one loop can contain AAA, and that loop must also
/// contain ZZZ.
///
/// I really dislike problems like this one, but, if we want to generate valid
/// inputs, it's what we have to do.
#[derive(Debug, Default, Clone, Copy)]
pub struct Day08;

impl Day for Day08 {
    fn generate<R: Rng + Clone + ?Sized>(
        rng: &mut R,
    ) -> Result<String, <Self as proliferatr::InputGenerator>::GeneratorError> {
        let (inst, nodes) = Day08.gen_input(rng)?;
        Ok(format!("{}\n\n{}", &inst, nodes.iter().join("\n")))
    }
}

impl InputGenerator for Day08 {
    type GeneratorError = Infallible;
    type Output = (String, Vec<Node>);

    fn gen_input<R: Rng + Clone + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<Self::Output, Self::GeneratorError> {
        // select 6 numbers to make loops from
        let lengths = LOOP_PRIMES
            .choose_multiple(rng, NUM_LOOPS)
            .copied()
            .collect::<Vec<_>>();
        let inst_length = INST_PRIMES.choose(rng).copied().unwrap();
        let mut seen: HashSet<String> =
            HashSet::with_capacity(lengths.iter().sum::<usize>() * 2 + NUM_LOOPS);
        let mut nodes = Vec::with_capacity(lengths.iter().sum::<usize>() * 2 + NUM_LOOPS);
        let mut instructions = String::with_capacity(inst_length);

        // we have to start with 'L' so we don't accidentally create another run
        // of 4 'R's
        let mut prev = 'L';
        let mut run = 1;
        instructions.push(prev);

        for _ in 1..(inst_length - 5) {
            if prev == 'R' && run >= 3 {
                // we have to pick an 'L'
                instructions.push('L');
                prev = 'L';
                run = 1;
            } else if rng.gen_bool(P_CONTINUE_RUN) && run < MAX_RUN {
                // continue the current run
                instructions.push(prev);
                run += 1;
            } else {
                // switch chars
                prev = if prev == 'L' { 'R' } else { 'L' };
                run = 1;
                instructions.push(prev);
            }
        }

        // the last 5 chars are fixed, because we need to make sure we break a
        // potential existing run of 'R's and then include 4 'R's
        instructions.push('L');
        instructions.push('R');
        instructions.push('R');
        instructions.push('R');
        instructions.push('R');

        // now generate the loops

        // the first loop is special because it'll contain AAA and ZZZ.
        seen.insert("AAA".into());
        seen.insert("ZZZ".into());
        nodes.extend(make_loop(rng, lengths[0], "AAA", "ZZZ", &mut seen));

        #[allow(clippy::needless_range_loop)]
        for i in 1..lengths.len() {
            let start = loop {
                let mut s = String::with_capacity(3);
                s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
                s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
                s.push('A');
                if !seen.contains(&s) {
                    seen.insert(s.clone());
                    break s;
                }
            };
            let end = loop {
                let mut s = String::with_capacity(3);
                s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
                s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
                s.push('Z');
                if !seen.contains(&s) {
                    seen.insert(s.clone());
                    break s;
                }
            };
            nodes.extend(make_loop(rng, lengths[i], &start, &end, &mut seen));
        }

        // randomize the order of all the nodes to obscrure the implementation
        nodes.shuffle(rng);

        Ok((instructions, nodes))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    name: String,
    left: String,
    right: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = ({}, {})", &self.name, &self.left, &self.right)
    }
}

fn make_loop<R: Rng + Clone + ?Sized>(
    rng: &mut R,
    len: usize,
    start: &str,
    end: &str,
    seen: &mut HashSet<String>,
) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(len * 2 + 1);

    // the first node is outside of the loop
    nodes.push(Node {
        name: start.to_string(),
        left: make_name(rng, seen),
        right: make_name(rng, seen),
    });

    let mut cur = vec![nodes[0].left.clone(), nodes[0].right.clone()];

    // last 4 nodes are special
    for _ in 0..(len - 4) {
        let left_child_name = make_name(rng, seen);
        let right_child_name = make_name(rng, seen);

        let right = Node {
            name: cur.pop().unwrap(),
            left: left_child_name.clone(),
            right: right_child_name.clone(),
        };

        let left = Node {
            name: cur.pop().unwrap(),
            left: left_child_name.clone(),
            right: right_child_name.clone(),
        };

        // yeah, this ordering isn't going to be a problem or anything
        cur.push(left_child_name);
        cur.push(right_child_name);

        // this means that nodes[1] is going to be the first left and
        // nodes[2] is going to be the first right
        nodes.push(left);
        nodes.push(right);
    }

    // shunt
    for _ in 0..2 {
        let left_child_name = make_name(rng, seen);
        let right_child_name = make_name(rng, seen);

        // the right node is normal
        let right = Node {
            name: cur.pop().unwrap(),
            left: left_child_name.clone(),
            right: right_child_name.clone(),
        };

        // the left node has both it's left and right pointing to the left
        let left = Node {
            name: cur.pop().unwrap(),
            left: left_child_name.clone(),
            right: left_child_name.clone(),
        };

        cur.push(left_child_name);
        cur.push(right_child_name);

        nodes.push(left);
        nodes.push(right);
    }

    // node group of the shunt leading to the end
    let left_child_name = make_name(rng, seen);
    let right_child_name = end.to_string();

    let right = Node {
        name: cur.pop().unwrap(),
        left: left_child_name.clone(),
        right: right_child_name.clone(),
    };

    let left = Node {
        name: cur.pop().unwrap(),
        left: left_child_name.clone(),
        right: left_child_name.clone(),
    };

    cur.push(left_child_name);
    cur.push(right_child_name);

    nodes.push(left);
    nodes.push(right);

    // then the last two nodes. The left is "normal", the right is our special
    // end node
    let left_child_name = nodes[1].name.clone();
    let right_child_name = nodes[2].name.clone();

    let right = Node {
        name: cur.pop().unwrap(),
        left: left_child_name.clone(),
        right: right_child_name.clone(),
    };

    let left = Node {
        name: cur.pop().unwrap(),
        left: left_child_name.clone(),
        right: right_child_name.clone(),
    };

    nodes.push(left);
    nodes.push(right);

    nodes
}

fn make_name<R: Rng + Clone + ?Sized>(rng: &mut R, seen: &mut HashSet<String>) -> String {
    loop {
        let mut s = String::with_capacity(3);
        s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
        s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
        s.push(CHARSET[rng.gen_range(0..CHARSET.len())] as char);
        if !seen.contains(&s) {
            seen.insert(s.clone());
            return s;
        }
    }
}
