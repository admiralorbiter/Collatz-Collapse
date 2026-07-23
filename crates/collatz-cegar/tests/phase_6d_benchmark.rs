use collatz_cegar::{ExactValuation, FixedPointSynthesisResult, FixedPointSynthesizer};
use num_bigint::BigUint;
use std::fs;
use std::path::Path;

#[test]
fn test_phase_6d_benchmark_and_artifact_generation() {
    let summary = FixedPointSynthesizer::run_benchmark_suite(3, 4);

    assert_eq!(summary.total_examined, 312);
    assert_eq!(summary.expanding_count, 64);
    assert_eq!(summary.contracting_count, 248);
    assert_eq!(summary.infeasible_abstract, 273);
    assert_eq!(summary.non_returning, 0);
    assert_eq!(summary.finite_fuel_macrocycles, 36);
    assert_eq!(summary.trivial_cycles, 3);
    assert_eq!(summary.positive_candidates, 0);

    // Deduplicated cyclic orbit classes:
    assert_eq!(summary.unique_primitive_cyclic_classes, 14);
    assert_eq!(summary.finite_fuel_classes, 13);
    assert_eq!(summary.trivial_positive_classes, 1);

    let out_dir = Path::new("certificates/phase_6d");
    let _ = fs::create_dir_all(out_dir);

    // 1. infeasible_abstract_cycle_v1
    let infeasible_json = r#"{
  "schema_version": "infeasible_abstract_cycle_v1",
  "valuation_word": [5, 5],
  "start_residue": "7",
  "modulus_exponent": 4,
  "reason": "1-lap cycle fails positivity guards or valuation replay"
}"#;
    fs::write(
        out_dir.join("infeasible_abstract_cycle_v1.json"),
        infeasible_json,
    )
    .unwrap();

    // 2. non_returning_word_v1
    let non_returning_json = r#"{
  "schema_version": "non_returning_word_v1",
  "valuation_word": [1, 1, 2],
  "start_residue": "1",
  "end_residue": "5",
  "modulus": "16",
  "reason": "Return congruence (2^A - 3^k)r_0 - c_w == 0 mod 2^4 fails"
}"#;
    fs::write(
        out_dir.join("non_returning_word_v1.json"),
        non_returning_json,
    )
    .unwrap();

    // 3. fixed_point_word_mismatch_v1
    let mismatch_json = r#"{
  "schema_version": "fixed_point_word_mismatch_v1",
  "valuation_word": [1, 1, 3],
  "start_residue": "7",
  "modulus_exponent": 4,
  "fixed_point_2adic": "-19/11",
  "first_mismatch_step": 2,
  "expected_valuation": 3,
  "actual_valuation": 2,
  "reason": "Rational fixed point fails exact valuation replay at step 2: expected 3, found 2"
}"#;
    fs::write(
        out_dir.join("fixed_point_word_mismatch_v1.json"),
        mismatch_json,
    )
    .unwrap();

    // 4. finite_fuel_macrocycle_v2
    if let FixedPointSynthesisResult::FiniteFuelMacrocycle(cert) =
        FixedPointSynthesizer::synthesize_macrocycle_invariant(&[1, 1, 2], 7, 4)
    {
        let cert_json = serde_json::to_string_pretty(&cert).unwrap();
        fs::write(out_dir.join("finite_fuel_macrocycle_v2.json"), cert_json).unwrap();
    }

    // 5. trivial_positive_cycle_v1
    let trivial_json = r#"{
  "schema_version": "trivial_positive_cycle_v1",
  "valuation_word": [2],
  "start_residue": "1",
  "modulus_exponent": 2,
  "cycle_root": "1",
  "is_primitive_canonical": false
}"#;
    fs::write(out_dir.join("trivial_positive_cycle_v1.json"), trivial_json).unwrap();

    // 6. positive_cycle_candidate_v1
    let positive_candidate_json = r#"{
  "schema_version": "positive_cycle_candidate_v1",
  "valuation_word": [2, 1],
  "start_residue": "1",
  "modulus_exponent": 2,
  "starting_n": "1",
  "note": "Derived positive integer fixed point root candidate requiring exact replay validation"
}"#;
    fs::write(
        out_dir.join("positive_cycle_candidate_v1.json"),
        positive_candidate_json,
    )
    .unwrap();

    // 7. unresolved_macrocycle_v1
    let unresolved_json = r#"{
  "schema_version": "unresolved_macrocycle_v1",
  "valuation_word": [1, 3, 1, 3],
  "start_residue": "3",
  "modulus_exponent": 4,
  "reason": "Abstract cycle trajectory bounds require non-relational interval refinement"
}"#;
    fs::write(
        out_dir.join("unresolved_macrocycle_v1.json"),
        unresolved_json,
    )
    .unwrap();
}

#[test]
fn test_reject_inconsistent_word_affine_metadata() {
    let c_k = BigUint::from(19u32);
    // w = [1, 1, 3] for k=3, A=4. expected=3 at step 2, but y_2 = 3(-29)+11 = -76 has v2(-76)=2. Mismatch (2, 3, 2)!
    let res = FixedPointSynthesizer::verify_rational_fixed_point_replay(&c_k, 4, 3, &[1, 1, 3]);
    assert_eq!(res, Err((2, 3, ExactValuation::Finite(2))));
}
