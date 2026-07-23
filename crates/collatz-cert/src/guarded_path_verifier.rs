use crate::schema::GuardedPathCertificateJson;
use crate::VerificationError;
use collatz_affine::{AffinePrefix, ValuationWord};
use num_bigint::BigUint;
use num_traits::One;
use std::str::FromStr;

pub fn verify_guarded_path_certificate(
    cert_json: &str,
) -> Result<GuardedPathCertificateJson, VerificationError> {
    let cert: GuardedPathCertificateJson =
        serde_json::from_str(cert_json).map_err(|e| VerificationError::JsonError(e.to_string()))?;

    if cert.schema_version != "guarded_path_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "guarded_path_v1".to_string(),
            found: cert.schema_version.clone(),
        });
    }

    if cert.execution_semantics != "left_to_right_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "left_to_right_v1".to_string(),
            found: cert.execution_semantics.clone(),
        });
    }

    // 1. Flatten valuation words left-to-right
    let mut computed_flattened = Vec::new();
    for step in &cert.steps {
        computed_flattened.extend_from_slice(&step.valuation_word);
    }

    if computed_flattened != cert.flattened_valuation_word {
        return Err(VerificationError::InvalidValuationWord(
            "Flattened valuation word mismatch".to_string(),
        ));
    }

    // 2. Recompute composite affine transform
    let val_word = ValuationWord::from_u32_slice(&computed_flattened)
        .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
    let composite_prefix = AffinePrefix::from_valuation_word(val_word)
        .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

    let k = computed_flattened.len() as u32;
    let pow3_k = BigUint::from(3u32).pow(k);
    let expected_mult = BigUint::from_str(&cert.composite_multiplier)
        .map_err(|e| VerificationError::ParseBigIntError(e.to_string()))?;
    if pow3_k != expected_mult {
        return Err(VerificationError::ConstantMismatch {
            declared: cert.composite_multiplier.clone(),
            computed: pow3_k.to_string(),
        });
    }

    let expected_constant = BigUint::from_str(&cert.composite_constant)
        .map_err(|e| VerificationError::ParseBigIntError(e.to_string()))?;
    if composite_prefix.constant != expected_constant {
        return Err(VerificationError::ConstantMismatch {
            declared: cert.composite_constant.clone(),
            computed: composite_prefix.constant.to_string(),
        });
    }

    let denom = BigUint::one() << composite_prefix.total_twos;
    let expected_denom = BigUint::from_str(&cert.composite_denominator)
        .map_err(|e| VerificationError::ParseBigIntError(e.to_string()))?;
    if denom != expected_denom {
        return Err(VerificationError::TotalTwosMismatch {
            declared: expected_denom.to_u64_digits().first().cloned().unwrap_or(0),
            computed: composite_prefix.total_twos,
        });
    }

    // 3. Recompute and verify guarded path cylinder
    let base_residue = BigUint::from_str(&cert.base_state_residue)
        .map_err(|e| VerificationError::ParseBigIntError(e.to_string()))?;
    let base_mod = BigUint::one() << cert.base_state_modulus_exponent;

    let path_residue = BigUint::from_str(&cert.path_source_residue)
        .map_err(|e| VerificationError::ParseBigIntError(e.to_string()))?;

    // Check source is in base state
    if (&path_residue % &base_mod) != base_residue {
        return Err(VerificationError::ResidueMismatch {
            declared: cert.path_source_residue.clone(),
            computed: "Not in base state".to_string(),
        });
    }

    // Check universal target determinism: path_source_modulus_exponent >= composite_prefix.total_twos + base_state_modulus_exponent
    let required_mod_exp = composite_prefix.total_twos + cert.base_state_modulus_exponent;
    if cert.path_source_modulus_exponent < required_mod_exp {
        return Err(VerificationError::ResidueMismatch {
            declared: format!("Modulus exponent {}", cert.path_source_modulus_exponent),
            computed: format!("Required minimum modulus exponent {}", required_mod_exp),
        });
    }

    // Check intermediate state progression for path_residue
    let step_prefixes: Vec<AffinePrefix> = cert
        .steps
        .iter()
        .map(|s| {
            let w = ValuationWord::from_u32_slice(&s.valuation_word).unwrap();
            AffinePrefix::from_valuation_word(w).unwrap()
        })
        .collect();

    let mut curr = path_residue.clone();
    for step_pref in &step_prefixes {
        let next_val = step_pref
            .apply(&curr)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        if (&next_val % &base_mod) != base_residue {
            return Err(VerificationError::ResidueMismatch {
                declared: cert.path_source_residue.clone(),
                computed: format!("Step failed: image {} not in base state", next_val),
            });
        }
        curr = next_val;
    }

    Ok(cert)
}
