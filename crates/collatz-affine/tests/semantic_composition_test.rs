use collatz_affine::{ExactWordCylinder, ThenSequence, ValuationWord};
use collatz_cegar::{GuardedPathCylinder, StateMembership};
use num_bigint::BigUint;
use std::fs;
use std::path::Path;

#[test]
fn test_group_a_exact_valuation_word_semantics() {
    let u = ValuationWord::from_slice(&[1, 1, 2]);
    let v = ValuationWord::from_slice(&[1, 1, 2, 1, 2, 2]);

    let seq_uv = u.clone().then(v.clone());
    let seq_vu = v.clone().then(u.clone());

    let exact_uv = ExactWordCylinder::from_valuation_word(seq_uv.flatten_valuation_word()).unwrap();
    let exact_vu = ExactWordCylinder::from_valuation_word(seq_vu.flatten_valuation_word()).unwrap();

    let q1 = StateMembership::q1();
    let guarded_uv = GuardedPathCylinder::compute(seq_uv.clone(), q1.clone()).unwrap();
    let guarded_vu = GuardedPathCylinder::compute(seq_vu.clone(), q1).unwrap();

    // Headline exact valuation-word cylinders
    assert_eq!(exact_uv.source.residue, BigUint::from(1767u32));
    assert_eq!(exact_uv.source.modulus_exponent, 14); // mod 16384

    assert_eq!(exact_vu.source.residue, BigUint::from(1959u32));
    assert_eq!(exact_vu.source.modulus_exponent, 14); // mod 16384

    // Composite affine map constants
    let pref_uv = seq_uv.combined_affine_prefix().unwrap();
    let pref_vu = seq_vu.combined_affine_prefix().unwrap();

    assert_eq!(pref_uv.constant, BigUint::from(27947u32));
    assert_eq!(pref_vu.constant, BigUint::from(33515u32));
    assert_eq!(pref_uv.total_twos, 13);
    assert_eq!(pref_vu.total_twos, 13);

    // Constant difference matches commutator identity: 33515 - 27947 = 5568 = -\Delta_{u,v}
    let diff = &pref_vu.constant - &pref_uv.constant;
    assert_eq!(diff, BigUint::from(5568u32));

    // Export rust_semantic_results.json artifact
    let rust_output = serde_json::json!({
        "execution_semantics": "left_to_right_v1",
        "sequence_uv": {
            "flattened_word": [1, 1, 2, 1, 1, 2, 1, 2, 2],
            "affine_form": {"k": pref_uv.odd_steps, "total_twos": pref_uv.total_twos, "constant": pref_uv.constant.to_string()},
            "exact_word_cylinder": {"residue": exact_uv.source.residue.to_string(), "modulus_exponent": exact_uv.source.modulus_exponent},
            "guarded_path_cylinder": {"residue": guarded_uv.source.residue.to_string(), "modulus_exponent": guarded_uv.source.modulus_exponent},
        },
        "sequence_vu": {
            "flattened_word": [1, 1, 2, 1, 2, 2, 1, 1, 2],
            "affine_form": {"k": pref_vu.odd_steps, "total_twos": pref_vu.total_twos, "constant": pref_vu.constant.to_string()},
            "exact_word_cylinder": {"residue": exact_vu.source.residue.to_string(), "modulus_exponent": exact_vu.source.modulus_exponent},
            "guarded_path_cylinder": {"residue": guarded_vu.source.residue.to_string(), "modulus_exponent": guarded_vu.source.modulus_exponent},
        },
        "commutator_constant_diff": diff.to_string()
    });

    let dir = Path::new("artifacts/phase73_0");
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("rust_semantic_results.json"),
        serde_json::to_string_pretty(&rust_output).unwrap(),
    )
    .unwrap();
}
