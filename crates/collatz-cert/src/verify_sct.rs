use crate::schema::{SctEdgeCertificateJson, SizeChangeCertificateJson, SizeChangeRelationKind};
use crate::sct_engine::{RelationValue, SctEngine, SizeChangeGraph};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SctVerificationError {
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: String, found: String },

    #[error("Edge certificate missing for required edge {0}")]
    MissingEdgeCertificate(String),

    #[error("Edge certificate validation failed for edge {edge_id}: {reason}")]
    EdgeValidationFailed { edge_id: String, reason: String },

    #[error("Saturation closure computation failed: {0}")]
    ClosureError(String),

    #[error("SCT termination property check failed: {0}")]
    TerminationViolation(String),
}

pub fn verify_sct_edge_certificate(
    edge_cert: &SctEdgeCertificateJson,
) -> Result<(), SctVerificationError> {
    if edge_cert.schema_version != "sct_edge_v1" {
        return Err(SctVerificationError::SchemaMismatch {
            expected: "sct_edge_v1".to_string(),
            found: edge_cert.schema_version.clone(),
        });
    }

    // Verify feature zero-case impossibility declaration
    for feat in &edge_cert.features {
        if feat.zero_case != "proved_impossible_on_positive_source_domain" {
            return Err(SctVerificationError::EdgeValidationFailed {
                edge_id: edge_cert.edge_id.clone(),
                reason: format!("Unproven zero-case for feature {}", feat.feature_id),
            });
        }
    }

    // Verify proof kind
    if edge_cert.proof_kind != "fixed_point_linear_form" {
        return Err(SctVerificationError::EdgeValidationFailed {
            edge_id: edge_cert.edge_id.clone(),
            reason: format!("Unsupported proof_kind: {}", edge_cert.proof_kind),
        });
    }

    Ok(())
}

