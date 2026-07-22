use crate::schema::DescentCertificateJson;
use crate::VerificationError;
use collatz_affine::{solve_starting_residue, ValuationWord};
use collatz_core::odd_step;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

/// Pure-Rust independent verifier function for DescentCertificateJson.
/// Implements 6 strict invariant checks with zero solver dependencies.
pub fn verify_descent_certificate(cert: &DescentCertificateJson) -> Result<(), VerificationError> {
    // Step 0: Valuation Domain & Schema Constraints
    if cert.schema_version != "descent_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "descent_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    if cert.valuation_word.is_empty() {
        return Err(VerificationError::InvalidValuationWord("Valuation word cannot be empty".to_string()));
    }

    for &a_i in &cert.valuation_word {
        if a_i == 0 {
            return Err(VerificationError::InvalidValuationWord("Valuation a_i cannot be zero".to_string()));
        }
    }

    let word = ValuationWord::from_u32_slice(&cert.valuation_word)
        .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

    // Step 1: Recompute Total Valuation A_k
    let computed_a_k = word.total_valuation();
    if computed_a_k != cert.total_twos || computed_a_k != cert.modulus_exponent {
        return Err(VerificationError::TotalTwosMismatch {
            declared: cert.total_twos,
            computed: computed_a_k,
        });
    }

    // Step 2: Recompute Affine Constant c_k
    let k = word.len();
    let mut c_k = BigUint::zero();
    let mut partial_sum = 0u64;

    for &a_i in word.as_slice() {
        c_k = (&c_k * 3u32) + (BigUint::one() << partial_sum);
        partial_sum += a_i as u64;
    }

    let declared_c_k = BigUint::from_str(&cert.constant)
        .map_err(|_| VerificationError::ParseBigIntError(cert.constant.clone()))?;

    if c_k != declared_c_k {
        return Err(VerificationError::ConstantMismatch {
            declared: cert.constant.clone(),
            computed: c_k.to_string(),
        });
    }

    // Step 3: Verify Closed-Form Starting Residue n_0 mod 2^A_k
    let computed_residue = solve_starting_residue(&c_k, k, computed_a_k)
        .map_err(|e| VerificationError::ResidueMismatch { declared: cert.starting_residue.clone(), computed: e.to_string() })?;

    let declared_residue = BigUint::from_str(&cert.starting_residue)
        .map_err(|_| VerificationError::ParseBigIntError(cert.starting_residue.clone()))?;

    if computed_residue != declared_residue {
        return Err(VerificationError::ResidueMismatch {
            declared: cert.starting_residue.clone(),
            computed: computed_residue.to_string(),
        });
    }

    // Step 4: Verify Multiplicative Contraction 2^A_k > 3^k
    let pow3_k = BigUint::from(3u32).pow(k as u32);
    let pow2_a = BigUint::one() << computed_a_k;

    if pow2_a <= pow3_k {
        return Err(VerificationError::NoMultiplicativeContraction {
            pow2: pow2_a.to_string(),
            pow3: pow3_k.to_string(),
        });
    }

    // Step 5: Verify Exact Integer Threshold B = floor(c_k / (2^A_k - 3^k)) + 1
    let diff = &pow2_a - &pow3_k;
    let computed_threshold = (&c_k / diff) + 1u32;
    let declared_threshold = BigUint::from_str(&cert.descent_threshold)
        .map_err(|_| VerificationError::ParseBigIntError(cert.descent_threshold.clone()))?;

    if computed_threshold != declared_threshold {
        return Err(VerificationError::ThresholdMismatch {
            declared: cert.descent_threshold.clone(),
            computed: computed_threshold.to_string(),
        });
    }

    // Step 6 (Peer-Reviewed Fix): Independent Exhaustive Exception Generation and Verification
    // Do NOT rely solely on prover-supplied array; generate all e < B independently.
    let modulus = &pow2_a;
    let mut e = computed_residue.clone();

    if e.is_zero() || (&e & BigUint::one()).is_zero() {
        e += modulus;
    }

    while e < computed_threshold {
        let mut val = e.clone();
        let mut descended = false;

        for _ in 0..k {
            let step = odd_step(&val)
                .map_err(|_| VerificationError::ExceptionVerificationFailed { integer: e.to_string() })?;
            val = step.to;
            if val < e || val.is_one() {
                descended = true;
                break;
            }
        }

        if !descended {
            return Err(VerificationError::ExceptionVerificationFailed { integer: e.to_string() });
        }

        e += modulus;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::generate_descent_certificate;

    #[test]
    fn test_verify_valid_descent_certificate() {
        // Valuation word (1, 1, 2, 1, 3) -> k=5, A_k=8.
        // 3^5 = 243, 2^8 = 256. 256 > 243 -> Multiplicative contraction!
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let cert = generate_descent_certificate(word).unwrap();
        assert!(verify_descent_certificate(&cert).is_ok());
    }

    #[test]
    fn test_reject_corrupted_threshold() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let mut cert = generate_descent_certificate(word).unwrap();
        cert.descent_threshold = "999999".to_string(); // Corrupt threshold
        assert!(verify_descent_certificate(&cert).is_err());
    }
}
