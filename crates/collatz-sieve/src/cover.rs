use crate::measure_trie::MeasureTrie;
use crate::pipeline::SievePipeline;
use crate::traits::{PrefixState, SieveResult};
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_cert::generate_descent_certificate;
use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};
use smallvec::SmallVec;
use std::io::Write;

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
    pub raw_overlap_mass: BigRational,
    pub exact_cylinder_measure: BigRational,
    pub measure_trie: MeasureTrie,
    pub certified_count: usize,
}

impl PrefixTrie {
    pub fn new(max_depth: usize, pipeline: SievePipeline) -> Self {
        Self {
            max_depth,
            pipeline,
            certified_measure: BigRational::zero(),
            raw_overlap_mass: BigRational::zero(),
            exact_cylinder_measure: BigRational::zero(),
            measure_trie: MeasureTrie::new(),
            certified_count: 0,
        }
    }

    /// Returns canonical 2-adic union measure in [0, 1] computed via the LSB Patricia Trie.
    pub fn broad_union_measure(&self) -> BigRational {
        self.measure_trie.canonical_union_measure()
    }

    /// Returns unresolved measure (1.0 - broad_union_measure).
    pub fn unresolved_measure(&self) -> BigRational {
        BigRational::one() - self.broad_union_measure()
    }

    /// Computes analytical Tail-Cutoff valuation threshold a_crit using exact integer bit length:
    /// a_crit = max(1, bitlength(3c_k + 2^{A_k} + 3^{k+1}) - A_k)
    pub fn compute_tail_cutoff(c_k: &BigUint, k: usize, total_twos: u64) -> u8 {
        let pow3_k1 = BigUint::from(3u32).pow((k + 1) as u32);
        let pow2_ak = BigUint::one() << total_twos;
        let bound_val = (c_k * 3u32) + &pow2_ak + &pow3_k1;

        let bits = bound_val.bits();
        if bits > total_twos {
            let cutoff = (bits - total_twos) as u8;
            cutoff.max(1).min(16) // Cap maximum search expansion per DFS step
        } else {
            1
        }
    }

    /// Expands the valuation tree up to max_depth using Depth-First Search (DFS).
    /// Memory complexity is O(max_depth) instead of O(width), ensuring RAM usage remains < 1 MB.
    pub fn build_cover_streaming<W: Write>(&mut self, writer: &mut W) -> std::io::Result<()> {
        let mut stack: Vec<PrefixState> = Vec::with_capacity(self.max_depth * 16);

        // Seed stack with valuation words a_0 \in {1..8} in reverse order for DFS
        for a_0 in (1..=8u8).rev() {
            if let Ok(word) = ValuationWord::new(vec![a_0]) {
                if let Ok(affine) = AffinePrefix::from_valuation_word(word) {
                    stack.push(PrefixState {
                        valuations: SmallVec::from_slice(&[a_0]),
                        affine,
                        growth_debt: 0.0,
                    });
                }
            }
        }

        while let Some(state) = stack.pop() {
            let result = self.pipeline.evaluate(&state);

            match result {
                SieveResult::Reject { reason } => {
                    if matches!(reason, crate::traits::RejectionReason::DescentCertified) {
                        self.register_and_stream_certified_leaf(&state, writer)?;
                    }
                    // Infeasible nodes drop immediately from memory
                }
                SieveResult::Keep => {
                    let current_depth = state.valuations.len();
                    if current_depth < self.max_depth {
                        let c_k = &state.affine.constant;
                        let k = state.affine.odd_steps;
                        let total_twos = state.affine.total_twos;

                        let a_crit = Self::compute_tail_cutoff(c_k, k, total_twos);

                        for a_next in (1..=a_crit).rev() {
                            let mut next_vals = state.valuations.clone();
                            next_vals.push(a_next);

                            if let Ok(word) = ValuationWord::new(next_vals.to_vec()) {
                                if let Ok(affine) = AffinePrefix::from_valuation_word(word) {
                                    stack.push(PrefixState {
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

        Ok(())
    }

    fn register_and_stream_certified_leaf<W: Write>(
        &mut self,
        state: &PrefixState,
        writer: &mut W,
    ) -> std::io::Result<()> {
        let total_twos = state.affine.total_twos;

        // Broad measure = 1 / 2^{A_k - 1}
        if total_twos >= 1 {
            let broad_denom = BigUint::one() << (total_twos - 1);
            let broad_m = BigRational::new(BigUint::one().into(), broad_denom.into());
            self.raw_overlap_mass += &broad_m;
            self.certified_measure += &broad_m;

            // Exact cylinder measure = 1 / 2^{A_k}
            let exact_denom = BigUint::one() << total_twos;
            let exact_m = BigRational::new(BigUint::one().into(), exact_denom.into());
            self.exact_cylinder_measure += exact_m;

            // Insert broad residue class into LSB Patricia Trie for canonical union computation
            self.measure_trie.insert(&state.affine.starting_residue, total_twos);
        }

        self.certified_count += 1;

        // Generate JSON certificate and stream directly to disk
        if let Ok(word) = ValuationWord::new(state.valuations.to_vec()) {
            if let Ok(cert) = generate_descent_certificate(word) {
                if let Ok(line) = serde_json::to_string(&cert) {
                    writeln!(writer, "{}", line)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::DescentSieve;

    #[test]
    fn test_prefix_trie_streaming_cover_build() {
        let pipeline = SievePipeline::new().add_sieve(DescentSieve);
        let mut trie = PrefixTrie::new(5, pipeline);
        let mut buffer = Vec::new();
        trie.build_cover_streaming(&mut buffer).unwrap();

        assert!(trie.certified_count > 0);
        assert!(trie.certified_measure > BigRational::zero());
        assert!(!buffer.is_empty());
    }
}
