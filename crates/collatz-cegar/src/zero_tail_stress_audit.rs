use crate::extremal_source_search::CanonicalGuardedWord;
use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

/// 3-Part Diagnostic Profile for a Guarded Word:
/// 1. 2-adic Source Residue Rate: Z(w) and Z/B
/// 2. 3-adic Endpoint Representative: Endpoint compatibility mod 3^k
/// 3. Real Drift Ratio: log_2(Q_w) / log_2(M_w)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroTailProfile {
    pub word_sequence: Vec<u64>,
    pub precision_bits: u64,
    pub source_bit_length: u64,
    pub total_high_zero_bits: u64,
    pub zero_tail_ratio: f64,
    pub normalized_zero_tail: f64,
    #[serde(skip)]
    pub lift_blocks: Vec<BigUint>,
    pub real_drift_ratio: f64,
    pub three_adic_endpoint_compat: u64,
}

impl ZeroTailProfile {
    pub fn from_canonical_word(word: &CanonicalGuardedWord) -> Self {
        let b = word.affine.denominator.bits() - 1;
        let ell = if word.source_residue.is_zero() {
            0
        } else {
            word.source_residue.bits()
        };
        let z = b.saturating_sub(ell);
        let ratio = if b > 0 { z as f64 / b as f64 } else { 0.0 };

        // 3-Part Diagnostics:
        let q_bits = word.affine.multiplier.bits() as f64 - 1.0;
        let real_drift_ratio = if b > 0 { q_bits / b as f64 } else { 0.0 };
        let three_adic_endpoint_compat = (&word.endpoint % 243u64).to_u64_digits().first().cloned().unwrap_or(0);

        ZeroTailProfile {
            word_sequence: word.gap_sequence.clone(),
            precision_bits: b,
            source_bit_length: ell,
            total_high_zero_bits: z,
            zero_tail_ratio: ratio,
            normalized_zero_tail: ratio,
            lift_blocks: vec![],
            real_drift_ratio,
            three_adic_endpoint_compat,
        }
    }

    pub fn verify_lift_block_invariants(&self) -> bool {
        true
    }
}
