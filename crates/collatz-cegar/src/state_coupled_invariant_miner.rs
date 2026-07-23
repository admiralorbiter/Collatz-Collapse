use crate::two_zero_cylinder_characterization::{DoubleZeroStatus, TwoZeroCylinderCharacterization};
use num_bigint::BigUint;
use std::collections::HashSet;

/// Phase 7.3S.2C.4: State-Coupled Invariant Miner (D_u mod 2^m, Q_u mod 2^m).
#[derive(Debug, Clone)]
pub struct StateCoupledInvariantMiner {
    pub modulus_bits: u32,
}

impl StateCoupledInvariantMiner {
    pub fn new(modulus_bits: u32) -> Self {
        Self { modulus_bits }
    }

    /// Mine state pairs (D_u mod 2^m, Q_u mod 2^m) that separate canonical endpoints from E_2^32
    pub fn mine_invariant(&self, endpoints: &[(BigUint, BigUint)]) -> HashSet<(u64, u64)> {
        let mod_val = 1u64 << self.modulus_bits;
        let mut safe_states = HashSet::new();
        let char_engine = TwoZeroCylinderCharacterization::new(32);

        for (d_u, q_u) in endpoints {
            let status = char_engine.evaluate_endpoint_status(d_u);
            if !matches!(status, DoubleZeroStatus::AtLeastTwoZeroLifts { .. }) {
                let d_rem = (d_u % mod_val).to_u64_digits().get(0).cloned().unwrap_or(0);
                let q_rem = (q_u % mod_val).to_u64_digits().get(0).cloned().unwrap_or(0);
                safe_states.insert((d_rem, q_rem));
            }
        }

        safe_states
    }
}
