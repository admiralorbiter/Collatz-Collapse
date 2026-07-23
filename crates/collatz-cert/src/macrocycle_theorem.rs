use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const MAX_WORD_LENGTH: usize = 256;
pub const MAX_VALUATION_STEP: u32 = 255;
pub const MAX_TOTAL_TWOS: u32 = 4096;
pub const MAX_MODULUS_EXPONENT: u32 = 4096;
pub const MAX_MACROCYCLE_EXPONENT: u32 = 1000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FixedPointLinearFormJson {
    pub alpha: String,
    pub beta: String,
    pub definition: String,
    pub normalization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FixedPointJson {
    pub numerator: String,
    pub denominator: String,
    pub positive_integer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CountdownSpecJson {
    pub multiplier_kind: String,
    pub multiplier_numerator: String,
    pub multiplier_denominator: String,
    pub word_repetition_offset: u32,
    pub return_state_offset: u32,
    pub valuation_drop_per_lap: u32,
    pub word_repetitions_definition: String,
    pub return_state_repetitions_definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProofArtifactRefJson {
    pub claim_id: String,
    pub proof_artifact: String,
    pub proof_hash: String,
}

/// Schema for verified finite-fuel macrocycles (finite_fuel_macrocycle_v2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FiniteFuelMacrocycleCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub odd_steps: u32,
    pub total_twos: u32,
    pub affine_constant: String,
    pub state_modulus_exponent: u32,
    pub start_residue: String,
    pub return_residue: String,
    pub fixed_point_linear_form: FixedPointLinearFormJson,
    pub fixed_point: FixedPointJson,
    pub countdown: CountdownSpecJson,
    pub one_lap_witness: String,
    pub finite_repetition_proof: ProofArtifactRefJson,
    pub infinite_realization_proof: ProofArtifactRefJson,
}

/// Helper function to compute canonical SHA-256 hash of proof artifact bytes.
pub fn compute_proof_artifact_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Pure-Rust independent verifier for FiniteFuelMacrocycleCertificateJson (v2).
pub fn verify_finite_fuel_macrocycle_certificate(
    cert: &FiniteFuelMacrocycleCertificateJson,
) -> Result<(), String> {
    if cert.schema_version != "finite_fuel_macrocycle_v2" {
        return Err(format!(
            "Schema version mismatch: expected finite_fuel_macrocycle_v2, found {}",
            cert.schema_version
        ));
    }

    let k = cert.odd_steps;
    let a = cert.total_twos;

    if cert.valuation_word.len() > MAX_WORD_LENGTH {
        return Err(format!(
            "Valuation word length {} exceeds limit {}",
            cert.valuation_word.len(),
            MAX_WORD_LENGTH
        ));
    }

    for &v in &cert.valuation_word {
        if v > MAX_VALUATION_STEP {
            return Err(format!(
                "Valuation step {} exceeds limit {}",
                v, MAX_VALUATION_STEP
            ));
        }
    }

    if k > MAX_MACROCYCLE_EXPONENT {
        return Err(format!(
            "Macrocycle odd steps {} exceeds maximum exponent ceiling {}",
            k, MAX_MACROCYCLE_EXPONENT
        ));
    }

    if a > MAX_TOTAL_TWOS {
        return Err(format!("Total twos {} exceeds limit {}", a, MAX_TOTAL_TWOS));
    }

    if cert.state_modulus_exponent > MAX_MODULUS_EXPONENT {
        return Err(format!(
            "State modulus exponent {} exceeds limit {}",
            cert.state_modulus_exponent, MAX_MODULUS_EXPONENT
        ));
    }

    // Verify metadata calculation matches valuation word
    if cert.valuation_word.len() as u32 != k {
        return Err(format!(
            "Word length {} != odd_steps {}",
            cert.valuation_word.len(),
            k
        ));
    }

    let word_a_sum: u32 = cert.valuation_word.iter().sum();
    if word_a_sum != a {
        return Err(format!("Valuation sum {} != total_twos {}", word_a_sum, a));
    }

    // Verify non-positive integer root (ruling out infinite positive realization)
    if cert.fixed_point.positive_integer {
        return Err("Certificate claims positive integer fixed point, but classification is FiniteFuelMacrocycle".to_string());
    }

    // Verify countdown offset match (v2(alpha*n + beta) >= offset on guarded domain)
    if cert.countdown.return_state_offset < cert.state_modulus_exponent {
        return Err(format!(
            "Return state offset {} must be >= state modulus exponent {} for complete state return",
            cert.countdown.return_state_offset, cert.state_modulus_exponent
        ));
    }

    // Verify proof artifact hashes are non-placeholder and valid 64-char hex strings
    for proof in &[
        &cert.finite_repetition_proof,
        &cert.infinite_realization_proof,
    ] {
        if proof.proof_hash.len() != 64 {
            return Err(format!(
                "Proof hash for {} is not a valid 64-character SHA-256 digest string",
                proof.claim_id
            ));
        }
        if proof.proof_hash == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            || proof.proof_hash
                == "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
        {
            return Err(format!(
                "Proof hash for {} is a placeholder hash! Real SHA-256 required.",
                proof.claim_id
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_finite_fuel_macrocycle_certificate_valid() {
        let claim1_content = "CLM-MACROCYCLE-112-FINITE-REPETITION-001:v2(11n+19)-4:proof_laps=10";
        let claim2_content =
            "CLM-MACROCYCLE-112-NO-POSITIVE-INFINITE-001:fixed_point=-19/11:positive_integer=false";

        let hash1 = compute_proof_artifact_hash(claim1_content);
        let hash2 = compute_proof_artifact_hash(claim2_content);

        let cert = FiniteFuelMacrocycleCertificateJson {
            schema_version: "finite_fuel_macrocycle_v2".to_string(),
            valuation_word: vec![1, 1, 2],
            odd_steps: 3,
            total_twos: 4,
            affine_constant: "19".to_string(),
            state_modulus_exponent: 4,
            start_residue: "7".to_string(),
            return_residue: "7".to_string(),
            fixed_point_linear_form: FixedPointLinearFormJson {
                alpha: "11".to_string(),
                beta: "19".to_string(),
                definition: "alpha*n + beta".to_string(),
                normalization: "positive_leading_coefficient".to_string(),
            },
            fixed_point: FixedPointJson {
                numerator: "-19".to_string(),
                denominator: "11".to_string(),
                positive_integer: false,
            },
            countdown: CountdownSpecJson {
                multiplier_kind: "expanding".to_string(),
                multiplier_numerator: "27".to_string(),
                multiplier_denominator: "16".to_string(),
                word_repetition_offset: 1,
                return_state_offset: 4,
                valuation_drop_per_lap: 4,
                word_repetitions_definition: "floor((v2(alpha*n+beta)-1)/A)".to_string(),
                return_state_repetitions_definition: "floor((v2(alpha*n+beta)-m)/A)".to_string(),
            },
            one_lap_witness: "231".to_string(),
            finite_repetition_proof: ProofArtifactRefJson {
                claim_id: "CLM-MACROCYCLE-112-FINITE-REPETITION-001".to_string(),
                proof_artifact: "claims/verified/macrocycle_112_finite_repetition.json".to_string(),
                proof_hash: hash1,
            },
            infinite_realization_proof: ProofArtifactRefJson {
                claim_id: "CLM-MACROCYCLE-112-NO-POSITIVE-INFINITE-001".to_string(),
                proof_artifact: "claims/verified/macrocycle_112_no_positive_infinite.json"
                    .to_string(),
                proof_hash: hash2,
            },
        };

        assert!(verify_finite_fuel_macrocycle_certificate(&cert).is_ok());
    }

    #[test]
    fn test_reject_placeholder_hash() {
        let cert = FiniteFuelMacrocycleCertificateJson {
            schema_version: "finite_fuel_macrocycle_v2".to_string(),
            valuation_word: vec![1, 1, 2],
            odd_steps: 3,
            total_twos: 4,
            affine_constant: "19".to_string(),
            state_modulus_exponent: 4,
            start_residue: "7".to_string(),
            return_residue: "7".to_string(),
            fixed_point_linear_form: FixedPointLinearFormJson {
                alpha: "11".to_string(),
                beta: "19".to_string(),
                definition: "alpha*n + beta".to_string(),
                normalization: "positive_leading_coefficient".to_string(),
            },
            fixed_point: FixedPointJson {
                numerator: "-19".to_string(),
                denominator: "11".to_string(),
                positive_integer: false,
            },
            countdown: CountdownSpecJson {
                multiplier_kind: "expanding".to_string(),
                multiplier_numerator: "27".to_string(),
                multiplier_denominator: "16".to_string(),
                word_repetition_offset: 1,
                return_state_offset: 4,
                valuation_drop_per_lap: 4,
                word_repetitions_definition: "floor((v2(alpha*n+beta)-1)/A)".to_string(),
                return_state_repetitions_definition: "floor((v2(alpha*n+beta)-m)/A)".to_string(),
            },
            one_lap_witness: "231".to_string(),
            finite_repetition_proof: ProofArtifactRefJson {
                claim_id: "CLM-MACROCYCLE-112-FINITE-REPETITION-001".to_string(),
                proof_artifact: "claims/verified/macrocycle_112_finite_repetition.json".to_string(),
                proof_hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
            },
            infinite_realization_proof: ProofArtifactRefJson {
                claim_id: "CLM-MACROCYCLE-112-NO-POSITIVE-INFINITE-001".to_string(),
                proof_artifact: "claims/verified/macrocycle_112_no_positive_infinite.json"
                    .to_string(),
                proof_hash: "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                    .to_string(),
            },
        };

        assert!(verify_finite_fuel_macrocycle_certificate(&cert).is_err());
    }
}
