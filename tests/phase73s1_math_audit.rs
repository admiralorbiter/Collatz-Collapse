use collatz_cegar::{
    AdversarialCorpus, ExtremalSearchConfig, ExtremalSourceSearchEngine, PeriodicGhostAtlas,
};
use num_bigint::BigUint;
use num_traits::Zero;

#[test]
fn test_seed_corpus_and_serialization() {
    let corpus = AdversarialCorpus::seed_corpus();
    assert_eq!(corpus.records.len(), 2);

    let temp_path = "target/debug/test_adversarial_corpus.json";
    corpus.save_to_json(temp_path).expect("Failed to save corpus JSON");

    let loaded = AdversarialCorpus::load_from_json(temp_path).expect("Failed to load corpus JSON");
    assert_eq!(loaded.records.len(), 2);
    assert_eq!(loaded.records[0].source_residue, "200534");
}

#[test]
fn test_mandatory_orientation_identity_and_beta_j_table() {
    let (m0, q0, c0, d0, beta0) = ExtremalSourceSearchEngine::branch_parameters_j(0);
    assert_eq!(m0.to_string(), "512");
    assert_eq!(q0.to_string(), "729");
    assert_eq!(c0.to_string(), "342");
    assert_eq!(d0.to_string(), "487");
    assert_eq!(beta0.to_string(), "26");

    let (m1, q1, c1, d1, beta1) = ExtremalSourceSearchEngine::branch_parameters_j(1);
    assert_eq!(m1.to_string(), "8192");
    assert_eq!(q1.to_string(), "19683");
    assert_eq!(c1.to_string(), "7392");
    assert_eq!(d1.to_string(), "17761");
    assert_eq!(beta1.to_string(), "1376");

    let (m2, q2, c2, d2, beta2) = ExtremalSourceSearchEngine::branch_parameters_j(2);
    assert_eq!(m2.to_string(), "131072");
    assert_eq!(q2.to_string(), "531441");
    assert_eq!(c2.to_string(), "86208");
    assert_eq!(d2.to_string(), "349537");
    assert_eq!(beta2.to_string(), "47936");
}

#[test]
fn test_canonical_source_residue_regressions_and_structural_invariants() {
    let word_0 = ExtremalSourceSearchEngine::base_guarded_word(0);
    assert_eq!(word_0.source_residue.to_string(), "342");
    assert_eq!(word_0.affine.intercept.to_string(), "26");
    assert!(word_0.verify_structural_invariants());

    let word_1 = ExtremalSourceSearchEngine::base_guarded_word(1);
    assert_eq!(word_1.source_residue.to_string(), "7392");
    assert_eq!(word_1.affine.intercept.to_string(), "1376");
    assert!(word_1.verify_structural_invariants());

    let word_2 = ExtremalSourceSearchEngine::base_guarded_word(2);
    assert_eq!(word_2.source_residue.to_string(), "86208");
    assert_eq!(word_2.affine.intercept.to_string(), "47936");
    assert!(word_2.verify_structural_invariants());

    let word_00 = ExtremalSourceSearchEngine::extend_guarded_word(&word_0, 0);
    assert_eq!(word_00.source_residue.to_string(), "200534");
    assert_eq!(word_00.affine.intercept.to_string(), "32266");
    assert!(word_00.verify_structural_invariants());

    let word_01 = ExtremalSourceSearchEngine::extend_guarded_word(&word_0, 1);
    assert_eq!(word_01.source_residue.to_string(), "672598");
    assert_eq!(word_01.affine.intercept.to_string(), "1216270");
    assert!(word_01.verify_structural_invariants());

    let word_10 = ExtremalSourceSearchEngine::extend_guarded_word(&word_1, 0);
    assert_eq!(word_10.source_residue.to_string(), "2686176");
    assert_eq!(word_10.affine.intercept.to_string(), "1216096");
    assert!(word_10.verify_structural_invariants());

    // Source residue minimizer MUST choose (0,1) over (1,0)
    assert!(word_01.source_residue < word_10.source_residue);

    let word_02 = ExtremalSourceSearchEngine::extend_guarded_word(&word_0, 2);
    assert_eq!(word_02.source_residue.to_string(), "5768022");
    assert!(word_02.verify_structural_invariants());
}

