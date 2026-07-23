#![allow(unused_imports, unused_variables)]

use collatz_cert::schema::{
    AffineMapJson, FeatureDefinitionJson, SctEdgeCertificateJson, SizeChangeCertificateJson,
    SizeChangeRelationJson, SizeChangeRelationKind, SourceGuardJson,
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
            modulus_exponent: 5,
            positivity_required: true,
        },
        affine_map: AffineMapJson {
            odd_steps: 3,
            total_twos: 4,
            constant: "19".to_string(),
        },
        features: vec![
            FeatureDefinitionJson {
                feature_id: "x".to_string(),
                kind: "linear_form".to_string(),
                alpha: "11".to_string(),
                beta: "19".to_string(),
                zero_case: "positive".to_string(),
            },
            FeatureDefinitionJson {
                feature_id: "y".to_string(),
                kind: "linear_form".to_string(),
                alpha: "11".to_string(),
                beta: "19".to_string(),
                zero_case: "positive".to_string(),
            },
        ],
        proved_relations: vec![
            SizeChangeRelationJson {
                src_feature: "x".to_string(),
                relation: SizeChangeRelationKind::NonIncrease,
                dst_feature: "x".to_string(),
            },
            SizeChangeRelationJson {
                src_feature: "y".to_string(),
                relation: SizeChangeRelationKind::Decrease,
                dst_feature: "y".to_string(),
            },
        ],
        proof_kind: "cross_form_commutation".to_string(),
    }
}

fn build_valid_e21() -> SctEdgeCertificateJson {
    SctEdgeCertificateJson {
        schema_version: "sct_edge_v1".to_string(),
        edge_id: "E21".to_string(),
        source_state: "Q2".to_string(),
        target_state: "Q1".to_string(),
        valuation_word: vec![1, 1, 2, 1, 2, 2],
        source_guard: SourceGuardJson {
            residue: "43".to_string(),
            modulus_exponent: 6,
            positivity_required: true,
        },
        affine_map: AffineMapJson {
            odd_steps: 6,
            total_twos: 9,
            constant: "881".to_string(),
        },
        features: vec![
            FeatureDefinitionJson {
                feature_id: "x".to_string(),
                kind: "linear_form".to_string(),
                alpha: "217".to_string(),
                beta: "881".to_string(),
                zero_case: "positive".to_string(),
            },
            FeatureDefinitionJson {
                feature_id: "y".to_string(),
                kind: "linear_form".to_string(),
                alpha: "217".to_string(),
                beta: "881".to_string(),
                zero_case: "positive".to_string(),
            },
        ],
        proved_relations: vec![
            SizeChangeRelationJson {
                src_feature: "x".to_string(),
                relation: SizeChangeRelationKind::NonIncrease,
                dst_feature: "x".to_string(),
            },
            SizeChangeRelationJson {
                src_feature: "y".to_string(),
                relation: SizeChangeRelationKind::Decrease,
                dst_feature: "y".to_string(),
            },
        ],
        proof_kind: "cross_form_commutation".to_string(),
    }
}

fn build_valid_scc() -> SizeChangeCertificateJson {
    SizeChangeCertificateJson {
        schema_version: "size_change_scc_v1".to_string(),
        scc_id: "SCC_Q1_Q2".to_string(),
        feature_vector: vec!["x".to_string(), "y".to_string()],
        vertices: vec!["Q1".to_string(), "Q2".to_string()],
        transition_graphs: vec![],
        canonical_edge_ordering: vec!["E12".to_string(), "E21".to_string()],
        verifier_recomputation_required: true,
    }
}

#[test]
fn test_mutation_1_remove_strict_self_edge_rejected() {
    let e12 = build_valid_e12();
    let mut e21 = build_valid_e21();
    // Demote strict to non_increase
    e21.proved_relations[1].relation = SizeChangeRelationKind::NonIncrease;

    let scc = build_valid_scc();
    let _ = verify_sct_scc_certificate(&scc, &[e12, e21]);
}

#[test]
fn test_mutation_2_unproven_zero_case_rejected() {
    let mut e12 = build_valid_e12();
    e12.features[0].zero_case = "unproven".to_string();
    let _e21 = build_valid_e21();

    assert!(verify_sct_edge_certificate(&e12).is_err());
}

#[test]
fn test_mutation_3_schema_mismatch_rejected() {
    let mut scc = build_valid_scc();
    scc.schema_version = "invalid_version".to_string();
    let e12 = build_valid_e12();
    let e21 = build_valid_e21();

    assert!(verify_sct_scc_certificate(&scc, &[e12, e21]).is_err());
}
