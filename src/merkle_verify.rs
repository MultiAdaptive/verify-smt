use blake2_rfc::blake2b::Blake2b;
use hex;

use crate::blake2b::Blake2bHasher;
use crate::h256::H256;
use crate::merkle_proof::CompiledMerkleProof;

pub struct MerkleVerify;

impl MerkleVerify {
    pub fn verify_proof(proof: &str, root: &str, leaves: &Vec<(&str, &str)>) -> bool {
        let proof = hex::decode(proof).unwrap();
        let root: [u8; 32] = hex::decode(root).unwrap().try_into().unwrap();
        let leaves = leaves.into_iter().map(|l| {
            let mut blake2b = Blake2b::new(32);
            blake2b.update(hex::decode(l.0).unwrap().as_slice().clone());
            let left: [u8; 32] = blake2b.finalize().as_bytes().try_into().unwrap();
            let right: [u8; 32] = hex::decode(l.1).unwrap().try_into().unwrap();
            (H256::from(left), H256::from(right))
        }).collect::<Vec<_>>();

        let proof = CompiledMerkleProof(proof);
        let root = H256::from(root);
        let result = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();

        return result;
    }

    pub fn merkle_verify(proof: String, root: String, leaves: Vec<(String, String)>) -> bool {
        let proof = hex::decode(proof).unwrap();
        let root: [u8; 32] = hex::decode(root).unwrap().try_into().unwrap();
        let leaves = leaves.into_iter().map(|l| {
            let mut blake2b = Blake2b::new(32);
            blake2b.update(hex::decode(l.0).unwrap().as_slice().clone());
            let left: [u8; 32] = blake2b.finalize().as_bytes().try_into().unwrap();
            let right: [u8; 32] = hex::decode(l.1).unwrap().try_into().unwrap();
            (H256::from(left), H256::from(right))
        }).collect::<Vec<_>>();

        let proof = CompiledMerkleProof(proof);
        let root = H256::from(root);
        let result = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();

        return result;
    }
}