#[test]
fn test_exact_precision_and_threshold_tables_and_alpha_regressions() {
    let mut corpus = AdversarialCorpus::seed_corpus();
    let config = ExtremalSearchConfig {
        max_accelerated_depth_r: 4,
        max_gap_j: 2,
        max_precision_h: 36,
    };

    let exact_map = ExtremalSourceSearchEngine::run_exact_precision_search(&config);
    assert_eq!(exact_map.get(&9).unwrap().source_residue.to_string(), "342");
    assert_eq!(exact_map.get(&13).unwrap().source_residue.to_string(), "7392");
    assert_eq!(exact_map.get(&17).unwrap().source_residue.to_string(), "86208");
    assert_eq!(exact_map.get(&18).unwrap().source_residue.to_string(), "200534");
    assert_eq!(exact_map.get(&22).unwrap().source_residue.to_string(), "672598");
    assert_eq!(exact_map.get(&26).unwrap().source_residue.to_string(), "5768022");

    let results = ExtremalSourceSearchEngine::run_search(&config, &mut corpus);
    assert!(!results.is_empty());

    let first = &results[0];
    assert_eq!(first.min_source_z, "342");

    // Direct regression assertions for B=9, M=342
    assert!((first.growth_density_alpha - 0.935316).abs() < 1e-4);
    assert!((first.bits_per_source_bit - 1.069156).abs() < 1e-4);

    let prod = first.growth_density_alpha * first.bits_per_source_bit;
    assert!((prod - 1.0).abs() < 1e-4);
}

#[test]
fn test_mixed_word_ghost_hand_calculation_0_1() {
    let (num, denom) = PeriodicGhostAtlas::fixed_point(&[0, 1]);
    assert_eq!(num.to_string(), "1216270");
    assert_eq!(denom.to_string(), "-10154603");

    assert!(PeriodicGhostAtlas::verify_guarded_domain(&[0, 1]));

    let z_r1 = PeriodicGhostAtlas::positive_representative(&[0, 1], 1);
    assert!(z_r1 > BigUint::zero());
}

#[test]
fn test_periodic_ghost_orbit_atlas_and_guarded_domain() {
    let mut corpus = AdversarialCorpus::seed_corpus();
    let entries = PeriodicGhostAtlas::build_atlas(2, 1, &mut corpus);
    assert!(!entries.is_empty());

    let j0_entry = entries
        .iter()
        .find(|e| e.gap_sequence == vec![0])
        .expect("j=0 ghost entry missing");

    assert_eq!(j0_entry.fixed_point_numerator, "26");
    assert_eq!(j0_entry.fixed_point_denominator, "-217");
    assert!(j0_entry.fixed_point_is_negative);
    assert!(j0_entry.guarded_domain_valid);

    assert_eq!(j0_entry.min_positive_rep_r1, "200534");
    assert_eq!(j0_entry.min_positive_rep_r3, "23750971222");

    let z_witness = BigUint::from(200534u64);
    let shadow_res = PeriodicGhostAtlas::evaluate_shadow_metrics("test_200534", &z_witness, &[0]);
    assert_eq!(shadow_res.closeness_v2, 19);
    assert_eq!(shadow_res.exact_repetitions, 2);
}

#[test]
fn test_print_experimental_audit_tables() {
    let mut corpus = AdversarialCorpus::seed_corpus();
    let config = ExtremalSearchConfig {
        max_accelerated_depth_r: 4,
        max_gap_j: 2,
        max_precision_h: 36,
    };

    let exact_map = ExtremalSourceSearchEngine::run_exact_precision_search(&config);
    println!("\n=== TABLE 1: EXACT PRECISION MINIMA E_{{H,J}}(b) (H=36, J=2) ===");
    println!("| Precision b | E_{{H,J}}(b) (Source \\rho_w) | \\beta_w Intercept | Winning Gap Sequence |");
    println!("|-------------|----------------------------|------------------|----------------------|");
    for (&b, word) in &exact_map {
        println!(
            "| {} | {} | {} | {:?} |",
            b, word.source_residue, word.affine.intercept, word.gap_sequence
        );
    }

    let results = ExtremalSourceSearchEngine::run_search(&config, &mut corpus);
    println!("\n=== TABLE 2: THRESHOLD MINIMA M_{{H,J}}(B) (H=36, J=2) ===");
    println!("| Target B | Actual B_s | M_{{H,J}}(B) | k-Coordinate | Gap Sequence | alpha(B) = log2(M)/B | alpha_witness | Reciprocal |");
    println!("|----------|------------|--------------|--------------|--------------|----------------------|---------------|------------|");
    for res in &results {
        println!(
            "| {} | {} | {} | {} | {:?} | {:.4} | {:.4} | {:.4} |",
            res.precision_b,
            res.actual_b_s,
            res.min_source_z,
            res.min_source_k,
            res.minimizing_gap_sequence,
            res.growth_density_alpha,
            res.alpha_witness,
            res.bits_per_source_bit
        );
    }
}
