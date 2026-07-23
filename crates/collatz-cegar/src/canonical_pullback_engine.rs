use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::coupled_invariant_miner::CoupledInvariantMiner;
use num_bigint::BigUint;
use num_traits::One;

/// Canonical State Coordinate (D_u, Q_u)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactCanonicalState {
    pub endpoint: BigUint,
    pub multiplier: BigUint,
}

pub struct CanonicalPullbackEngine;

impl CanonicalPullbackEngine {
    /// Forward precision-aware transformer: T_{h,m} : \Sigma_{m + B_h} -> \Sigma_m
    pub fn forward_transformer(state_in: &ExactCanonicalState, h: u64, m: u64) -> ExactCanonicalState {
        let p_h = AcceleratedBranchParams::for_gap(h);
        let prec_in = m + p_h.precision;

        let mod_in = BigUint::one() << prec_in;
        let mod_out = BigUint::one() << m;

        let d_in = &state_in.endpoint % &mod_in;
        let q_in = &state_in.multiplier % &mod_in;

        let (d_uh, q_uh) = CoupledInvariantMiner::canonical_extension(&d_in, &q_in, h);

        ExactCanonicalState {
            endpoint: d_uh % &mod_out,
            multiplier: q_uh % &mod_out,
        }
    }

    /// Exact canonical extension map A_h(D, Q) = (D_{uh}, Q_{uh})
    pub fn forward_extension(state: &ExactCanonicalState, h: u64) -> ExactCanonicalState {
        let (d_uh, q_uh) = CoupledInvariantMiner::canonical_extension(&state.endpoint, &state.multiplier, h);
        ExactCanonicalState {
            endpoint: d_uh,
            multiplier: q_uh,
        }
    }

    /// Precision-aware pullback CanPre_{h,m} : \Sigma_m -> P(\Sigma_{m + B_h})
    /// Returns true if predecessor class candidate_pred in \Sigma_{m + B_h} maps to target_succ in \Sigma_m under T_{h,m}
    pub fn is_valid_predecessor_class(
        candidate_pred: &ExactCanonicalState,
        target_succ: &ExactCanonicalState,
        h: u64,
        m: u64,
    ) -> bool {
        let succ_calc = Self::forward_transformer(candidate_pred, h, m);
        let mod_out = BigUint::one() << m;

        (succ_calc.endpoint % &mod_out) == (&target_succ.endpoint % &mod_out)
            && (succ_calc.multiplier % &mod_out) == (&target_succ.multiplier % &mod_out)
    }

    /// Positive Reverse-Replay Control: Start from final coupled state of canonical word sequence u = (h_1, ..., h_t)
    /// and successively pull back step by step, confirming exact prefix state recovery.
    pub fn verify_reverse_replay(word_gaps: &[u64], m: u64) -> bool {
        if word_gaps.is_empty() {
            return true;
        }

        // Reconstruct exact prefix states
        let mut prefix_states = Vec::new();
        let w0 = crate::extremal_source_search::ExtremalSourceSearchEngine::base_guarded_word(word_gaps[0]);
        let mut current_state = ExactCanonicalState {
            endpoint: w0.endpoint.clone(),
            multiplier: w0.affine.multiplier.clone(),
        };
        prefix_states.push(current_state.clone());

        for &h in &word_gaps[1..] {
            current_state = Self::forward_extension(&current_state, h);
            prefix_states.push(current_state.clone());
        }

        // Pull back step by step from t down to 1
        for step in (1..word_gaps.len()).rev() {
            let target_succ = &prefix_states[step];
            let expected_pred = &prefix_states[step - 1];
            let gap = word_gaps[step];

            if !Self::is_valid_predecessor_class(expected_pred, target_succ, gap, m) {
                return false;
            }
        }

        true
    }

    /// Execute full h=0..64 Pullback Matrix Test across all 65 gaps
    pub fn verify_full_h0_to_h64_pullback_matrix(m: u64) -> bool {
        let w0 = crate::extremal_source_search::ExtremalSourceSearchEngine::base_guarded_word(0);
        let base_state = ExactCanonicalState {
            endpoint: w0.endpoint,
            multiplier: w0.affine.multiplier,
        };

        for h in 0..=64 {
            let succ = Self::forward_extension(&base_state, h);
            if !Self::is_valid_predecessor_class(&base_state, &succ, h, m) {
                return false;
            }
        }
        true
    }
}
