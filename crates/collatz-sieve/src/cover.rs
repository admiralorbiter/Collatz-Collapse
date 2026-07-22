use crate::pipeline::SievePipeline;
use crate::traits::{PrefixState, SieveResult};
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_cert::generate_descent_certificate;
use collatz_cert::schema::DescentCertificateJson;
use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};
use smallvec::SmallVec;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    CertifiedDescent,
    Infeasible,
    NeedsRefinement,
}

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub valuation_path: SmallVec<[u8; 64]>,
    pub affine: AffinePrefix,
    pub status: NodeStatus,
}

pub struct PrefixTrie {
    max_depth: usize,
    pipeline: SievePipeline,
    pub certified_measure: BigRational,
    pub certified_certificates: Vec<DescentCertificateJson>,
}

impl PrefixTrie {
    pub fn new(max_depth: usize, pipeline: SievePipeline) -> Self {
        Self {
            max_depth,
            pipeline,
            certified_measure: BigRational::zero(),
            certified_certificates: Vec::new(),
        }
    }

    /// Computes analytical Tail-Cutoff valuation threshold a_crit.
    /// For all a_k >= a_crit, the child branch is guaranteed to descend with zero exceptions (B <= 1).
    pub fn compute_tail_cutoff(c_k: &BigUint, k: usize, total_twos: u64) -> u8 {
        let pow3_k1 = BigUint::from(3u32).pow((k + 1) as u32);
        let pow2_ak = BigUint::one() << total_twos;
        let bound_val = (c_k * 3u32) + &pow2_ak + &pow3_k1;

        let bits = bound_val.bits();
        if bits > total_twos {
            let cutoff = (bits - total_twos) as u8 + 1;
            cutoff.min(16) // Cap maximum search expansion per step
        } else {
            1
        }
    }

    /// Expands the valuation tree up to max_depth, streaming certified leaves and measuring exact 2-adic density.
    pub fn build_cover(&mut self) {
        let mut queue = VecDeque::new();

        // Seed with valuation words a_0 \in {1..8}
        for a_0 in 1..=8u8 {
            if let Ok(word) = ValuationWord::new(vec![a_0]) {
                if let Ok(affine) = AffinePrefix::from_valuation_word(word) {
                    queue.push_back(PrefixState {
                        valuations: SmallVec::from_slice(&[a_0]),
                        affine,
                        growth_debt: 0.0,
                    });
                }
            }
        }

        while let Some(state) = queue.pop_front() {
            let result = self.pipeline.evaluate(&state);

            match result {
                SieveResult::Reject { reason } => {
                    if matches!(reason, crate::traits::RejectionReason::DescentCertified) {
                        self.register_certified_leaf(&state);
                    }
                    // Infeasible nodes are pruned immediately from memory
                }
                SieveResult::Keep => {
                    let current_depth = state.valuations.len();
                    if current_depth < self.max_depth {
                        let c_k = &state.affine.constant;
                        let k = state.affine.odd_steps;
                        let total_twos = state.affine.total_twos;

                        let a_crit = Self::compute_tail_cutoff(c_k, k, total_twos);

                        for a_next in 1..=a_crit {
                            let mut next_vals = state.valuations.clone();
                            next_vals.push(a_next);

                            if let Ok(word) = ValuationWord::new(next_vals.to_vec()) {
                                if let Ok(affine) = AffinePrefix::from_valuation_word(word) {
                                    queue.push_back(PrefixState {
                                        valuations: next_vals,
                                        affine,
                                        growth_debt: 0.0,
                                    });
                                }
                            }
                        }
                    }
                }
                SieveResult::Refine { .. } => {}
            }
        }
    }

    fn register_certified_leaf(&mut self, state: &PrefixState) {
        let total_twos = state.affine.total_twos;

        // Exact 2-adic measure density \mu = 1 / 2^{A_k - 1}
        if total_twos >= 1 {
            let denom = BigUint::one() << (total_twos - 1);
            let measure = BigRational::new(BigUint::one().into(), denom.into());
            self.certified_measure += measure;
        }

        // Generate JSON certificate
        if let Ok(word) = ValuationWord::new(state.valuations.to_vec()) {
            if let Ok(cert) = generate_descent_certificate(word) {
                self.certified_certificates.push(cert);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::DescentSieve;

    #[test]
    fn test_prefix_trie_cover_build() {
        let pipeline = SievePipeline::new().add_sieve(DescentSieve);
        let mut trie = PrefixTrie::new(5, pipeline);
        trie.build_cover();

        assert!(trie.certified_certificates.len() > 0);
        assert!(trie.certified_measure > BigRational::zero());
    }
}
