use serde::Deserialize;
use serde::Serialize;

use crate::sha256::Hash;
use crate::types::Transaction;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate(transactions: &[Transaction]) -> Self {
        let mut layer: Vec<Hash> = vec![];
        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }

        while layer.len() > 1 {
            let mut new_layer = vec![];
            for pair in layer.chunks(2) {
                let left = pair[0];
                let right = pair.get(1).unwrap_or(&left);
                new_layer.push(Hash::hash(&[left, *right]))
            }
            layer = new_layer;
        }
        MerkleRoot(layer[0])
    }
}
