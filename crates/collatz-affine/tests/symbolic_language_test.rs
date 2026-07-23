use collatz_affine::SymbolicLanguageEnumerator;
use collatz_cegar::{LiftDigitEngine, PeriodicNecklaceAnalyzer, TopologicalEntropyEngine};

#[test]
fn test_symbolic_language_enumeration_8190_words() {
    // Depth 1..=12 => sum_{r=1}^{12} 2^r = 2^13 - 2 = 8190 non-empty words
    let words = SymbolicLanguageEnumerator::enumerate(12).unwrap();
    assert_eq!(words.len(), 8190);

    // Cross-validate 3 guard methods for all 8,190 words
    for w in &words {
        assert!(
            SymbolicLanguageEnumerator::cross_validate_guards(w).unwrap(),
            "Cross validation failed for word {:?}",
            w.word
        );
    }
}

#[test]
fn test_lift_digit_engine_and_zero_lift_chains() {
    let words = SymbolicLanguageEnumerator::enumerate(4).unwrap();
    let (chains, stats) = LiftDigitEngine::analyze_zero_lift_chains(&words);

    // Check that child depths have zero-lift stats reported
    assert!(stats.contains_key(&2));
    assert!(stats.contains_key(&3));
    assert!(stats.contains_key(&4));

    // Zero lift chains found
    assert!(!chains.is_empty());
}

#[test]
fn test_topological_entropy_and_dual_haar_measures() {
    // Depth r=3
    let (mu_k, mu_n) = TopologicalEntropyEngine::compute_dual_haar_measures(3);
    let expected_mu_k = (33.0f64 / 512.0f64).powi(3);
    let expected_mu_n = (2.0f64.powi(-5)) * expected_mu_k;

    assert!((mu_k - expected_mu_k).abs() < 1e-12);
    assert!((mu_n - expected_mu_n).abs() < 1e-12);

    let hist = TopologicalEntropyEngine::compute_histogram(3);
    assert_eq!(hist.len(), 4); // j = 0, 1, 2, 3
    let total_combinations: u64 = hist.iter().map(|h| h.word_count_combinations).sum();
    assert_eq!(total_combinations, 8); // 2^3 = 8
}

#[test]
fn test_periodic_necklace_fixed_points() {
    let words = SymbolicLanguageEnumerator::enumerate(2).unwrap();
    let necklaces = PeriodicNecklaceAnalyzer::extract_primitive_necklaces(&words).unwrap();

    // Primitive necklaces for depth 2: u, v, uv (with phases uv and vu)
    assert_eq!(necklaces.len(), 3);

    for neck in &necklaces {
        for rot in &neck.rotations {
            // Fixed point denominator 2^A_w - a_w < 0
            assert!(rot.fixed_point_den < num_bigint::BigInt::from(0));
        }
    }
}
