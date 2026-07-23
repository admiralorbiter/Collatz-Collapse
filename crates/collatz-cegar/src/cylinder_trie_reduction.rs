use crate::precision_aware_cylinder::Cylinder;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

/// Trie Node for Canonical Binary Cylinder Reduction.
#[derive(Debug, Clone, Default)]
pub struct TrieNode {
    pub is_cylinder_end: bool,
    pub left_0: Option<Box<TrieNode>>,
    pub right_1: Option<Box<TrieNode>>,
}

/// Phase 7.3S.2B: Canonical Binary Trie with Subsumption & Sibling Merging.
#[derive(Debug, Clone)]
pub struct CylinderTrie {
    pub root: TrieNode,
    pub provenance: HashMap<Cylinder, Vec<Vec<u64>>>,
}

impl CylinderTrie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
            provenance: HashMap::new(),
        }
    }

    /// Insert cylinder [r]_p into Trie with optional witness tail sequence v
    pub fn insert(&mut self, cyl: Cylinder, tail_seq: Option<Vec<u64>>) {
        if let Some(seq) = tail_seq {
            self.provenance.entry(cyl.clone()).or_default().push(seq);
        }

        Self::insert_node(&mut self.root, &cyl.residue, cyl.precision, 0);
        Self::compact_node(&mut self.root);
    }

    fn insert_node(node: &mut TrieNode, residue: &BigUint, precision: u32, bit_idx: u32) {
        if node.is_cylinder_end {
            // Already subsumed by shorter precision cylinder
            return;
        }

        if bit_idx == precision {
            node.is_cylinder_end = true;
            node.left_0 = None;
            node.right_1 = None;
            return;
        }

        let bit: BigUint = (residue >> bit_idx) & BigUint::one();
        if bit.is_zero() {
            if node.left_0.is_none() {
                node.left_0 = Some(Box::new(TrieNode::default()));
            }
            Self::insert_node(node.left_0.as_mut().unwrap(), residue, precision, bit_idx + 1);
        } else {
            if node.right_1.is_none() {
                node.right_1 = Some(Box::new(TrieNode::default()));
            }
            Self::insert_node(node.right_1.as_mut().unwrap(), residue, precision, bit_idx + 1);
        }
    }

    /// Sibling Merging: [r]_p U [r + 2^{p-1}]_p = [r mod 2^{p-1}]_{p-1}
    fn compact_node(node: &mut TrieNode) {
        if node.is_cylinder_end {
            return;
        }

        if let Some(ref mut l) = node.left_0 {
            Self::compact_node(l);
        }
        if let Some(ref mut r) = node.right_1 {
            Self::compact_node(r);
        }

        let left_end = node.left_0.as_ref().map_or(false, |n| n.is_cylinder_end);
        let right_end = node.right_1.as_ref().map_or(false, |n| n.is_cylinder_end);

        if left_end && right_end {
            node.is_cylinder_end = true;
            node.left_0 = None;
            node.right_1 = None;
        }
    }

    /// Collect minimal reduced list of cylinders from Trie
    pub fn to_cylinders(&self) -> Vec<Cylinder> {
        let mut result = Vec::new();
        let mut current_residue = BigUint::default();
        Self::collect_nodes(&self.root, 0, &mut current_residue, &mut result);
        result
    }

    fn collect_nodes(node: &TrieNode, precision: u32, current_residue: &mut BigUint, acc: &mut Vec<Cylinder>) {
        if node.is_cylinder_end {
            acc.push(Cylinder::new(current_residue.clone(), precision));
            return;
        }

        if let Some(ref l) = node.left_0 {
            Self::collect_nodes(l, precision + 1, current_residue, acc);
        }

        if let Some(ref r) = node.right_1 {
            let bit_val = BigUint::one() << precision;
            *current_residue += &bit_val;
            Self::collect_nodes(r, precision + 1, current_residue, acc);
            *current_residue -= &bit_val;
        }
    }

    /// Check if target integer D belongs to any cylinder in the Trie
    pub fn contains_endpoint(&self, endpoint: &BigUint) -> bool {
        let mut curr = &self.root;
        let mut bit_idx = 0;

        loop {
            if curr.is_cylinder_end {
                return true;
            }

            let bit: BigUint = (endpoint >> bit_idx) & BigUint::one();
            if bit.is_zero() {
                if let Some(ref l) = curr.left_0 {
                    curr = l;
                } else {
                    return false;
                }
            } else {
                if let Some(ref r) = curr.right_1 {
                    curr = r;
                } else {
                    return false;
                }
            }
            bit_idx += 1;
        }
    }
}
