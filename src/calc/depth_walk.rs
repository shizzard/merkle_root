//!
//! Implements a depth-walk merkle tree root calculation.
//!
//! Time complexity: O(n*log(n))
//! Space complexity: O(log(n))
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
//! The idea is to walk up and down the tree, starting from the lvl1,
//! calculating the right branch (walk_down) and left branch (walk_up)
//! recursively. Algorithm terminates when:
//! - all branches converged into single hash;
//! - there are no more source hashes left.
//!
//! Pros: low disk usage, low space complexity.
//!
//! Cons: impossible to calculate parts of the tree in parallel.
//!
//! Use-cases: single-thread environments, embedded systems.

use std::iter::Peekable;

pub struct DepthWalk;

impl super::MerkleTreeRoot for DepthWalk {
    fn calculate<I, H, F>(source: &mut Peekable<I>, hash_fn: &F) -> H
    where
        I: Iterator<Item = H>,
        F: Fn(H, Option<H>) -> H,
    {
        let left = source.next().expect("Expected source not to be empty");
        walk_up(1, left, source, hash_fn)
    }
}

fn walk_up<I, H, F>(height: usize, left: H, source: &mut Peekable<I>, hash_fn: &F) -> H
where
    I: Iterator<Item = H>,
    F: Fn(H, Option<H>) -> H,
{
    let right = walk_down(height - 1, source, hash_fn);
    let hash = hash_fn(left, right);
    match source.peek() {
        // source still contains hash to continue
        Some(_) => walk_up(height + 1, hash, source, hash_fn),
        // no hashes left in the source, return the root
        None => hash,
    }
}

fn walk_down<I, H, F>(height: usize, source: &mut Peekable<I>, hash_fn: &F) -> Option<H>
where
    I: Iterator<Item = H>,
    F: Fn(H, Option<H>) -> H,
{
    if height == 0 {
        // we're at the very bottom of the tree, collect the hash from the source
        source.next()
    } else {
        // recurse down once again
        Some(hash_fn(
            walk_down(height - 1, source, hash_fn)?,
            walk_down(height - 1, source, hash_fn),
        ))
    }
}

///
/// To simplify testing, hashes and hashing function are mocked.
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

    fn hash(left: Vec<char>, right: Option<Vec<char>>) -> Vec<char> {
        let mut ret = Vec::new();
        ret.extend(&left);
        match right {
            None => ret.extend(&left),
            Some(right) => ret.extend(&right),
        }
        ret
    }

    #[test]
    #[should_panic]
    fn empty_source() {
        let mut source = Vec::<Vec<char>>::new().into_iter().peekable();
        DepthWalk::calculate(&mut source, &hash);
    }

    #[test]
    fn full_tree() {
        let mut source = vec![vec!['a'], vec!['b']].into_iter().peekable();
        assert_eq!(vec!['a', 'b'], DepthWalk::calculate(&mut source, &hash));

        let mut source = vec![vec!['a'], vec!['b'], vec!['c'], vec!['d']]
            .into_iter()
            .peekable();
        assert_eq!(
            vec!['a', 'b', 'c', 'd'],
            DepthWalk::calculate(&mut source, &hash)
        );
    }

    #[test]
    fn partial_tree() {
        let mut source = vec![vec!['a']].into_iter().peekable();
        assert_eq!(vec!['a', 'a'], DepthWalk::calculate(&mut source, &hash));

        let mut source = vec![vec!['a'], vec!['b'], vec!['c']].into_iter().peekable();
        assert_eq!(
            vec!['a', 'b', 'c', 'c'],
            DepthWalk::calculate(&mut source, &hash)
        );
    }
}
