use anyhow::anyhow;
use poseidon_rs::Fr;
use std::usize;

use crate::{gen_poseidon_hash, gen_poseidon_hash_by_fr};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Proof {
    pub index: usize,
    pub value: String,
    pub siblings: Vec<String>,
    pub root: String,
    pub empty: bool,
}

#[derive(Clone)]
pub struct MerkleTree {
    data: Vec<Vec<Fr>>,
    leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            data: Vec::new(),
            leaves: Vec::new(),
        }
    }

    fn get_height(&self) -> usize {
        let mut tl = 1;
        let mut height = 0;
        while tl < self.leaves.len() {
            tl = tl * 2;
            height += 1;
        }
        height
    }

    fn update(&mut self, mut index: usize, value: String) -> anyhow::Result<()> {
        let vfr = gen_poseidon_hash(value.as_bytes())?;
        self.leaves[index] = value;
        let height = self.get_height();
        for i in 0..height {
            if i == 0 {
                self.data[i][index] = vfr;
            } else {
                let fr1 = self.data[i - 1][index * 2];
                let fr2 = self.data[i - 1][index * 2 + 1];
                let fr = gen_poseidon_hash_by_fr(vec![fr1, fr2])?;
                self.data[i][index] = fr;
            }
            index = index / 2;
        }
        Ok(())
    }

    fn push_node(&mut self, value: String) -> anyhow::Result<()> {
        let vfr = gen_poseidon_hash(value.as_bytes())?;
        self.leaves.push(value);
        let height = self.get_height();
        for i in 0..height {
            if self.data.len() <= i {
                self.data.push(Vec::new());
            }
            if i == 0 {
                self.data[i].push(vfr);
            } else {
                let tl = self.data[i].len();
                let fr1 = self.data[i - 1][tl * 2];
                let fr2 = self.data[i - 1][tl * 2 + 1];
                let fr = gen_poseidon_hash_by_fr(vec![fr1, fr2])?;
                self.data[i].push(fr);
            }
        }

        Ok(())
    }

    pub fn insert_leaf(&mut self, index: usize, value: String) -> anyhow::Result<()> {
        if index == self.leaves.len() {
            self.push_node(value)
        } else if index < self.leaves.len() {
            self.update(index, value)
        } else {
            Err(anyhow!("index out of range"))
        }
    }

    pub fn get_proof(&self, mut index: usize) -> Option<Proof> {
        if index >= self.leaves.len() {
            return None;
        }
        let mut siblings = Vec::new();
        let height = self.get_height();
        for i in 0..height {
            siblings.push(self.data[i][index].to_string());
            index = index / 2;
        }
        Some(Proof {
            index,
            value: self.leaves[index].clone(),
            siblings,
            root: self.data[height - 1][0].to_string(),
            empty: false,
        })
    }

    pub fn verify(&self, proof: &Proof) -> bool {
        self.get_proof(proof.index)
            .map(|ref v| v == proof)
            .unwrap_or(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = MerkleTree::new();
        tree.insert_leaf(0, "hello".to_string()).unwrap();
        tree.insert_leaf(1, "world".to_string()).unwrap();
        tree.insert_leaf(2, "hello world".to_string()).unwrap();
        let proof = tree.get_proof(0).unwrap();
        println!("proof: {:?}", proof);
        assert!(tree.verify(&proof));
    }
}