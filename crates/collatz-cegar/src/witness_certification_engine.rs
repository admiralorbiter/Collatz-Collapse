use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::coupled_invariant_miner::CoupledInvariantMiner;
use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::spine_quotient_oracle::{GlobalGuardResult, SpineQuotientOracle};
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone)]
pub struct WitnessCertificate {
    pub word: Vec<u64>,
    pub depth: usize,
    pub endpoint_d_u: BigInt,
    pub multiplier_q_u: BigUint,
    pub first_gap_j: u64,
    pub n_u: BigInt,
    pub successor_d_1: BigInt,
    pub second_gap_k: u64,
    pub n_1: BigInt,
    pub successor_d_2: BigInt,
    pub is_triple_zero: bool,
    pub third_gap_l: Option<u64>,
}

pub struct WitnessCertificationEngine;

impl WitnessCertificationEngine {
    /// Evaluate word to compute exact endpoint D_u and multiplier Q_u
    pub fn compute_word_endpoint_and_multiplier(word: &[u64]) -> (BigInt, BigUint) {
        assert!(!word.is_empty(), "Word cannot be empty");
        let w0 = ExtremalSourceSearchEngine::base_guarded_word(word[0]);
        let mut endpoint_big = BigInt::from(w0.endpoint.clone());
        let mut q_u = w0.affine.multiplier.clone();

        for &h in &word[1..] {
            let (d_next, q_next) = CoupledInvariantMiner::canonical_extension(
                &endpoint_big.to_biguint().unwrap_or_default(),
                &q_u,
                h,
            );
            endpoint_big = BigInt::from(d_next);
            q_u = q_next;
        }

        (endpoint_big, q_u)
    }

    /// Certify witness word through direct arithmetic, quotient classification, and E_3 test
    pub fn certify_witness(word: &[u64]) -> Option<WitnessCertificate> {
        let (endpoint_d_u, multiplier_q_u) = Self::compute_word_endpoint_and_multiplier(word);

        // 1. First zero guard test (E_1)
        let (first_gap_j, c_j_source) = match SpineQuotientOracle::classify_global_zero_guard(&endpoint_d_u) {
            GlobalGuardResult::FirstZeroGuardFound { gap_j, source_residue, .. } => (gap_j, source_residue),
            GlobalGuardResult::NoFirstZero => return None,
        };

        // Compute n_u = (D_u - C_j) / M_j
        let p_j = AcceleratedBranchParams::for_gap(first_gap_j);
        let m_j_big = BigInt::from(p_j.modulus.clone());
        let c_j_big = BigInt::from(c_j_source);
        let n_u = (&endpoint_d_u - &c_j_big) / &m_j_big;

        // Compute first successor D^{(1)} = D_j + Q_j * n_u
        let d_j_endpoint = BigInt::from(p_j.z_endpoint.clone());
        let q_j_big = BigInt::from(p_j.multiplier.clone());
        let successor_d_1 = &d_j_endpoint + &q_j_big * &n_u;

        // 2. Second zero guard test (E_2)
        let (second_gap_k, c_k_source) = match SpineQuotientOracle::classify_global_zero_guard(&successor_d_1) {
            GlobalGuardResult::FirstZeroGuardFound { gap_j, source_residue, .. } => (gap_j, source_residue),
            GlobalGuardResult::NoFirstZero => return None,
        };

        // Compute n^{(1)} = (D^{(1)} - C_k) / M_k
        let p_k = AcceleratedBranchParams::for_gap(second_gap_k);
        let m_k_big = BigInt::from(p_k.modulus.clone());
        let c_k_big = BigInt::from(c_k_source);
        let n_1 = (&successor_d_1 - &c_k_big) / &m_k_big;

        // Compute second successor D^{(2)} = D_k + Q_k * n^{(1)}
        let d_k_endpoint = BigInt::from(p_k.z_endpoint.clone());
        let q_k_big = BigInt::from(p_k.multiplier.clone());
        let successor_d_2 = &d_k_endpoint + &q_k_big * &n_1;

        // 3. Third zero guard test (E_3)
        let (is_triple_zero, third_gap_l) = match SpineQuotientOracle::classify_global_zero_guard(&successor_d_2) {
            GlobalGuardResult::FirstZeroGuardFound { gap_j, .. } => (true, Some(gap_j)),
            GlobalGuardResult::NoFirstZero => (false, None),
        };

        Some(WitnessCertificate {
            word: word.to_vec(),
            depth: word.len(),
            endpoint_d_u,
            multiplier_q_u,
            first_gap_j,
            n_u,
            successor_d_1,
            second_gap_k,
            n_1,
            successor_d_2,
            is_triple_zero,
            third_gap_l,
        })
    }
}
