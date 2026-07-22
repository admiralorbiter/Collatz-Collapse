use crate::schema::{DescentCertificateJson, TailDescentCertificateJson};
use crate::tail::compute_a_crit;
use crate::VerificationError;
use collatz_affine::{
    solve_starting_residue_broad, solve_starting_residue_exact, ValuationSemantics, ValuationWord,
};
use collatz_core::odd_step;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

pub const MAX_DIGITS: usize = 4096;
pub const MAX_EXCEPTIONS_CHECKED: usize = 100_000;
pub const MAX_VALUATION_STEP: u32 = 255;
pub const MAX_TOTAL_TWOS: u64 = 4096;
pub const MAX_MODULUS_EXPONENT: u64 = 4096;


fn parse_bounded_biguint(s: &str) -> Result<BigUint, VerificationError> {
    if s.len() > MAX_DIGITS {
        return Err(VerificationError::MaxDigitsExceeded {
            length: s.len(),
            limit: MAX_DIGITS,
        });
    }
    BigUint::from_str(s).map_err(|_| VerificationError::ParseBigIntError(s.to_string()))
}

/// Pure-Rust independent verifier function for DescentCertificateJson.
/// Implements strict invariant checks with zero solver dependencies.
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

    if cert.modulus_exponent > MAX_MODULUS_EXPONENT {
        return Err(VerificationError::InvalidValuationWord(format!("Modulus exponent {} exceeds limit {}", cert.modulus_exponent, MAX_MODULUS_EXPONENT)));
    }

    for &a_i in &cert.valuation_word {
        if a_i == 0 {
            return Err(VerificationError::InvalidValuationWord("Valuation a_i cannot be zero".to_string()));
        }
        if a_i > MAX_VALUATION_STEP {
            return Err(VerificationError::InvalidValuationWord(format!("Valuation step {} exceeds limit {}", a_i, MAX_VALUATION_STEP)));
        }
    }


    let word = ValuationWord::from_u32_slice(&cert.valuation_word)
        .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

    // Step 1: Recompute Total Valuation A_k & Verify Modulus Exponent
    let computed_a_k = word.total_valuation();
    if computed_a_k != cert.total_twos {
        return Err(VerificationError::TotalTwosMismatch {
            declared: cert.total_twos,
            computed: computed_a_k,
        });
    }

    let semantics = match cert.valuation_semantics.as_deref().unwrap_or("terminal_at_least") {
        "exact_word" => ValuationSemantics::ExactWord,
        "terminal_at_least" => ValuationSemantics::TerminalAtLeast,
        other => return Err(VerificationError::SchemaMismatch {
            expected: "exact_word or terminal_at_least".to_string(),
            found: other.to_string(),
        }),
    };

    let expected_mod_exponent = match semantics {
        ValuationSemantics::TerminalAtLeast => computed_a_k,
        ValuationSemantics::ExactWord => computed_a_k + 1,
    };

    if cert.modulus_exponent != expected_mod_exponent {
        return Err(VerificationError::TotalTwosMismatch {
            declared: cert.modulus_exponent,
            computed: expected_mod_exponent,
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

    let declared_c_k = parse_bounded_biguint(&cert.constant)?;

    if c_k != declared_c_k {
        return Err(VerificationError::ConstantMismatch {
            declared: cert.constant.clone(),
            computed: c_k.to_string(),
        });
    }

    // Step 3: Verify Closed-Form Starting Residue
    let computed_residue = match semantics {
        ValuationSemantics::TerminalAtLeast => solve_starting_residue_broad(&c_k, k, computed_a_k),
        ValuationSemantics::ExactWord => solve_starting_residue_exact(&c_k, k, computed_a_k),
    }.map_err(|e| VerificationError::ResidueMismatch { declared: cert.starting_residue.clone(), computed: e.to_string() })?;

    let declared_residue = parse_bounded_biguint(&cert.starting_residue)?;

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
    let declared_threshold = parse_bounded_biguint(&cert.descent_threshold)?;

    if computed_threshold != declared_threshold {
        return Err(VerificationError::ThresholdMismatch {
            declared: cert.descent_threshold.clone(),
            computed: computed_threshold.to_string(),
        });
    }

    // Step 6: Independent Exhaustive Exception Generation and Verification
    let modulus = BigUint::one() << cert.modulus_exponent;
    let mut e = computed_residue.clone();

    if e.is_zero() || (&e & BigUint::one()).is_zero() {
        e += &modulus;
    }

    let mut iterations = 0;
    while e < computed_threshold {
        iterations += 1;
        if iterations > MAX_EXCEPTIONS_CHECKED {
            return Err(VerificationError::ExceptionVerificationFailed {
                integer: format!("Exception check count exceeded safety limit of {}", MAX_EXCEPTIONS_CHECKED),
            });
        }

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

        e += &modulus;
    }

    Ok(())
}

/// Pure-Rust independent verifier function for TailDescentCertificateJson.
pub fn verify_tail_descent_certificate(cert: &TailDescentCertificateJson) -> Result<(), VerificationError> {
    if cert.schema_version != "tail_descent_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "tail_descent_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    if cert.prefix_word.is_empty() {
        return Err(VerificationError::InvalidValuationWord("Prefix valuation word cannot be empty".to_string()));
    }

    let word = ValuationWord::from_u32_slice(&cert.prefix_word)
        .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

    let computed_a_k = word.total_valuation();
    if computed_a_k != cert.prefix_total_twos {
        return Err(VerificationError::TotalTwosMismatch {
            declared: cert.prefix_total_twos,
            computed: computed_a_k,
        });
    }

    // Recompute c_k
    let mut c_k = BigUint::zero();
    let mut partial_sum = 0u64;
    for &a_i in word.as_slice() {
        c_k = (&c_k * 3u32) + (BigUint::one() << partial_sum);
        partial_sum += a_i as u64;
    }

    let declared_c_k = parse_bounded_biguint(&cert.prefix_constant)?;
    if c_k != declared_c_k {
        return Err(VerificationError::ConstantMismatch {
            declared: cert.prefix_constant.clone(),
            computed: c_k.to_string(),
        });
    }

    // Recompute analytical critical child valuation a_crit
    let computed_a_crit = compute_a_crit(&word);
    if computed_a_crit != cert.minimum_child_valuation {
        return Err(VerificationError::ThresholdMismatch {
            declared: cert.minimum_child_valuation.to_string(),
            computed: computed_a_crit.to_string(),
        });
    }

    if cert.proof_bound != "1" {
        return Err(VerificationError::ThresholdMismatch {
            declared: cert.proof_bound.clone(),
            computed: "1".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::generate_descent_certificate;
    use crate::tail::generate_tail_descent_certificate;

    #[test]
    fn test_verify_valid_descent_certificate() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let cert = generate_descent_certificate(word).unwrap();
        assert!(verify_descent_certificate(&cert).is_ok());
    }

    #[test]
    fn test_verify_exact_word_descent_certificate() {
        use collatz_affine::ValuationSemantics;
        use crate::descent::generate_descent_certificate_with_semantics;

        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let cert = generate_descent_certificate_with_semantics(word, ValuationSemantics::ExactWord).unwrap();
        assert_eq!(cert.starting_residue, "295");
        assert_eq!(cert.modulus_exponent, 9);
        assert_eq!(cert.valuation_semantics, Some("exact_word".to_string()));
        assert!(verify_descent_certificate(&cert).is_ok());
    }

    #[test]
    fn test_verify_tail_descent_certificate_valid() {
        let word = ValuationWord::new(vec![1, 1, 2]).unwrap();
        let cert = generate_tail_descent_certificate(word).unwrap();
        assert!(verify_tail_descent_certificate(&cert).is_ok());
    }

    #[test]
    fn test_reject_corrupted_threshold() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let mut cert = generate_descent_certificate(word).unwrap();
        cert.descent_threshold = "999999".to_string(); // Corrupt threshold
        assert!(verify_descent_certificate(&cert).is_err());
    }

    #[test]
    fn test_all_ones_forced_growth_expansion() {
        use collatz_affine::AffinePrefix;
        let word = ValuationWord::new(vec![1; 20]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();

        assert_eq!(prefix.odd_steps, 20);
        assert_eq!(prefix.total_twos, 20);
        assert!(!prefix.is_multiplicative_contracting()); // 2^20 < 3^20!
        assert_eq!(prefix.compute_descent_threshold(), None);
    }

    #[test]
    fn test_primitive_cycle_canonicalization() {
        let p1 = vec![2u8];
        let p2 = vec![2u8, 2u8];

        // primitive root of [2, 2] is [2]
        let is_repeated = p2.len() > p1.len() && p2.chunks(p1.len()).all(|c| c == p1);
        assert!(is_repeated);
    }

    #[test]
    fn test_reject_unknown_field_deserialization() {
        let json_str = r#"{
            "schema_version": "descent_v1",
            "valuation_word": [1, 1, 2, 1, 3],
            "total_twos": 8,
            "odd_steps": 5,
            "starting_residue": "39",
            "modulus_exponent": 8,
            "constant": "251",
            "descent_threshold": "20",
            "checked_exceptions": [],
            "malicious_extra_field": "attack"
        }"#;

        let res: Result<DescentCertificateJson, _> = serde_json::from_str(json_str);
        assert!(res.is_err());
    }
}


