// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT

//! This is a util handles the merkle tree root and proof generation.

use anyhow::Context;
use merkle_tree_rs::format::Raw;
use merkle_tree_rs::standard::{LeafType, StandardMerkleTree};
use std::marker::PhantomData;

pub type Hash = ethers::types::H256;

pub struct MerkleGen<F, E> {
    /// The function that converts the payload `E` to vec string that feeds into the inner merkle tree
    f_gen: F,
    tree: StandardMerkleTree<Raw>,
    _p: PhantomData<E>,
}

impl<F: Fn(&E) -> Vec<String>, E> MerkleGen<F, E> {
    pub fn root(&self) -> Hash {
        self.tree.root()
    }

    pub fn new<S: ToString>(f: F, values: &[E], fields: &[S]) -> anyhow::Result<Self> {
        let values = values.iter().map(&f).collect::<Vec<_>>();

        let tree =
            StandardMerkleTree::of(&values, fields).context("failed to construct Merkle tree")?;
        Ok(Self {
            f_gen: f,
            tree,
            _p: Default::default(),
        })
    }

    pub fn get_proof(&self, data: &E) -> anyhow::Result<Vec<Hash>> {
        let leaf = (self.f_gen)(data);
        self.tree.get_proof(LeafType::LeafBytes(leaf))
    }
}
