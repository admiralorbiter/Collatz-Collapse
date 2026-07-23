use crate::schema::Phase73cVerificationReportJson;
use crate::VerificationError;
use collatz_affine::{SymbolicLanguageEnumerator, SymbolicWordData, ValuationWord};

pub fn verify_phase73c_report(
    report_json: &str,
) -> Result<Phase73cVerificationReportJson, VerificationError> {
    let report: Phase73cVerificationReportJson = serde_json::from_str(report_json)
        .map_err(|e| VerificationError::JsonError(e.to_string()))?;

    if report.schema_version != "phase73c_verification_report_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "phase73c_verification_report_v1".to_string(),
            found: report.schema_version.clone(),
        });
    }

    if !report.all_guards_cross_validated {
        return Err(VerificationError::ConstantMismatch {
            declared: "all_guards_cross_validated = false".to_string(),
            computed: "all_guards_cross_validated = true".to_string(),
        });
    }

    if report.total_nonempty_words != report.word_classifications.len() {
        return Err(VerificationError::ConstantMismatch {
            declared: report.total_nonempty_words.to_string(),
            computed: report.word_classifications.len().to_string(),
        });
    }

    for json_w in &report.word_classifications {
        if json_w.schema_version != "symbolic_word_classification_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "symbolic_word_classification_v1".to_string(),
                found: json_w.schema_version.clone(),
            });
        }

        let word = ValuationWord::from_u32_slice(&json_w.valuation_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let data = SymbolicWordData::from_word(word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        if data.eta.to_string() != json_w.eta {
            return Err(VerificationError::ConstantMismatch {
                declared: json_w.eta.clone(),
                computed: data.eta.to_string(),
            });
        }

        if data.guard_residue.to_string() != json_w.guard_residue {
            return Err(VerificationError::ConstantMismatch {
                declared: json_w.guard_residue.clone(),
                computed: data.guard_residue.to_string(),
            });
        }

        if data.guard_modulus_exponent != json_w.guard_modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_w.guard_modulus_exponent,
                computed: data.guard_modulus_exponent,
            });
        }

        if data.least_source_n.to_string() != json_w.least_source_n {
            return Err(VerificationError::ConstantMismatch {
                declared: json_w.least_source_n.clone(),
                computed: data.least_source_n.to_string(),
            });
        }

        let computed_is_zero = json_w.lift_digit == "0";
        if json_w.is_zero_lift != computed_is_zero {
            return Err(VerificationError::ConstantMismatch {
                declared: format!("is_zero_lift = {}", json_w.is_zero_lift),
                computed: format!("is_zero_lift = {}", computed_is_zero),
            });
        }

        // 3-way guard cross-validation check
        let cross_ok = SymbolicLanguageEnumerator::cross_validate_guards(&data)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        if !cross_ok {
            return Err(VerificationError::ConstantMismatch {
                declared: "cross_validation_ok = true".to_string(),
                computed: "cross_validation_ok = false".to_string(),
            });
        }
    }

    Ok(report)
}
