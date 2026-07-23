use collatz_affine::{
    CanonicalCoreSelector, DeterministicBenchmarkGenerators, FactorComplexityAnalyzer,
    SturmianCubeAnalyzer, SubstitutivePotentialFunction,
};
use std::collections::HashMap;

#[test]
fn test_fibonacci_sturmian_factor_complexity_and_cubes() {
    let fib_word = DeterministicBenchmarkGenerators::fibonacci_word(100);

    // Sturmian property: p(n) = n + 1 for all n <= 10
    for n in 1..=10 {
        let p_n = FactorComplexityAnalyzer::factor_complexity(&fib_word, n);
        assert_eq!(p_n, n + 1, "Fibonacci word must satisfy Sturmian factor complexity p(n) = n + 1");
    }

    // Sturmian cube occurrence gap test
    let cube_gap = SturmianCubeAnalyzer::max_cube_gap_distance(&fib_word, 4);
    assert!(cube_gap.is_some(), "Fibonacci word must exhibit bounded-gap cube occurrences");
}

#[test]
fn test_silver_ratio_sturmian_word() {
    let silver_word = DeterministicBenchmarkGenerators::silver_ratio_word(100);

    // Silver ratio Sturmian factor complexity p(n) = n + 1
    for n in 1..=8 {
        let p_n = FactorComplexityAnalyzer::factor_complexity(&silver_word, n);
        assert_eq!(p_n, n + 1, "Silver ratio word must satisfy p(n) = n + 1");
    }
}

#[test]
fn test_substitutive_potential_function_certificate() {
    // 3-node directed graph representing substitutive return-word transitions:
    // Edge (0, 1, w=-3), Edge (1, 2, w=-2), Edge (2, 0, w=-4)
    let nodes = vec![0, 1, 2];
    let edges = vec![(0, 1, -3), (1, 2, -2), (2, 0, -4)];

    // Potential function assignment \Phi(0)=0, \Phi(1)=2, \Phi(2)=3
    let mut potential = HashMap::new();
    potential.insert(0, 0);
    potential.insert(1, 2);
    potential.insert(2, 3);

    // Verify w(e) + \Phi(t) - \Phi(s) <= -1
    // (0,1): -3 + 2 - 0 = -1 <= -1 (OK)
    // (1,2): -2 + 3 - 2 = -1 <= -1 (OK)
    // (2,0): -4 + 0 - 3 = -7 <= -1 (OK)
    let valid = SubstitutivePotentialFunction::verify_potential_certificate(
        &nodes, &edges, &potential, 1,
    );
    assert!(valid, "Potential certificate must verify strictly negative cycle mean");
}

#[test]
fn test_multiscale_coverage_and_adversarial_defect() {
    let selector = CanonicalCoreSelector::default();
    let defect_word = DeterministicBenchmarkGenerators::adversarial_periodic_defect_word(64);

    let metrics = FactorComplexityAnalyzer::evaluate_multiscale_coverage(&selector, &defect_word, 4);

    assert_eq!(metrics.total_length, 64);
    assert!(metrics.covered_positions > 0);
}
