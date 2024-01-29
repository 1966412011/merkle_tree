use anyhow::anyhow;
use ff::*;
use poseidon_rs::Fr;
use std::{collections::BTreeMap, usize, vec};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Proof {
    pub index: usize,
    pub value: String,
    pub siblings: Vec<String>,
    pub root: String,
    pub empty: bool,
}

#[derive(Clone)]
pub struct MerkleNode {
    pub left: usize,
    pub right: usize,
    pub father: usize,
    pub value: Fr,
    pub range: (usize, usize),
}

#[derive(Clone)]
pub struct MerkleTree {
    root: usize,
    size: usize,
    counter: usize,
    leaves: BTreeMap<usize, Proof>,
    nodes: BTreeMap<usize, MerkleNode>,
}

impl MerkleTree {
    pub fn new() -> Self {
        let mut nodes = BTreeMap::new();
        nodes.insert(
            1,
            MerkleNode {
                left: 1,
                right: 1,
                father: 1,
                value: Fr::zero(),
                range: (1, 1),
            },
        );
        let mut leaves = BTreeMap::new();
        leaves.insert(
            1,
            Proof {
                index: 1,
                value: String::from(""),
                siblings: Vec::new(),
                root: String::from(""),
                empty: true,
            },
        );
        MerkleTree {
            root: 0,
            counter: 1,
            size: 1,
            leaves,
            nodes,
        }
    }

    fn get_node(&self, index: usize) -> anyhow::Result<&MerkleNode> {
        self.nodes
            .get(&self.root)
            .ok_or_else(|| anyhow!("root not find"))
    }

    fn get_mut_node(&mut self, index: usize) -> anyhow::Result<&mut MerkleNode> {
        self.nodes
            .get_mut(&self.root)
            .ok_or_else(|| anyhow!("root not find"))
    }

    fn gen_new_node(&mut self, range: (usize, usize)) -> usize {
        let node = MerkleNode {
            left: 0,
            right: 0,
            father: 0,
            value: Fr::zero(),
            range,
        };
        self.counter += 1;
        let index = self.counter;
        self.nodes.insert(index, node);
        index
    }

    pub fn insert(&mut self, index: usize, value: String) -> anyhow::Result<()> {
        loop {
            let root = self.get_node(self.root)?.clone();
            if root.range.1 >= index {
                break;
            }
            let r1 = (root.range.0, root.range.1 * 2);
            let r2 = (root.range.1 + 1, root.range.1 * 2);
            let n1 = self.gen_new_node(r1);
            let n2 = self.gen_new_node(r2);
            {
                let node = self.get_mut_node(n1)?;
            }
            self.root = n1;
        }

        Ok(())
    }

    pub fn get_proof(&self, index: usize) -> Option<Proof> {
        self.leaves.get(&index).cloned()
    }

    pub fn verify(&self, proof: &Proof) -> bool {
        self.leaves
            .get(&proof.index)
            .map(|v| v == proof)
            .unwrap_or(false)
    }
}
