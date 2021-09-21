// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    errors::MerkleTrieError,
    merkle_trie::{calculate_root, hash_leaf},
    traits::CRH,
};
use snarkvm_utilities::ToBytes;

use itertools::Itertools;
use std::sync::Arc;

pub struct MerkleTriePath<P: CRH, T: ToBytes> {
    pub parameters: Arc<P>,
    /// A Vector of existing sibling children from leaf to root.
    /// (Does NOT including the parents of the leaf being proven)
    pub path: Vec<Vec<[u8; 32]>>, // Vector of branch roots
    pub parents: Vec<(Vec<u8>, Option<T>)>, // Vector of (key, value) pairs.
    pub traversal: Vec<usize>,
}

impl<P: CRH, T: ToBytes> MerkleTriePath<P, T> {
    pub fn verify(&self, root_hash: &[u8; 32], key: &[u8], value: &T) -> Result<bool, MerkleTrieError> {
        assert_eq!(self.path.len(), self.traversal.len());
        assert_eq!(self.parents.len(), self.traversal.len());

        let mut curr_hash = calculate_root(&self.parameters, &key, &Some(value), &vec![])?;

        println!("\n LEAF HASH: {:?}", curr_hash);

        // Check that the given leaf matches the leaf in the membership proof.
        if !self.path.is_empty() {
            for (i, (index, siblings)) in self.traversal.iter().zip_eq(self.path.iter()).enumerate() {
                let mut node_hashes: Vec<&[u8; 32]> = siblings.iter().map(|x| x).collect();
                node_hashes.insert(*index, &curr_hash);

                let (key, value) = &self.parents[i];

                curr_hash = calculate_root(&self.parameters, key, value, &node_hashes)?;
            }

            // Check if final hash is root
            if &curr_hash != root_hash {
                return Ok(false);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
