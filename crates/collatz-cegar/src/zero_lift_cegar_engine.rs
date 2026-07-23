use crate::accelerated_branch_params::AcceleratedBranchParams;
use std::collections::{HashMap, HashSet};

/// Phase 7.3S.2A Zero-Lift Endpoint CEGAR Engine & Redesign Classifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CegarOutcome {
    NoReachableZeroCycleCertified,
    AllRecurrentZeroPathsEventuallyPeriodic,
    BranchingZeroOutputSccIsolated,
    QuotientRefinementDoesNotStabilize,
    AbstractionRedesignRequired,
}

#[derive(Debug, Clone)]
pub struct ZeroLiftCegarEngine {
    pub refinement_bits: u32,
    pub max_gap: u64,
}

impl ZeroLiftCegarEngine {
    pub fn new(refinement_bits: u32, max_gap: u64) -> Self {
        Self {
            refinement_bits,
            max_gap,
        }
    }

    /// Run CEGAR probe. Simple modulo-2^m overapproximations collapse quotient bits.
    /// Returns AbstractionRedesignRequired until Precision-Aware Cylinder abstraction is implemented.
    pub fn run_cegar_probe(&self) -> (CegarOutcome, usize, usize) {
        let modulus_m = 1u64 << self.refinement_bits;
        let mut adj: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for d_rem in 0..modulus_m {
            nodes.insert(d_rem);

            for j in 0..=self.max_gap {
                let p_j = AcceleratedBranchParams::for_gap(j);
                let c_j_rem = (&p_j.z_source_residue % modulus_m).to_u64_digits().first().cloned().unwrap_or(0);
                let d_j_rem = (&p_j.z_endpoint % modulus_m).to_u64_digits().first().cloned().unwrap_or(0);

                if d_rem == c_j_rem {
                    let next_rem = d_j_rem;
                    adj.entry(d_rem).or_default().push((next_rem, j));
                    nodes.insert(next_rem);
                }
            }
        }

        let num_nodes = nodes.len();
        let num_edges: usize = adj.values().map(|v| v.len()).sum();

        (CegarOutcome::AbstractionRedesignRequired, num_nodes, num_edges)
    }
}
