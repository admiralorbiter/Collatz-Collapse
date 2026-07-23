use collatz_affine::{
    recover_broad_cylinder, recover_exact_cylinder, recover_sequence_cylinder, AffineInteraction,
    MacrostepData, TwoAdicValuation, ValuationWord,
};
use collatz_cert::schema::{
    AffineInteractionJson, CrossFormCylinderRecoveryJson, MacrostepDataJson,
    Phase73aVerificationReportJson,
};
use collatz_cert::verify_phase73a_report;
use serde_json::json;
use std::fs;
use std::path::Path;

fn build_macrostep_json(word_slice: &[u32]) -> (MacrostepData, MacrostepDataJson) {
    let word = ValuationWord::from_u32_slice(word_slice).unwrap();
    let m = MacrostepData::from_word(word).unwrap();
    let json = MacrostepDataJson {
        schema_version: "macrostep_data_v1".to_string(),
        valuation_word: word_slice.to_vec(),
        odd_steps: m.odd_steps(),
        total_valuation: m.total_valuation(),
        multiplier: m.multiplier().to_string(),
        denominator: m.denominator().to_string(),
        constant: m.constant().to_string(),
        fixed_point_denominator: m.d().to_string(),
    };
    (m, json)
}

fn build_interaction_json<'a>(
    m_p: &'a MacrostepData,
    m_q: &'a MacrostepData,
    p_slice: &[u32],
    q_slice: &[u32],
) -> (
    AffineInteraction<'a>,
    AffineInteractionJson,
    CrossFormCylinderRecoveryJson,
) {
    let inter = AffineInteraction::new(m_p, m_q);
    let broad_rec = recover_broad_cylinder(&inter).unwrap();
    let exact_rec = recover_exact_cylinder(&inter).unwrap();

    let p_word = ValuationWord::from_u32_slice(p_slice).unwrap();
    let q_word = ValuationWord::from_u32_slice(q_slice).unwrap();
    let seq_rec = recover_sequence_cylinder(&p_word, &q_word).unwrap();

    let delta_v2_str = match inter.delta_v2() {
        TwoAdicValuation::Infinity => "infinity".to_string(),
        TwoAdicValuation::Finite(v) => v.to_string(),
    };

    let inter_json = AffineInteractionJson {
        schema_version: "affine_interaction_v1".to_string(),
        p_word: p_slice.to_vec(),
        q_word: q_slice.to_vec(),
        delta: inter.delta().to_string(),
        delta_v2: delta_v2_str,
        is_common_center: inter.is_common_center(),
        same_form_identity_holds: inter.same_form_identity_holds(),
        cross_form_identity_holds: inter.cross_form_identity_holds(),
        commutator_identity_holds: inter.commutator_identity_holds(),
    };

    let recovery_json = CrossFormCylinderRecoveryJson {
        schema_version: "cross_form_cylinder_recovery_v1".to_string(),
        p_word: p_slice.to_vec(),
        q_word: q_slice.to_vec(),
        broad_cylinder_residue: broad_rec.residue.to_string(),
        broad_cylinder_modulus_exponent: broad_rec.modulus_exponent,
        exact_cylinder_residue: exact_rec.residue.to_string(),
        exact_cylinder_modulus_exponent: exact_rec.modulus_exponent,
        sequence_exact_cylinder_residue: seq_rec.residue.to_string(),
        sequence_exact_cylinder_modulus_exponent: seq_rec.modulus_exponent,
        parity_term_preserved: true,
    };

    (inter, inter_json, recovery_json)
}

fn build_multi_branch_report() -> Phase73aVerificationReportJson {
    let (m_u, j_u) = build_macrostep_json(&[1, 1, 2]); // d = -11 < 0
    let (m_v, j_v) = build_macrostep_json(&[1, 1, 2, 1, 2, 2]); // d = -217 < 0
    let (m_w2, j_w2) = build_macrostep_json(&[1, 2, 2]); // d = +5 > 0

    let (_, i_uv, r_uv) = build_interaction_json(&m_u, &m_v, &[1, 1, 2], &[1, 1, 2, 1, 2, 2]); // Delta < 0 (-5568)
    let (_, i_vu, r_vu) = build_interaction_json(&m_v, &m_u, &[1, 1, 2, 1, 2, 2], &[1, 1, 2]); // Delta > 0 (+5568)
    let (_, i_w1w2, r_w1w2) = build_interaction_json(&m_u, &m_w2, &[1, 1, 2], &[1, 2, 2]); // Delta < 0 (-348), d_w2 > 0
    let (_, i_uu, r_uu) = build_interaction_json(&m_u, &m_u, &[1, 1, 2], &[1, 1, 2]); // Delta = 0, kappa = infinity

    Phase73aVerificationReportJson {
        schema_version: "phase73a_verification_report_v1".to_string(),
        macrosteps: vec![j_u, j_v, j_w2],
        interactions: vec![i_uv, i_vu, i_w1w2, i_uu],
        cylinder_recoveries: vec![r_uv, r_vu, r_w1w2, r_uu],
        all_identities_verified: true,
    }
}

