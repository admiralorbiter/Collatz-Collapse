use collatz_affine::SymbolicLanguageEnumerator;
use collatz_cert::schema::{Phase73cVerificationReportJson, SymbolicWordClassificationJson};
use collatz_cert::verify_phase73c_report;
use serde_json::json;
use std::fs;
use std::path::Path;

fn build_valid_phase73c_report() -> Phase73cVerificationReportJson {
    let words_data = SymbolicLanguageEnumerator::enumerate(2).unwrap();

    let mut json_words = Vec::new();
    for w in &words_data {
        let lift_str = w.parent_lift_digit.as_ref().map(|d| d.to_string()).unwrap_or_else(|| "0".to_string());
        let is_zero = w.is_zero_lift_from_parent.unwrap_or(true);
        json_words.push(SymbolicWordClassificationJson {
            schema_version: "symbolic_word_classification_v1".to_string(),
            valuation_word: w.word.elements().to_vec(),
            eta: w.eta.to_string(),
            guard_residue: w.guard_residue.to_string(),
            guard_modulus_exponent: w.guard_modulus_exponent,
            least_source_n: w.least_source_n.to_string(),
            is_zero_lift: is_zero,
            lift_digit: lift_str,
            primitive_root: w.primitive_root.elements().to_vec(),
            repetition_count: w.repetition_count,
        });
    }

    Phase73cVerificationReportJson {
        schema_version: "phase73c_verification_report_v1".to_string(),
        total_nonempty_words: json_words.len(),
        word_classifications: json_words,
        all_guards_cross_validated: true,
    }
}

#[test]
fn test_phase73c_roundtrip_verification() {
    let valid_report = build_valid_phase73c_report();
    let json_str = serde_json::to_string_pretty(&valid_report).unwrap();
    assert!(verify_phase73c_report(&json_str).is_ok());

    let artifact_dir = Path::new("artifacts/phase73c");
    let _ = fs::create_dir_all(artifact_dir);

    // Export rust_symbolic_results.json
    fs::write(artifact_dir.join("rust_symbolic_results.json"), &json_str).unwrap();
}

#[test]
#[allow(clippy::type_complexity)]
fn test_10_corruption_mutation_matrix_phase73c() {
    let valid_report = build_valid_phase73c_report();
    let mut mutation_results = Vec::new();

    let mutations: Vec<(&str, Box<dyn Fn(&mut Phase73cVerificationReportJson)>)> = vec![
        (
            "Corrupt word valuation element",
            Box::new(|r| r.word_classifications[0].valuation_word[0] = 99),
        ),
        (
            "Corrupt eta constant",
            Box::new(|r| r.word_classifications[0].eta = "9999".to_string()),
        ),
        (
            "Corrupt guard_residue",
            Box::new(|r| r.word_classifications[0].guard_residue = "9999".to_string()),
        ),
        (
            "Corrupt guard_modulus_exponent",
            Box::new(|r| r.word_classifications[0].guard_modulus_exponent = 99),
        ),
        (
            "Corrupt least_source_n",
            Box::new(|r| r.word_classifications[0].least_source_n = "9999".to_string()),
        ),
        (
            "Corrupt is_zero_lift flag",
            Box::new(|r| {
                r.word_classifications[0].is_zero_lift = !r.word_classifications[0].is_zero_lift
            }),
        ),
        (
            "Corrupt lift_digit string",
            Box::new(|r| r.word_classifications[0].lift_digit = "9999".to_string()),
        ),
        (
            "Corrupt total_nonempty_words count",
            Box::new(|r| r.total_nonempty_words = 999),
        ),
        (
            "Corrupt schema_version",
            Box::new(|r| r.schema_version = "corrupted_v1".to_string()),
        ),
        (
            "Set all_guards_cross_validated to false",
            Box::new(|r| r.all_guards_cross_validated = false),
        ),
    ];

    let mut rejected_count = 0;
    for (name, mutator) in &mutations {
        let mut mutated = valid_report.clone();
        mutator(&mut mutated);
        let str_mut = serde_json::to_string(&mutated).unwrap();

        let is_rejected = verify_phase73c_report(&str_mut).is_err();
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

    let artifact_dir = Path::new("artifacts/phase73c");
    let _ = fs::create_dir_all(artifact_dir);

    let mutation_report = json!({
        "schema_version": "mutation_test_report_phase73c_v1",
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
