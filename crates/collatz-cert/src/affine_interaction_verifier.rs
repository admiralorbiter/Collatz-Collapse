#[allow(unused_imports)]
use crate::schema::{
    AffineInteractionJson, CrossFormCylinderRecoveryJson, MacrostepDataJson,
    Phase73aVerificationReportJson,
};
use crate::VerificationError;
use collatz_affine::{
    recover_broad_cylinder, recover_exact_cylinder, recover_sequence_cylinder, AffineInteraction,
    MacrostepData, TwoAdicValuation, ValuationWord,
};

pub fn verify_phase73a_report(
    report_json: &str,
) -> Result<Phase73aVerificationReportJson, VerificationError> {
    let report: Phase73aVerificationReportJson = serde_json::from_str(report_json)
        .map_err(|e| VerificationError::JsonError(e.to_string()))?;

    if report.schema_version != "phase73a_verification_report_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "phase73a_verification_report_v1".to_string(),
            found: report.schema_version.clone(),
        });
    }

    if !report.all_identities_verified {
        return Err(VerificationError::ConstantMismatch {
            declared: "all_identities_verified = false".to_string(),
            computed: "all_identities_verified = true".to_string(),
        });
    }

    // 1. Verify all MacrostepData objects by recomputing from raw valuation words
    let mut computed_macrosteps = Vec::new();
    for json_m in &report.macrosteps {
        if json_m.schema_version != "macrostep_data_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "macrostep_data_v1".to_string(),
                found: json_m.schema_version.clone(),
            });
        }

        let word = ValuationWord::from_u32_slice(&json_m.valuation_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m = MacrostepData::from_word(word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        if json_m.odd_steps != m.odd_steps() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_m.odd_steps.to_string(),
                computed: m.odd_steps().to_string(),
            });
        }

        if json_m.total_valuation != m.total_valuation() {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_m.total_valuation,
                computed: m.total_valuation(),
            });
        }

        if json_m.multiplier != m.multiplier().to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_m.multiplier.clone(),
                computed: m.multiplier().to_string(),
            });
        }

        if json_m.denominator != m.denominator().to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_m.denominator.clone(),
                computed: m.denominator().to_string(),
            });
        }

        if json_m.constant != m.constant().to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_m.constant.clone(),
                computed: m.constant().to_string(),
            });
        }

        if json_m.fixed_point_denominator != m.d().to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_m.fixed_point_denominator.clone(),
                computed: m.d().to_string(),
            });
        }

        computed_macrosteps.push(m);
    }

    // 2. Verify all AffineInteraction objects
    for json_i in &report.interactions {
        if json_i.schema_version != "affine_interaction_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "affine_interaction_v1".to_string(),
                found: json_i.schema_version.clone(),
            });
        }

        let p_word = ValuationWord::from_u32_slice(&json_i.p_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let q_word = ValuationWord::from_u32_slice(&json_i.q_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let m_p = MacrostepData::from_word(p_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m_q = MacrostepData::from_word(q_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let inter = AffineInteraction::new(&m_p, &m_q);

        if json_i.delta != inter.delta().to_string() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_i.delta.clone(),
                computed: inter.delta().to_string(),
            });
        }

        let expected_delta_v2 = match inter.delta_v2() {
            TwoAdicValuation::Infinity => "infinity".to_string(),
            TwoAdicValuation::Finite(v) => v.to_string(),
        };

        if json_i.delta_v2 != expected_delta_v2 {
            return Err(VerificationError::ConstantMismatch {
                declared: json_i.delta_v2.clone(),
                computed: expected_delta_v2,
            });
        }

        if json_i.is_common_center != inter.is_common_center() {
            return Err(VerificationError::ConstantMismatch {
                declared: json_i.is_common_center.to_string(),
                computed: inter.is_common_center().to_string(),
            });
        }

        if !inter.same_form_identity_holds() || !json_i.same_form_identity_holds {
            return Err(VerificationError::ConstantMismatch {
                declared: "same_form_identity".to_string(),
                computed: "failed".to_string(),
            });
        }

        if !inter.cross_form_identity_holds() || !json_i.cross_form_identity_holds {
            return Err(VerificationError::ConstantMismatch {
                declared: "cross_form_identity".to_string(),
                computed: "failed".to_string(),
            });
        }

        if !inter.commutator_identity_holds() || !json_i.commutator_identity_holds {
            return Err(VerificationError::ConstantMismatch {
                declared: "commutator_identity".to_string(),
                computed: "failed".to_string(),
            });
        }
    }

    // 3. Verify all CrossFormCylinderRecovery objects
    for json_r in &report.cylinder_recoveries {
        if json_r.schema_version != "cross_form_cylinder_recovery_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "cross_form_cylinder_recovery_v1".to_string(),
                found: json_r.schema_version.clone(),
            });
        }

        let p_word = ValuationWord::from_u32_slice(&json_r.p_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let q_word = ValuationWord::from_u32_slice(&json_r.q_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let m_p = MacrostepData::from_word(p_word.clone())
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m_q = MacrostepData::from_word(q_word.clone())
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let inter = AffineInteraction::new(&m_p, &m_q);

        let broad_rec = recover_broad_cylinder(&inter)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let exact_rec = recover_exact_cylinder(&inter)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let seq_rec = recover_sequence_cylinder(&p_word, &q_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        if json_r.broad_cylinder_residue != broad_rec.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_r.broad_cylinder_residue.clone(),
                computed: broad_rec.residue.to_string(),
            });
        }

        if json_r.broad_cylinder_modulus_exponent != broad_rec.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_r.broad_cylinder_modulus_exponent,
                computed: broad_rec.modulus_exponent,
            });
        }

        if json_r.exact_cylinder_residue != exact_rec.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_r.exact_cylinder_residue.clone(),
                computed: exact_rec.residue.to_string(),
            });
        }

        if json_r.exact_cylinder_modulus_exponent != exact_rec.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_r.exact_cylinder_modulus_exponent,
                computed: exact_rec.modulus_exponent,
            });
        }

        if json_r.sequence_exact_cylinder_residue != seq_rec.residue.to_string() {
            return Err(VerificationError::ResidueMismatch {
                declared: json_r.sequence_exact_cylinder_residue.clone(),
                computed: seq_rec.residue.to_string(),
            });
        }

        if json_r.sequence_exact_cylinder_modulus_exponent != seq_rec.modulus_exponent {
            return Err(VerificationError::TotalTwosMismatch {
                declared: json_r.sequence_exact_cylinder_modulus_exponent,
                computed: seq_rec.modulus_exponent,
            });
        }

        if !json_r.parity_term_preserved {
            return Err(VerificationError::ConstantMismatch {
                declared: "parity_term_preserved = false".to_string(),
                computed: "parity_term_preserved = true".to_string(),
            });
        }
    }

    Ok(report)
}
