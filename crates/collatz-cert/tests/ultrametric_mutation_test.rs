use collatz_affine::{ConcreteUltrametricState, Q1Quotient};
use collatz_cert::schema::{Phase73b2VerificationReportJson, UltrametricStateTransitionJson};
use collatz_cert::verify_phase73b_2_report;
use num_bigint::BigUint;
use serde_json::json;
use std::fs;
use std::path::Path;

fn build_valid_phase73b_2_report() -> Phase73b2VerificationReportJson {
    // k = 7 (n = 231) => u returns next_k = 12 (n' = 391)
    let q_7 = Q1Quotient::from_k(BigUint::from(7u32));
    let state_7 = ConcreteUltrametricState::from_q1_quotient(&q_7);
    let (start_x_7, start_u_7) = match state_7 {
        ConcreteUltrametricState::Finite { x, unit } => (x, unit.to_string()),
        _ => panic!(),
    };

    let q_12 = Q1Quotient::from_k(BigUint::from(12u32));
    let state_12 = ConcreteUltrametricState::from_q1_quotient(&q_12);
    let (next_x_12, next_u_12) = match state_12 {
        ConcreteUltrametricState::Finite { x, unit } => (x, unit.to_string()),
        _ => panic!(),
    };

    let json_tu = UltrametricStateTransitionJson {
        schema_version: "ultrametric_state_transition_v1".to_string(),
        valuation_word: vec![1, 1, 2],
        starting_k: "7".to_string(),
        start_x: start_x_7,
        start_unit: start_u_7,
        outcome_type: "based_return".to_string(),
        next_x: Some(next_x_12),
        next_unit: Some(next_u_12),
    };

    // k = 61 (n = 1959) => v returns next_k = 87 (n' = 2791)
    let q_61 = Q1Quotient::from_k(BigUint::from(61u32));
    let state_61 = ConcreteUltrametricState::from_q1_quotient(&q_61);
    let (start_x_61, start_u_61) = match state_61 {
        ConcreteUltrametricState::Finite { x, unit } => (x, unit.to_string()),
        _ => panic!(),
    };

    let q_87 = Q1Quotient::from_k(BigUint::from(87u32));
    let state_87 = ConcreteUltrametricState::from_q1_quotient(&q_87);
    let (next_x_87, next_u_87) = match state_87 {
        ConcreteUltrametricState::Finite { x, unit } => (x, unit.to_string()),
        _ => panic!(),
    };

    let json_tv = UltrametricStateTransitionJson {
        schema_version: "ultrametric_state_transition_v1".to_string(),
        valuation_word: vec![1, 1, 2, 1, 2, 2],
        starting_k: "61".to_string(),
        start_x: start_x_61,
        start_unit: start_u_61,
        outcome_type: "based_return".to_string(),
        next_x: Some(next_x_87),
        next_unit: Some(next_u_87),
    };

    Phase73b2VerificationReportJson {
        schema_version: "phase73b_2_verification_report_v1".to_string(),
        transitions: vec![json_tu, json_tv],
        all_commuting_diagrams_verified: true,
    }
}

#[test]
fn test_phase73b_2_roundtrip_verification() {
    let valid_report = build_valid_phase73b_2_report();
    let json_str = serde_json::to_string_pretty(&valid_report).unwrap();
    assert!(verify_phase73b_2_report(&json_str).is_ok());

    let artifact_dir = Path::new("artifacts/phase73b_2");
    let _ = fs::create_dir_all(artifact_dir);

    // Export rust_ultrametric_results.json
    fs::write(
        artifact_dir.join("rust_ultrametric_results.json"),
        &json_str,
    )
    .unwrap();
}

#[test]
#[allow(clippy::type_complexity)]
fn test_8_corruption_mutation_matrix_phase73b_2() {
    let valid_report = build_valid_phase73b_2_report();
    let mut mutation_results = Vec::new();

    let mutations: Vec<(&str, Box<dyn Fn(&mut Phase73b2VerificationReportJson)>)> = vec![
        (
            "Change one valuation in p",
            Box::new(|r| r.transitions[0].valuation_word[0] = 2),
        ),
        (
            "Corrupt starting x valuation",
            Box::new(|r| r.transitions[0].start_x = 99),
        ),
        (
            "Corrupt starting unit U",
            Box::new(|r| r.transitions[0].start_unit = "99".to_string()),
        ),
        (
            "Corrupt outcome type (claiming non_integral for returning k=7)",
            Box::new(|r| r.transitions[0].outcome_type = "non_integral".to_string()),
        ),
        (
            "Corrupt next_x valuation",
            Box::new(|r| r.transitions[0].next_x = Some(99)),
        ),
        (
            "Corrupt next_unit U",
            Box::new(|r| r.transitions[0].next_unit = Some("99".to_string())),
        ),
        (
            "Corrupt starting_k integer",
            Box::new(|r| r.transitions[0].starting_k = "8".to_string()),
        ),
        (
            "Set all_commuting_diagrams_verified to false",
            Box::new(|r| r.all_commuting_diagrams_verified = false),
        ),
    ];

    let mut rejected_count = 0;
    for (name, mutator) in &mutations {
        let mut mutated = valid_report.clone();
        mutator(&mut mutated);
        let str_mut = serde_json::to_string(&mutated).unwrap();

        let is_rejected = verify_phase73b_2_report(&str_mut).is_err();
        assert!(is_rejected, "Mutation '{}' should have been REJECTED", name);

        if is_rejected {
            rejected_count += 1;
        }

        mutation_results.push(json!({
            "mutation_description": name,
            "expected_result": "REJECTED",
            "actual_result": if is_rejected { "REJECTED" } else { "ACCEPTED" },
            "passed": is_rejected
        }));
    }

    assert_eq!(rejected_count, 8);

    let artifact_dir = Path::new("artifacts/phase73b_2");
    let _ = fs::create_dir_all(artifact_dir);

    let mutation_report = json!({
        "schema_version": "mutation_test_report_phase73b_2_v1",
        "valid_fixtures_accepted": "1/1",
        "mutated_fixtures_rejected": format!("{}/8", rejected_count),
        "unexpected_accepts": 0,
        "unexpected_rejects": 0,
        "mutations": mutation_results
    });

    fs::write(
        artifact_dir.join("mutation_test_report.json"),
        serde_json::to_string_pretty(&mutation_report).unwrap(),
    )
    .unwrap();
}
