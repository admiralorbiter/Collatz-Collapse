use crate::schema::Phase73b2VerificationReportJson;
use crate::VerificationError;
use collatz_affine::{
    ConcreteUltrametricState, MacrostepData, Q1Quotient, UltrametricMachine, UltrametricStepOutcome,
    ValuationWord,
};
use num_bigint::BigUint;

pub fn verify_phase73b_2_report(
    report_json: &str,
) -> Result<Phase73b2VerificationReportJson, VerificationError> {
    let report: Phase73b2VerificationReportJson = serde_json::from_str(report_json)
        .map_err(|e| VerificationError::JsonError(e.to_string()))?;

    if report.schema_version != "phase73b_2_verification_report_v1" {
        return Err(VerificationError::SchemaMismatch {
            expected: "phase73b_2_verification_report_v1".to_string(),
            found: report.schema_version.clone(),
        });
    }

    if !report.all_commuting_diagrams_verified {
        return Err(VerificationError::ConstantMismatch {
            declared: "all_commuting_diagrams_verified = false".to_string(),
            computed: "all_commuting_diagrams_verified = true".to_string(),
        });
    }

    for json_t in &report.transitions {
        if json_t.schema_version != "ultrametric_state_transition_v1" {
            return Err(VerificationError::SchemaMismatch {
                expected: "ultrametric_state_transition_v1".to_string(),
                found: json_t.schema_version.clone(),
            });
        }

        let word = ValuationWord::from_u32_slice(&json_t.valuation_word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        let m = MacrostepData::from_word(word)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;

        let k_val = BigUint::parse_bytes(json_t.starting_k.as_bytes(), 10)
            .ok_or_else(|| VerificationError::ParseBigIntError(json_t.starting_k.clone()))?;
        let q = Q1Quotient::from_k(k_val);

        // Verify commuting diagram
        let diagram_ok = UltrametricMachine::verify_commuting_diagram(&q, &m)
            .map_err(|e| VerificationError::InvalidValuationWord(e.to_string()))?;
        if !diagram_ok {
            return Err(VerificationError::ConstantMismatch {
                declared: "commuting_diagram_ok = true".to_string(),
                computed: "commuting_diagram_ok = false".to_string(),
            });
        }

        let init_state = ConcreteUltrametricState::from_q1_quotient(&q);
        if let ConcreteUltrametricState::Finite { x, unit } = &init_state {
            if *x != json_t.start_x {
                return Err(VerificationError::TotalTwosMismatch {
                    declared: json_t.start_x,
                    computed: *x,
                });
            }
            if unit.to_string() != json_t.start_unit {
                return Err(VerificationError::ConstantMismatch {
                    declared: json_t.start_unit.clone(),
                    computed: unit.to_string(),
                });
            }
        }

        let outcome = if m.odd_steps() == 3 {
            UltrametricMachine::step_u(&init_state)
        } else {
            UltrametricMachine::step_v_resonant(&init_state)
        };

        match (json_t.outcome_type.as_str(), outcome) {
            ("non_integral", UltrametricStepOutcome::NonIntegral) => {}
            (
                "integral_even_outside_q1",
                UltrametricStepOutcome::IntegralEvenOutsideQ1 { image_x },
            ) => {
                let decl_x = json_t
                    .next_x
                    .ok_or_else(|| VerificationError::ConstantMismatch {
                        declared: "missing_next_x".to_string(),
                        computed: image_x.to_string(),
                    })?;
                if decl_x != image_x {
                    return Err(VerificationError::TotalTwosMismatch {
                        declared: decl_x,
                        computed: image_x,
                    });
                }
            }
            (
                "exact_leaves_q1",
                UltrametricStepOutcome::ExactButLeavesQ1 {
                    image_x,
                    image_unit,
                },
            ) => {
                let decl_x = json_t
                    .next_x
                    .ok_or_else(|| VerificationError::ConstantMismatch {
                        declared: "missing_next_x".to_string(),
                        computed: image_x.to_string(),
                    })?;
                if decl_x != image_x {
                    return Err(VerificationError::TotalTwosMismatch {
                        declared: decl_x,
                        computed: image_x,
                    });
                }
                let decl_unit = json_t.next_unit.as_ref().ok_or_else(|| {
                    VerificationError::ConstantMismatch {
                        declared: "missing_next_unit".to_string(),
                        computed: image_unit.to_string(),
                    }
                })?;
                if decl_unit != &image_unit.to_string() {
                    return Err(VerificationError::ConstantMismatch {
                        declared: decl_unit.clone(),
                        computed: image_unit.to_string(),
                    });
                }
            }
            ("based_return", UltrametricStepOutcome::BasedReturn { next_state }) => {
                if let ConcreteUltrametricState::Finite { x, unit } = next_state {
                    let decl_x =
                        json_t
                            .next_x
                            .ok_or_else(|| VerificationError::ConstantMismatch {
                                declared: "missing_next_x".to_string(),
                                computed: x.to_string(),
                            })?;
                    if decl_x != x {
                        return Err(VerificationError::TotalTwosMismatch {
                            declared: decl_x,
                            computed: x,
                        });
                    }
                    let decl_unit = json_t.next_unit.as_ref().ok_or_else(|| {
                        VerificationError::ConstantMismatch {
                            declared: "missing_next_unit".to_string(),
                            computed: unit.to_string(),
                        }
                    })?;
                    if decl_unit != &unit.to_string() {
                        return Err(VerificationError::ConstantMismatch {
                            declared: decl_unit.clone(),
                            computed: unit.to_string(),
                        });
                    }
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

    Ok(report)
}