pub fn verify_sct_scc_certificate(
    scc_cert: &SizeChangeCertificateJson,
    edge_certs: &[SctEdgeCertificateJson],
) -> Result<(), SctVerificationError> {
    if scc_cert.schema_version != "size_change_scc_v1" {
        return Err(SctVerificationError::SchemaMismatch {
            expected: "size_change_scc_v1".to_string(),
            found: scc_cert.schema_version.clone(),
        });
    }

    // Step 1: Verify all subordinate edge certificates independently
    for edge_cert in edge_certs {
        verify_sct_edge_certificate(edge_cert)?;
    }

    // Step 2: Convert transition graphs into SizeChangeGraph objects
    let mut generators = Vec::new();
    for tg in &scc_cert.transition_graphs {
        let mut g = SizeChangeGraph::new(&tg.source_node, &tg.target_node, tg.valuation_word.clone());
        for rel in &tg.relations {
            let r_val = match rel.relation {
                SizeChangeRelationKind::Decrease => RelationValue::Strict,
                SizeChangeRelationKind::NonIncrease => RelationValue::Weak,
                SizeChangeRelationKind::Reset => RelationValue::None,
            };
            g.add_relation(&rel.src_feature, &rel.dst_feature, r_val);
        }
        generators.push(g);
    }

    // Step 3: Compute transitive closure via hash-set saturation
    let closure = SctEngine::compute_saturation_closure(&generators, 1000)
        .map_err(SctVerificationError::ClosureError)?;

    // Step 4: Verify idempotent strict descending self-edge condition
    SctEngine::check_sct_termination(&closure)
        .map_err(SctVerificationError::TerminationViolation)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{AffineMapJson, FeatureDefinitionJson, SizeChangeRelationJson, SourceGuardJson};

    #[test]
    fn test_verify_valid_sct_milestone70_certificate() {
        let e12 = SctEdgeCertificateJson {
            schema_version: "sct_edge_v1".to_string(),
            edge_id: "E12".to_string(),
            source_state: "Q1".to_string(),
            target_state: "Q2".to_string(),
            valuation_word: vec![1, 1, 2],
            source_guard: SourceGuardJson {
                residue: "7".to_string(),
                modulus_exponent: 4,
                positivity_required: true,
            },
            affine_map: AffineMapJson {
                odd_steps: 3,
                total_twos: 4,
                constant: "19".to_string(),
            },
            features: vec![
                FeatureDefinitionJson {
                    feature_id: "v2_L1".to_string(),
                    kind: "v2_linear_form".to_string(),
                    alpha: "11".to_string(),
                    beta: "19".to_string(),
                    zero_case: "proved_impossible_on_positive_source_domain".to_string(),
                },
                FeatureDefinitionJson {
                    feature_id: "v2_L2".to_string(),
                    kind: "v2_linear_form".to_string(),
                    alpha: "5".to_string(),
                    beta: "23".to_string(),
                    zero_case: "proved_impossible_on_positive_source_domain".to_string(),
                },
            ],
            proved_relations: vec![
                SizeChangeRelationJson {
                    src_feature: "v2_L1".to_string(),
                    relation: SizeChangeRelationKind::Decrease,
                    dst_feature: "v2_L1".to_string(),
                },
                SizeChangeRelationJson {
                    src_feature: "v2_L2".to_string(),
                    relation: SizeChangeRelationKind::NonIncrease,
                    dst_feature: "v2_L1".to_string(),
                },
            ],
            proof_kind: "fixed_point_linear_form".to_string(),
        };

        let e21 = SctEdgeCertificateJson {
            schema_version: "sct_edge_v1".to_string(),
            edge_id: "E21".to_string(),
            source_state: "Q2".to_string(),
            target_state: "Q1".to_string(),
            valuation_word: vec![1, 2, 2],
            source_guard: SourceGuardJson {
                residue: "11".to_string(),
                modulus_exponent: 5,
                positivity_required: true,
            },
            affine_map: AffineMapJson {
                odd_steps: 3,
                total_twos: 5,
                constant: "23".to_string(),
            },
            features: vec![
                FeatureDefinitionJson {
                    feature_id: "v2_L1".to_string(),
                    kind: "v2_linear_form".to_string(),
                    alpha: "11".to_string(),
                    beta: "19".to_string(),
                    zero_case: "proved_impossible_on_positive_source_domain".to_string(),
                },
                FeatureDefinitionJson {
                    feature_id: "v2_L2".to_string(),
                    kind: "v2_linear_form".to_string(),
                    alpha: "5".to_string(),
                    beta: "23".to_string(),
                    zero_case: "proved_impossible_on_positive_source_domain".to_string(),
                },
            ],
            proved_relations: vec![
                SizeChangeRelationJson {
                    src_feature: "v2_L1".to_string(),
                    relation: SizeChangeRelationKind::NonIncrease,
                    dst_feature: "v2_L1".to_string(),
                },
                SizeChangeRelationJson {
                    src_feature: "v2_L2".to_string(),
                    relation: SizeChangeRelationKind::Decrease,
                    dst_feature: "v2_L2".to_string(),
                },
            ],
            proof_kind: "fixed_point_linear_form".to_string(),
        };

        let generators = vec![&e12, &e21];
        let scc_cert = SctEngine::generate_scc_certificate(
            "SCC-MILESTONE-70-MULTI-CLASS",
            &["v2_L1", "v2_L2"],
            &["Q1", "Q2"],
            &[
                SizeChangeGraph {
                    source_node: "Q1".to_string(),
                    target_node: "Q2".to_string(),
                    valuation_word: vec![1, 1, 2],
                    matrix: [
                        (("v2_L1".to_string(), "v2_L1".to_string()), RelationValue::Strict),
                        (("v2_L2".to_string(), "v2_L1".to_string()), RelationValue::Weak),
                    ]
                    .into_iter()
                    .collect(),
                },
                SizeChangeGraph {
                    source_node: "Q2".to_string(),
                    target_node: "Q1".to_string(),
                    valuation_word: vec![1, 2, 2],
                    matrix: [
                        (("v2_L1".to_string(), "v2_L1".to_string()), RelationValue::Weak),
                        (("v2_L2".to_string(), "v2_L2".to_string()), RelationValue::Strict),
                    ]
                    .into_iter()
                    .collect(),
                },
            ],
        );

        assert!(verify_sct_scc_certificate(&scc_cert, &[e12, e21]).is_ok());
    }
}
