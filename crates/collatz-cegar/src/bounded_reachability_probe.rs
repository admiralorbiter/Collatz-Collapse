use crate::cylinder_trie_reduction::CylinderTrie;
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

/// Outcome Classification for Bounded Reachability Probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundedReachabilityOutcome {
    BoundedPrefixZeroTailExcluded {
        prefix_max_len: usize,
        j_pre_max: u64,
        j_tail_max: u64,
        first_empty_depth: usize,
    },
    ExactEventuallyPeriodicTemplateFound,
    BranchingInverseLimitCandidate,
    CylinderComplexityGrowthWithoutStabilization,
}

/// JSON Replay Certificate Witness for Non-Empty Intersections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectTailWitnessJson {
    pub prefix_sequence: Vec<u64>,
    pub endpoint_d_u: String,
    pub zero_lift_tail: Vec<u64>,
    pub intermediate_endpoints: Vec<String>,
    pub congruences_satisfied: Vec<bool>,
}

/// Phase 7.3S.2B: Bounded Canonical Reachability Probe & Replay Witness Generator.
pub struct BoundedReachabilityProbe {
    pub prefix_max_len: usize,
    pub j_pre_max: u64,
    pub j_tail_max: u64,
}

impl BoundedReachabilityProbe {
    pub fn new(prefix_max_len: usize, j_pre_max: u64, j_tail_max: u64) -> Self {
        Self {
            prefix_max_len,
            j_pre_max,
            j_tail_max,
        }
    }

    /// Enumerate non-empty canonical initial endpoints I_{U, J_pre} = { D_u : 1 <= |u| <= U, j_i <= J_pre }
    pub fn generate_initial_endpoints(&self) -> Vec<(Vec<u64>, BigUint)> {
        let mut endpoints = Vec::new();
        let mut current_prefixes: Vec<(Vec<u64>, _)> = Vec::new();

        for j in 0..=self.j_pre_max {
            let word = ExtremalSourceSearchEngine::base_guarded_word(j);
            endpoints.push((vec![j], word.endpoint.clone()));
            current_prefixes.push((vec![j], word));
        }

        for _depth in 2..=self.prefix_max_len {
            let mut next_prefixes = Vec::new();
            for (seq, parent_word) in &current_prefixes {
                for j in 0..=self.j_pre_max {
                    let child_word = ExtremalSourceSearchEngine::extend_guarded_word(parent_word, j);
                    let mut child_seq = seq.clone();
                    child_seq.push(j);
                    endpoints.push((child_seq.clone(), child_word.endpoint.clone()));
                    next_prefixes.push((child_seq, child_word));
                }
            }
            current_prefixes = next_prefixes;
        }

        endpoints
    }

    /// Evaluate intersection I_{U, J_pre} \cap E_n and generate DirectTailWitnessJson if non-empty
    pub fn evaluate_intersection(
        &self,
        e_n: &CylinderTrie,
        tail_depth: usize,
    ) -> (BoundedReachabilityOutcome, Vec<DirectTailWitnessJson>) {
        let initial_endpoints = self.generate_initial_endpoints();
        let mut witnesses = Vec::new();

        for (seq_u, d_u) in &initial_endpoints {
            if e_n.contains_endpoint(d_u) {
                let mut curr_endpoint = d_u.clone();
                let mut intermediate = vec![curr_endpoint.to_string()];
                let mut congruences = Vec::new();

                let sample_tail = e_n
                    .to_cylinders()
                    .into_iter()
                    .find(|cyl| {
                        let mod_p = BigUint::from(1u64) << cyl.precision;
                        (d_u % &mod_p) == cyl.residue
                    })
                    .and_then(|cyl| e_n.provenance.get(&cyl).and_then(|seqs| seqs.first().cloned()))
                    .unwrap_or_else(|| vec![0; tail_depth]);

                for &j in &sample_tail {
                    let p_j = ExtremalSourceSearchEngine::base_guarded_word(j);
                    let is_cong = (&curr_endpoint % &p_j.affine.denominator) == (p_j.source_residue % &p_j.affine.denominator);
                    congruences.push(is_cong);

                    if is_cong {
                        let num = p_j.affine.multiplier * &curr_endpoint + p_j.affine.intercept.to_biguint().unwrap();
                        curr_endpoint = num / p_j.affine.denominator;
                        intermediate.push(curr_endpoint.to_string());
                    } else {
                        break;
                    }
                }

                witnesses.push(DirectTailWitnessJson {
                    prefix_sequence: seq_u.clone(),
                    endpoint_d_u: d_u.to_string(),
                    zero_lift_tail: sample_tail,
                    intermediate_endpoints: intermediate,
                    congruences_satisfied: congruences,
                });
            }
        }

        if witnesses.is_empty() {
            (
                BoundedReachabilityOutcome::BoundedPrefixZeroTailExcluded {
                    prefix_max_len: self.prefix_max_len,
                    j_pre_max: self.j_pre_max,
                    j_tail_max: self.j_tail_max,
                    first_empty_depth: tail_depth,
                },
                witnesses,
            )
        } else {
            (
                BoundedReachabilityOutcome::BranchingInverseLimitCandidate,
                witnesses,
            )
        }
    }
}
