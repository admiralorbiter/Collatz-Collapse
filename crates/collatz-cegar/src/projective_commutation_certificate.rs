use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::coupled_invariant_miner::CoupledInvariantMiner;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommutationCertificateReport {
    pub target_bits_m: u32,
    pub appended_gap_h: u64,
    pub branch_bits_b_h: u32,
    pub upstream_bits_p: u32,
    pub is_one_step_commutation_verified: bool,
}

pub struct ProjectiveCommutationCertificateEngine;

impl ProjectiveCommutationCertificateEngine {
    /// Verify projective commutation certificate:
    /// \pi_{p \to m}(A_h(D, e)) == T_{h,m}(\pi_{p+B_h}(D, e))
    pub fn verify_one_step_commutation(
        endpoint_d: &BigInt,
        multiplier_q: &BigUint,
        appended_gap_h: u64,
        target_bits_m: u32,
    ) -> CommutationCertificateReport {
        let p_h = AcceleratedBranchParams::for_gap(appended_gap_h);
        let branch_bits_b_h = p_h.precision as u32;
        let upstream_bits_p = target_bits_m + branch_bits_b_h;

        let d_biguint = endpoint_d.to_biguint().unwrap_or_default();

        // 1. Exact canonical extension A_h(D, Q)
        let (d_next_biguint, _) = CoupledInvariantMiner::canonical_extension(
            &d_biguint,
            multiplier_q,
            appended_gap_h,
        );

        // Project D' to m bits
        let m_mod = BigUint::from(2u64).pow(target_bits_m);
        let proj1 = &d_next_biguint % &m_mod;

        // 2. Project input D to (m + B_h) bits first, then extend and project to m bits
        let p_mod = BigUint::from(2u64).pow(upstream_bits_p);
        let d_trunc = &d_biguint % &p_mod;

        let (d_next_trunc, _) = CoupledInvariantMiner::canonical_extension(
            &d_trunc,
            multiplier_q,
            appended_gap_h,
        );
        let proj2 = &d_next_trunc % &m_mod;

        let is_one_step_commutation_verified = proj1 == proj2;

        CommutationCertificateReport {
            target_bits_m,
            appended_gap_h,
            branch_bits_b_h,
            upstream_bits_p,
            is_one_step_commutation_verified,
        }
    }

    /// Verify arbitrary word projective commutation certificate:
    /// \pi_m \circ A_{h_1 \dots h_d}(D, e) == T_{h_1 \dots h_d, m} \circ \pi_{m + \sum B_{h_i}}(D, e)
    pub fn verify_arbitrary_word_commutation(
        endpoint_d: &BigInt,
        multiplier_q: &BigUint,
        word: &[u64],
        target_bits_m: u32,
    ) -> bool {
        assert!(!word.is_empty());
        let mut total_branch_bits = 0u32;
        for &h in word {
            let p_h = AcceleratedBranchParams::for_gap(h);
            total_branch_bits += p_h.precision as u32;
        }

        let upstream_bits_p = target_bits_m + total_branch_bits;
        let d_biguint = endpoint_d.to_biguint().unwrap_or_default();

        // 1. Full word evaluation A_u(D, Q)
        let mut d_curr = d_biguint.clone();
        let mut q_curr = multiplier_q.clone();

        for &h in word {
            let (d_next, q_next) = CoupledInvariantMiner::canonical_extension(&d_curr, &q_curr, h);
            d_curr = d_next;
            q_curr = q_next;
        }

        let m_mod = BigUint::from(2u64).pow(target_bits_m);
        let proj1 = &d_curr % &m_mod;

        // 2. Truncated upstream input \pi_{m + \sum B_{h_i}}(D)
        let p_mod = BigUint::from(2u64).pow(upstream_bits_p);
        let mut d_trunc_curr = &d_biguint % &p_mod;
        let mut q_trunc_curr = multiplier_q.clone();

        for &h in word {
            let (d_next, q_next) = CoupledInvariantMiner::canonical_extension(&d_trunc_curr, &q_trunc_curr, h);
            d_trunc_curr = d_next;
            q_trunc_curr = q_next;
        }

        let proj2 = &d_trunc_curr % &m_mod;

        proj1 == proj2
    }
}
