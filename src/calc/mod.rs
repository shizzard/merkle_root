use crate::Hash;
use sha2::{Digest, Sha256};
use std::iter::Peekable;

pub mod depth_walk;

///
/// Generic Merkle Tree root calculator trait.
pub trait MerkleTreeRoot {
    ///
    /// The entry point to calculate the Merkle Tree root with the given
    /// strategy.
    fn calculate<I, H, F>(source: &mut Peekable<I>, hash_fn: &F) -> H
    where
        I: Iterator<Item = H>,
        F: Fn(H, Option<H>) -> H;
}

///
/// Calculates the hash of node, given the left and right branch hashes.
///
/// Left branch must be present. If the right branch hash is `None`, then the
/// left branch hash is copied over and hashed with itself.
///
/// # Examples:
///
/// ```
/// let left = [0u8; 32];
/// let right = [1u8; 32];
///
/// let result = hash(left, Some(right)); // hashing with both branches
/// let result = hash(left, None);        // hashing with the empty right branch
/// let result = hash(left, Some(left));  // same result, but less effective
/// ```
pub fn hash(left: Hash, right: Option<Hash>) -> Hash {
    let mut input = [0u8; 64];

    input[..32].copy_from_slice(&left);
    if let Some(hash) = right {
        // right branch has a hash, proceed
        input[32..].copy_from_slice(&hash);
    } else {
        // right branch is empty, copy the left hash and proceed
        input[32..].copy_from_slice(&left);
    };

    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}
