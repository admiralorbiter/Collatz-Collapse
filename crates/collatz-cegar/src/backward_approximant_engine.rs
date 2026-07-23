use crate::cylinder_trie_reduction::CylinderTrie;
use crate::precision_aware_cylinder::Cylinder;
use num_bigint::BigUint;

/// Phase 7.3S.2B: Backward Fixed-Point Approximant Engine E_{n+1} = Phi_J(E_n).
#[derive(Debug, Clone)]
pub struct BackwardApproximantEngine {
    pub max_gap: u64,
}

impl BackwardApproximantEngine {
    pub fn new(max_gap: u64) -> Self {
        Self { max_gap }
    }

    /// Base Set E_0 = Z_2 = [0]_0
    pub fn base_set(&self) -> CylinderTrie {
        let mut trie = CylinderTrie::new();
        trie.insert(Cylinder::new(BigUint::from(0u64), 0), Some(vec![]));
        trie
    }

    /// Inductive Step E_{n+1} = Phi_J(E_n)
    pub fn step_backward(&self, current_e_n: &CylinderTrie) -> CylinderTrie {
        let mut next_trie = CylinderTrie::new();
        let target_cylinders = current_e_n.to_cylinders();

        for target in &target_cylinders {
            let parent_seqs = current_e_n.provenance.get(target).cloned().unwrap_or_else(|| vec![vec![]]);

            for j in 0..=self.max_gap {
                let pred_cyl = Cylinder::pre_j(target, j);

                for parent_seq in &parent_seqs {
                    let mut extended_seq = vec![j];
                    extended_seq.extend_from_slice(parent_seq);
                    next_trie.insert(pred_cyl.clone(), Some(extended_seq));
                }
            }
        }

        next_trie
    }

    /// Compute sequence of approximants E_0, E_1, ..., E_depth
    pub fn compute_approximant_sequence(&self, depth: usize) -> Vec<CylinderTrie> {
        let mut sequence = Vec::with_capacity(depth + 1);
        let mut current = self.base_set();
        sequence.push(current.clone());

        for _ in 1..=depth {
            current = self.step_backward(&current);
            sequence.push(current.clone());
        }

        sequence
    }
}
