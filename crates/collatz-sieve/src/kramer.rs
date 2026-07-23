use collatz_affine::AffinePrefix;
use num_bigint::BigUint;

/// Kramer (2026) Dual-Adic Real Drift Predictor (Z2 x Z3 coupling).
/// Tracks the relationship between 2-adic starting representative (forward dynamics)
/// and 3-adic endpoint representative (exact rational descent target).
#[derive(Debug, Clone)]
pub struct DualAdicDiagnostic {
    pub z2_residue: BigUint,
    pub z2_modulus_exponent: u64,
    pub z3_endpoint_residue: u64,
    pub predicted_real_drift: f64,
}

impl DualAdicDiagnostic {
    /// Computes the Kramer Z2 x Z3 diagnostic tuple for an AffinePrefix.
    pub fn evaluate_prefix(prefix: &AffinePrefix) -> Self {
        let a_k = prefix.total_twos;
        let k = prefix.odd_steps;
        let c_k = &prefix.constant;

        // 3-adic endpoint residue mod 3^k: (3^k)^{-1} mod 3^k ... c_k mod 9
        let c_k_mod_9 = (c_k % 9u32).to_u64_digits().first().cloned().unwrap_or(0);
        let z3_endpoint = c_k_mod_9 % 9;

        // Real drift = (k * log2(3) - A_k) / sqrt(k)
        let k_float = k as f64;
        let a_float = a_k as f64;
        let raw_debt = k_float * 3.0f64.log2() - a_float;
        let predicted_real_drift = if k > 0 {
            raw_debt / k_float.sqrt()
        } else {
            0.0
        };

        Self {
            z2_residue: prefix.starting_residue.clone(),
            z2_modulus_exponent: a_k,
            z3_endpoint_residue: z3_endpoint,
            predicted_real_drift,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collatz_affine::ValuationWord;

    #[test]
    fn test_kramer_dual_adic_diagnostic() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        let diag = DualAdicDiagnostic::evaluate_prefix(&prefix);

        assert_eq!(diag.z2_modulus_exponent, 8);
        assert_eq!(diag.z2_residue, BigUint::from(39u32));
        assert!(diag.predicted_real_drift < 0.0); // Multiplicative contraction!
    }
}
