use collatz_affine::{classify_guarded_return, CanonicalCylinder, MacrostepData, ValuationWord};
use collatz_cert::schema::{
    GuardedReturnClassificationJson, Phase73bVerificationReportJson, QuotientRegisterTransitionJson,
};
use collatz_cert::verify_phase73b_report;
use num_bigint::BigUint;
use serde_json::json;
use std::fs;
use std::path::Path;

fn build_valid_phase73b_report() -> Phase73bVerificationReportJson {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word.clone()).unwrap();

    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();
    let m_v = MacrostepData::from_word(v_word.clone()).unwrap();

    let q1_base = CanonicalCylinder::new(BigUint::from(7u32), 5);

    let class_u = classify_guarded_return(&m_u, &q1_base).unwrap();
    let class_v = classify_guarded_return(&m_v, &q1_base).unwrap();

    let json_cu = GuardedReturnClassificationJson {
        schema_version: "guarded_return_classification_v1".to_string(),
        valuation_word: vec![1, 1, 2],
        exact_word_residue: class_u.exact_word_cylinder.residue.to_string(),
        exact_word_modulus_exponent: class_u.exact_word_cylinder.modulus_exponent,
        based_return_residue: class_u.based_return_cylinder.residue.to_string(),
        based_return_modulus_exponent: class_u.based_return_cylinder.modulus_exponent,
        positive_image_start: class_u.positive_image.start.to_string(),
        positive_image_step: class_u.positive_image.step.to_string(),
        target_residue: "7".to_string(),
        target_modulus_exponent: 5,
        quotient_guard_residue: class_u.quotient_guard.residue.to_string(),
        quotient_guard_modulus_exponent: class_u.quotient_guard.modulus_exponent,
    };

    let json_cv = GuardedReturnClassificationJson {
        schema_version: "guarded_return_classification_v1".to_string(),
        valuation_word: vec![1, 1, 2, 1, 2, 2],
        exact_word_residue: class_v.exact_word_cylinder.residue.to_string(),
        exact_word_modulus_exponent: class_v.exact_word_cylinder.modulus_exponent,
        based_return_residue: class_v.based_return_cylinder.residue.to_string(),
        based_return_modulus_exponent: class_v.based_return_cylinder.modulus_exponent,
        positive_image_start: class_v.positive_image.start.to_string(),
        positive_image_step: class_v.positive_image.step.to_string(),
        target_residue: "7".to_string(),
        target_modulus_exponent: 5,
        quotient_guard_residue: class_v.quotient_guard.residue.to_string(),
        quotient_guard_modulus_exponent: class_v.quotient_guard.modulus_exponent,
    };

    let json_tu = QuotientRegisterTransitionJson {
        schema_version: "quotient_register_transition_v1".to_string(),
        valuation_word: vec![1, 1, 2],
        eta: "3".to_string(),
        guard_residue: "7".to_string(),
        guard_modulus_exponent: 4,
        starting_k: "7".to_string(),
        outcome_type: "based_return".to_string(),
        next_k: Some("12".to_string()),
        image: Some("391".to_string()),
    };

    let json_tv = QuotientRegisterTransitionJson {
        schema_version: "quotient_register_transition_v1".to_string(),
        valuation_word: vec![1, 1, 2, 1, 2, 2],
        eta: "75".to_string(),
        guard_residue: "61".to_string(),
        guard_modulus_exponent: 9,
        starting_k: "61".to_string(),
        outcome_type: "based_return".to_string(),
        next_k: Some("87".to_string()),
        image: Some("2791".to_string()),
    };

    Phase73bVerificationReportJson {
        schema_version: "phase73b_verification_report_v1".to_string(),
        transitions: vec![json_tu, json_tv],
        classifications: vec![json_cu, json_cv],
        all_register_rules_verified: true,
    }
}

#[test]
fn test_phase73b_roundtrip_verification() {
    let valid_report = build_valid_phase73b_report();
    let json_str = serde_json::to_string_pretty(&valid_report).unwrap();
    assert!(verify_phase73b_report(&json_str).is_ok());

    let artifact_dir = Path::new("artifacts/phase73b");
    let _ = fs::create_dir_all(artifact_dir);

    // Export rust_quotient_results.json
    fs::write(artifact_dir.join("rust_quotient_results.json"), &json_str).unwrap();
}

#[test]
#[allow(clippy::type_complexity)]
fn test_10_corruption_mutation_matrix_phase73b() {
    let valid_report = build_valid_phase73b_report();
    let mut mutation_results = Vec::new();

    let mutations: Vec<(&str, Box<dyn Fn(&mut Phase73bVerificationReportJson)>)> = vec![
        (
            "Change one valuation in p",
            Box::new(|r| r.transitions[0].valuation_word[0] = 2),
        ),
        (
            "Change eta constant",
            Box::new(|r| r.transitions[0].eta = "4".to_string()),
        ),
        (
            "Change guard residue",
            Box::new(|r| r.transitions[0].guard_residue = "8".to_string()),
        ),
        (
            "Change guard modulus exponent",
            Box::new(|r| r.transitions[0].guard_modulus_exponent = 5),
        ),
        (
            "Corrupt outcome type (claiming leaves_base for returning k=7)",
            Box::new(|r| r.transitions[0].outcome_type = "leaves_base".to_string()),
        ),
        (
            "Corrupt next_k register",
            Box::new(|r| r.transitions[0].next_k = Some("13".to_string())),
        ),
        (
            "Corrupt transition image",
            Box::new(|r| r.transitions[0].image = Some("392".to_string())),
        ),
        (
            "Corrupt based return residue in classification",
            Box::new(|r| {
                r.classifications[0].based_return_residue = "232".to_string();
            }),
        ),
        (
            "Corrupt positive image start in classification",
            Box::new(|r| {
                r.classifications[0].positive_image_start = "392".to_string();
            }),
        ),
        (
            "Corrupt positive image step in classification",
            Box::new(|r| {
                r.classifications[0].positive_image_step = "865".to_string();
            }),
        ),
    ];

    let mut rejected_count = 0;
    for (name, mutator) in &mutations {
        let mut mutated = valid_report.clone();
        mutator(&mut mutated);
        let str_mut = serde_json::to_string(&mutated).unwrap();

        let is_rejected = verify_phase73b_report(&str_mut).is_err();
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

    assert_eq!(rejected_count, 10);

    let artifact_dir = Path::new("artifacts/phase73b");
    let _ = fs::create_dir_all(artifact_dir);

    let mutation_report = json!({
        "schema_version": "mutation_test_report_phase73b_v1",
        "valid_fixtures_accepted": "1/1",
        "mutated_fixtures_rejected": format!("{}/10", rejected_count),
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
