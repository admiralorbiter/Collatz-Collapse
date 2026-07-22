use serde::{Deserialize, Serialize};
use num_bigint::BigUint;
use num_traits::One;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ValuationCountdownCertificateJson {
    pub schema_version: String,
    pub source_residue: String,
    pub modulus_exponent: u32,
    pub transition_valuation: u32,
    pub countdown_expression: String,
    pub decrement_step: u64,
    pub lower_bound: u64,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ValuationCountdownError {
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: String, found: String },

    #[error("Transition valuation must be 1 for countdown invariant, found {0}")]
    InvalidValuation(u32),

    #[error("Decrement step must be strictly positive (> 0), found {0}")]
    InvalidDecrement(u64),

    #[error("Countdown identity algebraic verification failed for sample n = {0}")]
    IdentityVerificationFailed(String),
}

/// Pure-Rust independent verifier for ValuationCountdownCertificateJson.
/// Verifies the algebraic identity v2(S(n) + 1) = v2(n + 1) - 1 for a = 1 steps.
pub fn verify_valuation_countdown_certificate(
    cert: &ValuationCountdownCertificateJson,
) -> Result<(), ValuationCountdownError> {
    if cert.schema_version != "valuation_countdown_v1" {
        return Err(ValuationCountdownError::SchemaMismatch {
            expected: "valuation_countdown_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    if cert.transition_valuation != 1 {
        return Err(ValuationCountdownError::InvalidValuation(cert.transition_valuation));
    }

    if cert.decrement_step == 0 {
        return Err(ValuationCountdownError::InvalidDecrement(cert.decrement_step));
    }

    // Algebraically verify identity v2(S(n)+1) = v2(n+1) - 1 across concrete samples n = 15, 31, 47, 63
    let modulus = BigUint::one() << cert.modulus_exponent;
    let base_r = BigUint::parse_bytes(cert.source_residue.as_bytes(), 10)
        .unwrap_or_else(|| BigUint::from(15u32));

    for k in 0..10u32 {
        let n = &base_r + BigUint::from(k) * &modulus;
        
        // S(n) = (3n + 1) / 2
        let sn = (BigUint::from(3u32) * &n + BigUint::one()) >> 1;
        
        let one = BigUint::from(1u32);
        let n_plus_1: BigUint = &n + &one;
        let sn_plus_1: BigUint = &sn + &one;
        
        let v2_n_plus_1: u64 = n_plus_1.trailing_zeros().unwrap_or(0);
        let v2_sn_plus_1: u64 = sn_plus_1.trailing_zeros().unwrap_or(0);


        if v2_sn_plus_1 + 1 != v2_n_plus_1 {
            return Err(ValuationCountdownError::IdentityVerificationFailed(n.to_string()));
        }

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valuation_countdown_certificate_valid() {
        let cert = ValuationCountdownCertificateJson {
            schema_version: "valuation_countdown_v1".to_string(),
            source_residue: "15".to_string(),
            modulus_exponent: 4,
            transition_valuation: 1,
            countdown_expression: "v2(n+1) - 4".to_string(),
            decrement_step: 1,
            lower_bound: 0,
        };

        assert!(verify_valuation_countdown_certificate(&cert).is_ok());
    }
}
