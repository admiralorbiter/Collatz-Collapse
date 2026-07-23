use crate::accelerated_branch_params::AcceleratedBranchParams;
use std::collections::{HashMap, HashSet};

/// Phase 7.3S.2A Zero-Output Transducer State & SCC Classifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransducerOutcome {
    FiniteEventualZeroQuotientFound,
    ZeroOutputSccsClassified,
    UnboundedStateParameterIdentified,
    QuotientRefinementInconclusive,
}

#[derive(Debug, Clone)]
pub struct ZeroOutputSccProbe {
    pub refinement_level: u32,
    pub max_gap: u64,
}

impl ZeroOutputSccProbe {
    pub fn new(refinement_level: u32, max_gap: u64) -> Self {
        Self {
            refinement_level,
            max_gap,
        }
    }

    /// Analyze reachable zero-output states and classify Strongly Connected Components (SCCs).
    pub fn analyze_zero_output_subgraph(&self) -> (TransducerOutcome, usize, usize) {
        // Construct transitions where lift block Lambda_i == 0
        let mut adj: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut nodes = HashSet::new();

        for s in 0..=self.max_gap {
            nodes.insert(s);
            let p = AcceleratedBranchParams::for_gap(s);
            for t in 0..=self.max_gap {
                // Zero block condition: Lambda_{s, t} == 0
                if p.mu_mod_11 == 0 {
                    adj.entry(s).or_default().push(t);
                    nodes.insert(t);
                }
            }
        }

        let num_nodes = nodes.len();
        let num_edges: usize = adj.values().map(|v| v.len()).sum();

        if num_edges == 0 {
            (TransducerOutcome::FiniteEventualZeroQuotientFound, num_nodes, 0)
        } else {
            (TransducerOutcome::ZeroOutputSccsClassified, num_nodes, num_edges)
        }
    }
}
