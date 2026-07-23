use collatz_cert::schema::{GuardedPathCertificateJson, SequenceStepJson};
use collatz_cert::verify_guarded_path_certificate;
use std::fs;
use std::path::Path;

fn make_uv_certificate() -> GuardedPathCertificateJson {
    GuardedPathCertificateJson {
        schema_version: "guarded_path_v1".to_string(),
        execution_semantics: "left_to_right_v1".to_string(),
        steps: vec![
            SequenceStepJson {
                symbol: "u".to_string(),
                valuation_word: vec![1, 1, 2],
            },
            SequenceStepJson {
                symbol: "v".to_string(),
                valuation_word: vec![1, 1, 2, 1, 2, 2],
            },
        ],
        flattened_valuation_word: vec![1, 1, 2, 1, 1, 2, 1, 2, 2],
        base_state_residue: "7".to_string(),
        base_state_modulus_exponent: 5,
        path_source_residue: "214759".to_string(),
        path_source_modulus_exponent: 18,
        composite_multiplier: "19683".to_string(),
        composite_constant: "27947".to_string(),
        composite_denominator: "8192".to_string(),
    }
}

fn make_vu_certificate() -> GuardedPathCertificateJson {
    GuardedPathCertificateJson {
        schema_version: "guarded_path_v1".to_string(),
        execution_semantics: "left_to_right_v1".to_string(),
        steps: vec![
            SequenceStepJson {
                symbol: "v".to_string(),
                valuation_word: vec![1, 1, 2, 1, 2, 2],
            },
            SequenceStepJson {
                symbol: "u".to_string(),
                valuation_word: vec![1, 1, 2],
            },
        ],
        flattened_valuation_word: vec![1, 1, 2, 1, 2, 2, 1, 1, 2],
        base_state_residue: "7".to_string(),
        base_state_modulus_exponent: 5,
        path_source_residue: "1959".to_string(),
        path_source_modulus_exponent: 18,
        composite_multiplier: "19683".to_string(),
        composite_constant: "33515".to_string(),
        composite_denominator: "8192".to_string(),
    }
}

#[test]
fn test_guarded_path_roundtrip_and_export() {
    let uv_cert = make_uv_certificate();
    let vu_cert = make_vu_certificate();

    let uv_json = serde_json::to_string_pretty(&uv_cert).unwrap();
    let vu_json = serde_json::to_string_pretty(&vu_cert).unwrap();

    assert!(verify_guarded_path_certificate(&uv_json).is_ok());
    assert!(verify_guarded_path_certificate(&vu_json).is_ok());

    // Export artifacts to artifacts/phase73_0/
    let dir = Path::new("artifacts/phase73_0");
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("guarded_path_uv.json"), &uv_json).unwrap();
    fs::write(dir.join("guarded_path_vu.json"), &vu_json).unwrap();
}

#[test]
fn test_9_field_mutation_matrix() {
    let base_cert = make_uv_certificate();
    let mut mutation_results = Vec::new();

    // Mutation 1: Reverse sequence order
    let mut m1 = base_cert.clone();
    m1.steps.reverse();
    let res1 = verify_guarded_path_certificate(&serde_json::to_string(&m1).unwrap());
    assert!(res1.is_err());
    mutation_results.push(("reverse_sequence_order", res1.is_err()));

    // Mutation 2: Alter one valuation
    let mut m2 = base_cert.clone();
    m2.steps[0].valuation_word[2] = 3;
    let res2 = verify_guarded_path_certificate(&serde_json::to_string(&m2).unwrap());
    assert!(res2.is_err());
    mutation_results.push(("alter_one_valuation", res2.is_err()));

    // Mutation 3: Alter affine constant
    let mut m3 = base_cert.clone();
    m3.composite_constant = "27948".to_string();
    let res3 = verify_guarded_path_certificate(&serde_json::to_string(&m3).unwrap());
    assert!(res3.is_err());
    mutation_results.push(("alter_affine_constant", res3.is_err()));

    // Mutation 4: Alter source residue
    let mut m4 = base_cert.clone();
    m4.path_source_residue = "214760".to_string();
    let res4 = verify_guarded_path_certificate(&serde_json::to_string(&m4).unwrap());
    assert!(res4.is_err());
    mutation_results.push(("alter_source_residue", res4.is_err()));

    // Mutation 5: Lower source exponent
    let mut m5 = base_cert.clone();
    m5.path_source_modulus_exponent = 14;
    let res5 = verify_guarded_path_certificate(&serde_json::to_string(&m5).unwrap());
    assert!(res5.is_err());
    mutation_results.push(("lower_source_exponent", res5.is_err()));

    // Mutation 6: Remove an intermediate step
    let mut m6 = base_cert.clone();
    m6.steps.pop();
    let res6 = verify_guarded_path_certificate(&serde_json::to_string(&m6).unwrap());
    assert!(res6.is_err());
    mutation_results.push(("remove_intermediate_step", res6.is_err()));

    // Mutation 7: Change target state base residue
    let mut m7 = base_cert.clone();
    m7.base_state_residue = "11".to_string();
    let res7 = verify_guarded_path_certificate(&serde_json::to_string(&m7).unwrap());
    assert!(res7.is_err());
    mutation_results.push(("change_base_state_residue", res7.is_err()));

    // Mutation 8: Change execution semantics
    let mut m8 = base_cert.clone();
    m8.execution_semantics = "right_to_left_v1".to_string();
    let res8 = verify_guarded_path_certificate(&serde_json::to_string(&m8).unwrap());
    assert!(res8.is_err());
    mutation_results.push(("change_execution_semantics", res8.is_err()));

    // Mutation 9: Replace guarded path cylinder with broader exact word cylinder (1767 mod 16384)
    let mut m9 = base_cert.clone();
    m9.path_source_residue = "1767".to_string();
    m9.path_source_modulus_exponent = 14;
    let res9 = verify_guarded_path_certificate(&serde_json::to_string(&m9).unwrap());
    assert!(res9.is_err());
    mutation_results.push(("replace_with_exact_word_cylinder", res9.is_err()));

    // Export mutation test report
    let report = serde_json::json!({
        "total_mutations": mutation_results.len(),
        "all_rejected": mutation_results.iter().all(|(_, rejected)| *rejected),
        "results": mutation_results.iter().map(|(name, rejected)| serde_json::json!({ "mutation": name, "rejected": rejected })).collect::<Vec<_>>()
    });

    let dir = Path::new("artifacts/phase73_0");
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("mutation_test_report.json"),
        serde_json::to_string_pretty(&report).unwrap(),
    )
    .unwrap();
}
