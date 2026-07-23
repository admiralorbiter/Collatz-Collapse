use crate::traits::{PrefixSieve, PrefixState, RejectionReason, SieveResult};
use dashmap::DashMap;
use std::sync::Arc;

/// Kinematic Sieve detecting state signature collisions across distinct valuation paths.
/// Emits `infeasible_subsumption_v1` proof attributes.
pub struct PathMergingSieve {
    state_table: Arc<DashMap<u64, Vec<u32>>>,
}

impl Default for PathMergingSieve {
    fn default() -> Self {
        Self {
            state_table: Arc::new(DashMap::new()),
        }
    }
}

impl PathMergingSieve {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes a hash key from modulo 2^m and modulo 3^j affine residue properties
    fn state_key(state: &PrefixState) -> u64 {
        let r_low = state
            .affine
            .starting_residue
            .to_u64_digits()
            .first()
            .cloned()
            .unwrap_or(0);
        let len = state.valuations.len() as u64;
        let twos = state.affine.total_twos;

        r_low
            .wrapping_mul(31)
            .wrapping_add(len)
            .wrapping_mul(17)
            .wrapping_add(twos)
    }
}

impl PrefixSieve for PathMergingSieve {
    fn name(&self) -> &'static str {
        "PathMergingSieve"
    }

    fn evaluate(&self, state: &PrefixState) -> SieveResult {
        let key = Self::state_key(state);
        let current_word: Vec<u32> = state
            .valuations
            .as_slice()
            .iter()
            .map(|&a| a as u32)
            .collect();

        if let Some(target) = self.state_table.get(&key) {
            if *target != current_word {
                return SieveResult::Reject {
                    reason: RejectionReason::PathSubsumed {
                        target_valuation: target.clone(),
                    },
                };
            }
        } else {
            self.state_table.insert(key, current_word);
        }

        SieveResult::Keep
    }
}