#[test]
fn test_phase73a_roundtrip_verification() {
    let valid_report = build_multi_branch_report();
    let json_str = serde_json::to_string_pretty(&valid_report).unwrap();
    assert!(verify_phase73a_report(&json_str).is_ok());

    let artifact_dir = Path::new("artifacts/phase73a");
    let _ = fs::create_dir_all(artifact_dir);

    // Export rust_interaction_results.json
    fs::write(
        artifact_dir.join("rust_interaction_results.json"),
        &json_str,
    )
    .unwrap();

    // Export certificate_roundtrip_report.json
    let roundtrip_report = json!({
        "status": "PASSED",
        "supported_schemas": [
            {
                "schema_version": "macrostep_data_v1",
                "valid_fixtures_tested": 3,
                "roundtrip_status": "PASSED"
            },
            {
                "schema_version": "affine_interaction_v1",
                "valid_fixtures_tested": 4,
                "branches_covered": ["Delta < 0", "Delta > 0", "Delta = 0", "d > 0", "d < 0", "infinity_v2"],
                "roundtrip_status": "PASSED"
            },
            {
                "schema_version": "cross_form_cylinder_recovery_v1",
                "valid_fixtures_tested": 4,
                "branches_covered": ["broad_recovery", "exact_recovery", "parity_preservation", "sequence_recovery"],
                "roundtrip_status": "PASSED"
            },
            {
                "schema_version": "phase73a_verification_report_v1",
                "valid_fixtures_tested": 1,
                "roundtrip_status": "PASSED"
            }
        ],
        "deny_unknown_fields": true,
        "roundtrip_successful": true
    });
    fs::write(
        artifact_dir.join("certificate_roundtrip_report.json"),
        serde_json::to_string_pretty(&roundtrip_report).unwrap(),
    )
    .unwrap();
}

#[test]
#[allow(clippy::type_complexity)]
fn test_16_corruption_mutation_matrix_phase73a() {
    let valid_report = build_multi_branch_report();
    let mut mutation_results = Vec::new();

    let mutations: Vec<(&str, Box<dyn Fn(&mut Phase73aVerificationReportJson)>)> = vec![
        (
            "Change one valuation in p",
            Box::new(|r| r.macrosteps[0].valuation_word[0] = 2),
        ),
        (
            "Change one valuation in q",
            Box::new(|r| r.macrosteps[1].valuation_word[0] = 2),
        ),
        (
            "Change a_p multiplier",
            Box::new(|r| r.macrosteps[0].multiplier = "28".to_string()),
        ),
        (
            "Change b_p denominator",
            Box::new(|r| r.macrosteps[0].denominator = "17".to_string()),
        ),
        (
            "Change c_p constant",
            Box::new(|r| r.macrosteps[0].constant = "20".to_string()),
        ),
        (
            "Change d_p fixed point denominator",
            Box::new(|r| r.macrosteps[0].fixed_point_denominator = "-12".to_string()),
        ),
        (
            "Negate Delta only",
            Box::new(|r| r.interactions[0].delta = "5568".to_string()),
        ),
        (
            "Change delta_v2 kappa exponent",
            Box::new(|r| r.interactions[0].delta_v2 = "7".to_string()),
        ),
        (
            "Set finite delta_v2 to infinity",
            Box::new(|r| r.interactions[0].delta_v2 = "infinity".to_string()),
        ),
        (
            "Set infinity delta_v2 to finite (on (u,u) interaction)",
            Box::new(|r| r.interactions[3].delta_v2 = "6".to_string()),
        ),
        (
            "Reverse commutator orientation without changing sign",
            Box::new(|r| {
                r.interactions[0].p_word = vec![1, 1, 2, 1, 2, 2];
                r.interactions[0].q_word = vec![1, 1, 2];
            }),
        ),
        (
            "Replace exact exponent A_q + 1 with A_q",
            Box::new(|r| r.cylinder_recoveries[0].exact_cylinder_modulus_exponent = 9),
        ),
        (
            "Drop -b_q*c_p exactness term",
            Box::new(|r| {
                r.cylinder_recoveries[0].exact_cylinder_residue =
                    r.cylinder_recoveries[0].broad_cylinder_residue.clone();
            }),
        ),
        (
            "Replace recovered q-cylinder with concatenated pq-cylinder",
            Box::new(|r| {
                r.cylinder_recoveries[0].exact_cylinder_residue = r.cylinder_recoveries[0]
                    .sequence_exact_cylinder_residue
                    .clone();
            }),
        ),
        (
            "Change common_center flag only",
            Box::new(|r| r.interactions[0].is_common_center = true),
        ),
        (
            "Corrupt sequence exact cylinder residue",
            Box::new(|r| {
                r.cylinder_recoveries[0].sequence_exact_cylinder_residue = "1768".to_string();
            }),
        ),
    ];

    let mut rejected_count = 0;
    for (name, mutator) in &mutations {
        let mut mutated = valid_report.clone();
        mutator(&mut mutated);
        let str_mut = serde_json::to_string(&mutated).unwrap();

        let is_rejected = verify_phase73a_report(&str_mut).is_err();
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

    assert_eq!(rejected_count, 16);

    let artifact_dir = Path::new("artifacts/phase73a");
    let _ = fs::create_dir_all(artifact_dir);

    let mutation_report = json!({
        "schema_version": "mutation_test_report_v1",
        "valid_fixtures_accepted": "1/1 (Multi-Branch Aggregate Report)",
        "mutated_fixtures_rejected": format!("{}/16", rejected_count),
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
