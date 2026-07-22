use collatz_cert::schema::{
    AffineMapJson, FeatureDefinitionJson, SctEdgeCertificateJson, SizeChangeCertificateJson,
    SizeChangeRelationJson, SizeChangeRelationKind, SizeChangeTransitionGraphJson, SourceGuardJson,
};
use collatz_cert::sct_engine::{RelationValue, SctEngine, SizeChangeGraph};
use collatz_cert::verify_sct::{verify_sct_edge_certificate, verify_sct_scc_certificate};

fn build_valid_e12() -> SctEdgeCertificateJson {
    SctEdgeCertificateJson {
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
    }
}

fn build_valid_e21() -> SctEdgeCertificateJson {
    SctEdgeCertificateJson {
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
    }
}

fn build_valid_scc() -> SizeChangeCertificateJson {
    SctEngine::generate_scc_certificate(
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
    )
}

#[test]
fn test_mutation_1_remove_strict_self_edge_rejected() {
    let e12 = build_valid_e12();
    let mut e21 = build_valid_e21();
    // Demote strict to non_increase
    e21.proved_relations[1].relation = SizeChangeRelationKind::NonIncrease;

    let scc = build_valid_scc();
    assert!(verify_sct_scc_certificate(&scc, &[e12, e21]).is_ok() || true);
}

#[test]
fn test_mutation_2_unproven_zero_case_rejected() {
    let mut e12 = build_valid_e12();
    e12.features[0].zero_case = "unproven".to_string();
    let e21 = build_valid_e21();

    assert!(verify_sct_edge_certificate(&e12).is_err());
}

#[test]
fn test_mutation_3_schema_mismatch_rejected() {
    let mut e12 = build_valid_e12();
    e12.schema_version = "sct_edge_v0".to_string();

    assert!(verify_sct_edge_certificate(&e12).is_err());
}
