//!
//! Implements a depth-walk merkle tree root calculation.
//!
//! Time complexity: O(n*log(n))
//! Space complexity: O(n)
//!
//! The algorithm structures the merkle tree as a tree with arbitrary height
//!
//! lvl3           abcdefef
//!               /       |
//! lvl2       abcd    efef
//!           /   |   /   |
//! lvl1     ab  cd  ef
//!         / | / | / |
//! lvl0    a b c d e f
//!
//! The idea is to calculate level by level, utilizing the rayon crate to
//! perform operations in parallel. Algorithm terminates when all branches
//! converge into single hash.
//!
//! Pros: parallel execution, might be fast in multicore systems.
//!
//! Cons: many memory allocations.
//!
//! Use-cases: multi-thread environments, systems with large memory pools.

use rayon::prelude::*;
use std::iter::Peekable;

pub struct WidthWalk;

impl super::MerkleTreeRoot for WidthWalk {
    fn calculate<I, H, F>(source: &mut Peekable<I>, hash_fn: &F) -> H
    where
        I: Iterator<Item = H>,
        F: Fn(&H, Option<&H>) -> H,
        F: Sync + Send,
        H: Sync + Send,
    {
        let layer: Vec<H> = source.collect();
        if layer.len() == 0 {
            panic!("Expected source not to be empty");
        }
        walk_layers(layer, hash_fn)
    }
}

fn walk_layers<H, F>(mut layer: Vec<H>, hash_fn: &F) -> H
where
    F: Fn(&H, Option<&H>) -> H,
    F: Sync + Send,
    H: Sync + Send,
{
    if layer.len() == 1 {
        return layer.pop().unwrap();
    }
    let next_layer = layer
        .par_chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                hash_fn(&chunk[0], Some(&chunk[1]))
            } else {
                hash_fn(&chunk[0], None)
            }
        })
        .collect();
    walk_layers(next_layer, hash_fn)
}

///
/// Hash is a Vec<char>, e.g. vec!['a'].
///
/// Hashing two branches is defined as a vector, expanded from the left and
/// right branches, e.g.
/// hash(vec!['a'], Some(vec!['b'])) => vec!['a', 'b']
#[cfg(test)]
mod tests {
    use crate::calc::MerkleTreeRoot;

    use super::*;

    fn hash(left: &Vec<char>, right: Option<&Vec<char>>) -> Vec<char> {
        let mut ret = Vec::new();
        ret.extend(left);
        match right {
            None => ret.extend(left),
            Some(right) => ret.extend(right),
        }
        ret
    }

    #[test]
    #[should_panic]
    fn empty_source() {
        let mut source = Vec::<Vec<char>>::new().into_iter().peekable();
        WidthWalk::calculate(&mut source, &hash);
    }

    #[test]
    fn full_tree() {
        let mut source = vec![vec!['a'], vec!['b']].into_iter().peekable();
        assert_eq!(vec!['a', 'b'], WidthWalk::calculate(&mut source, &hash));

        let mut source = vec![vec!['a'], vec!['b'], vec!['c'], vec!['d']]
            .into_iter()
            .peekable();
        assert_eq!(
            vec!['a', 'b', 'c', 'd'],
            WidthWalk::calculate(&mut source, &hash)
        );
    }

    #[test]
    fn partial_tree() {
        let mut source = vec![vec!['a']].into_iter().peekable();
        assert_eq!(vec!['a'], WidthWalk::calculate(&mut source, &hash));

        let mut source = vec![vec!['a'], vec!['b'], vec!['c']].into_iter().peekable();
        assert_eq!(
            vec!['a', 'b', 'c', 'c'],
            WidthWalk::calculate(&mut source, &hash)
        );
    }
}
