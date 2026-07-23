use crate::schema::Phase73bVerificationReportJson;
use crate::VerificationError;
use collatz_affine::{
    classify_guarded_return, MacrostepData, Q1Quotient, QuotientRegisterMachine,
    ReturnTransitionOutcome, ValuationWord,
};
use num_bigint::BigUint;

pub fn verify_phase73b_report(
    report_json: &str,
) -> Result<Phase73bVerificationReportJson, VerificationError> {
    let report: Phase73bVerificationReportJson = serde_json::from_str(report_json)
        .map_err(|e| VerificationError::JsonError(e.to_string()))?;

    if report.schema_version != "phase73b_verification_report_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "phase73b_verification_report_v1".to_string(),
            found: report.schema_version.clone(),
        });
    }

    if !report.all_register_rules_verified {
        return Err(VerificationError::ConstantMismatch {
            declared: "all_register_rules_verified = false".to_string(),
            computed: "all_register_rules_verified = true".to_string(),
        });
    }

    // 1. Verify all QuotientRegisterTransition objects
    for json_t in &report.transitions {
        if json_t.schema_version != "quotient_register_transition_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "quotient_register_transition_v1".to_string(),
                found: json_t.schema_version.clone(),
            });
        }

        let word = ValuationWord::from_u32_slice(&json_t.valuation_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m = MacrostepData::from_word(word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let rule = QuotientRegisterMachine::derive_rule(&m)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        if json_t.eta != rule.eta.to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_t.eta.clone(),
                computed: rule.eta.to_string(),
            });
        }

        if json_t.guard_residue != rule.guard_residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_t.guard_residue.clone(),
                computed: rule.guard_residue.to_string(),
            });
        }

        if json_t.guard_modulus_exponent != rule.guard_modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_t.guard_modulus_exponent,
                computed: rule.guard_modulus_exponent,
            });
        }

        let k_val = BigUint::parse_bytes(json_t.starting_k.as_bytes(), 10)
            .ok_or_else(|| VerificationError::ParseBigIntError(json_t.starting_k.clone()))?;
        let q = Q1Quotient::from_k(k_val);

        let outcome = QuotientRegisterMachine::eval_transition(&m, &q)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        match (json_t.outcome_type.as_str(), outcome) {
            ("not_exact", ReturnTransitionOutcome::NotExactWord) => {}
            ("leaves_base", ReturnTransitionOutcome::ExactButLeavesBase { image }) => {
                let decl_img =
                    json_t
                        .image
                        .as_ref()
                        .ok_or_else(|| VerificationError::ConstantMismatch {
                            declared: "missing_image".to_string(),
                            computed: image.to_string(),
                        })?;
                if decl_img != &image.to_string() {
                    return Err(VerificationError::ConstantMismatch {
                        declared: decl_img.clone(),
                        computed: image.to_string(),
                    });
                }
            }
            ("based_return", ReturnTransitionOutcome::BasedReturn { next_k, image }) => {
                let decl_next_k =
                    json_t
                        .next_k
                        .as_ref()
                        .ok_or_else(|| VerificationError::ConstantMismatch {
                            declared: "missing_next_k".to_string(),
                            computed: next_k.value().to_string(),
                        })?;
                if decl_next_k != &next_k.value().to_string() {
                    return Err(VerificationError::ConstantMismatch {
                        declared: decl_next_k.clone(),
                        computed: next_k.value().to_string(),
                    });
                }

                let decl_img =
                    json_t
                        .image
                        .as_ref()
                        .ok_or_else(|| VerificationError::ConstantMismatch {
                            declared: "missing_image".to_string(),
                            computed: image.to_string(),
                        })?;
                if decl_img != &image.to_string() {
                    return Err(VerificationError::ConstantMismatch {
                        declared: decl_img.clone(),
                        computed: image.to_string(),
                    });
                }
            }
            (other, computed_outcome) => {
                return Err(VerificationError::ConstantMismatch {
                    declared: other.to_string(),
                    computed: format!("{:?}", computed_outcome),
                });
            }
        }
    }

    // 2. Verify all GuardedReturnClassification objects
    for json_c in &report.classifications {
        if json_c.schema_version != "guarded_return_classification_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "guarded_return_classification_v1".to_string(),
                found: json_c.schema_version.clone(),
            });
        }

        let word = ValuationWord::from_u32_slice(&json_c.valuation_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m = MacrostepData::from_word(word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let target_res = BigUint::parse_bytes(json_c.target_residue.as_bytes(), 10)
            .ok_or_else(|| VerificationError::ParseBigIntError(json_c.target_residue.clone()))?;
        let base =
            collatz_affine::CanonicalCylinder::new(target_res, json_c.target_modulus_exponent);

        let class = classify_guarded_return(&m, &base)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        if json_c.exact_word_residue != class.exact_word_cylinder.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_c.exact_word_residue.clone(),
                computed: class.exact_word_cylinder.residue.to_string(),
            });
        }

        if json_c.exact_word_modulus_exponent != class.exact_word_cylinder.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_c.exact_word_modulus_exponent,
                computed: class.exact_word_cylinder.modulus_exponent,
            });
        }

        if json_c.based_return_residue != class.based_return_cylinder.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_c.based_return_residue.clone(),
                computed: class.based_return_cylinder.residue.to_string(),
            });
        }

        if json_c.based_return_modulus_exponent != class.based_return_cylinder.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_c.based_return_modulus_exponent,
                computed: class.based_return_cylinder.modulus_exponent,
            });
        }

        if json_c.positive_image_start != class.positive_image.start.to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_c.positive_image_start.clone(),
                computed: class.positive_image.start.to_string(),
            });
        }

        if json_c.positive_image_step != class.positive_image.step.to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_c.positive_image_step.clone(),
                computed: class.positive_image.step.to_string(),
            });
        }

        if json_c.quotient_guard_residue != class.quotient_guard.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_c.quotient_guard_residue.clone(),
                computed: class.quotient_guard.residue.to_string(),
            });
        }

        if json_c.quotient_guard_modulus_exponent != class.quotient_guard.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_c.quotient_guard_modulus_exponent,
                computed: class.quotient_guard.modulus_exponent,
            });
        }
    }

    Ok(report)
}
