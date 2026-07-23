use collatz_cegar::accelerated_branch_params::AcceleratedBranchParams;
use collatz_cegar::adaptive_stress_engine::AdaptiveStressEngine;
use collatz_cegar::backward_fixed_point_probe::BackwardFixedPointProbe;
use collatz_cegar::extremal_source_search::ExtremalSourceSearchEngine;
use collatz_cegar::periodic_ghost_atlas::PeriodicGhostAtlas;
use collatz_cegar::precision_aware_cylinder::Cylinder;
use collatz_cegar::zero_lift_cegar_engine::{CegarOutcome, ZeroLiftCegarEngine};
use collatz_cegar::zero_lift_endpoint_graph::ZeroLiftEndpointGraph;
use collatz_cegar::zero_output_scc_probe::{TransducerOutcome, ZeroOutputSccProbe};
use collatz_cegar::zero_tail_stress_audit::ZeroTailProfile;
use num_bigint::BigUint;

#[test]
fn test_unconditional_freeze_gate_direct_return_matrix() {
    for j in 0..=32 {
        let p = AcceleratedBranchParams::for_gap(j);
        assert!(p.verify_invariants(), "Invariants failed for j={}", j);
        assert_eq!(
            AcceleratedBranchParams::exact_successor_gap(&p.z_source_residue),
            Some(j),
            "Successor gap gate failed for j={}",
            j
        );
        let d_sim = p.direct_original_gap_return(&p.z_source_residue);
        assert_eq!(d_sim, p.z_endpoint, "Direct return gate failed for j={}", j);
    }
}

#[test]
fn test_exact_cylinder_calculus_round_trip_theorem() {
    let targets = vec![
        Cylinder::new(BigUint::from(0u64), 5),
        Cylinder::new(BigUint::from(487u64), 5),
        Cylinder::new(BigUint::from(17761u64), 9),
    ];

    for target in &targets {
        for j in 0..=3 {
            let pred = Cylinder::pre_j(target, j);
            let succ = pred.post_j(j).expect("Post_j should succeed on Pre_j output");
            assert_eq!(succ.precision, target.precision);
            assert_eq!(succ.residue, target.residue);
        }
    }
}

#[test]
fn test_coherent_canonical_shift_mutation() {
    for j in 0..=5 {
        let p = AcceleratedBranchParams::for_gap(j);
        let c_corrupt = &p.z_source_residue + &p.modulus;
        let d_corrupt = &p.z_endpoint + &p.multiplier;

        // Mutation satisfies branch cylinder and affine identity, but violates canonical residue bound 0 <= C_j < M_j
        assert!(c_corrupt >= p.modulus);
        let direct_ret = p.direct_original_gap_return(&c_corrupt);
        assert_eq!(direct_ret, d_corrupt);
    }
}

#[test]
fn test_eventual_zero_endpoint_reduction_lemma_genuine_prefix() {
    let graph = ZeroLiftEndpointGraph::new(10);
    
    // Genuine canonical prefix u = (0, 0, 7)
    let w0 = ExtremalSourceSearchEngine::base_guarded_word(0);
    let w00 = ExtremalSourceSearchEngine::extend_guarded_word(&w0, 0);
    let w007 = ExtremalSourceSearchEngine::extend_guarded_word(&w00, 7);
    
    let d_u = &w007.endpoint;
    assert_eq!(
        d_u,
        &"2487743142969238870".parse::<BigUint>().unwrap()
    );
    
    // Extension by j = 0 produces true zero lift
    assert!(graph.is_zero_lift(d_u, 0));
    
    let d_u0_calculated = graph.zero_lift_successor(d_u, 0);
    let w0070 = ExtremalSourceSearchEngine::extend_guarded_word(&w007, 0);
    assert_eq!(d_u0_calculated, w0070.endpoint);
    assert_eq!(
        d_u0_calculated,
        "3542118654735498313".parse::<BigUint>().unwrap()
    );
}

#[test]
fn test_zero_lift_cegar_engine_probe() {
    let cegar = ZeroLiftCegarEngine::new(9, 4);
    let (outcome, nodes, _edges) = cegar.run_cegar_probe();
    assert!(nodes > 0);
    assert_eq!(outcome, CegarOutcome::AbstractionRedesignRequired);
}

#[test]
fn test_perfect_authoritative_branch_parameter_regressions() {
    // Exact user table vectors for j=0..5
    let expected_records = vec![
        (0u64, 179u64, 255u64, 7u8, 342u64, 487u64, 26i64),
        (1u64, 7585u64, 18225u64, 9u8, 7392u64, 17761u64, 1376i64),
        (2u64, 30785u64, 124821u64, 7u8, 86208u64, 349537u64, 47936i64),
        (3u64, 529985u64, 3626208u64, 9u8, 1764032u64, 12069670u64, 1466816i64),
        (4u64, 28554817u64, 329694786u64, 4u8, 14797504u64, 170852431u64, 42364736i64),
        (5u64, 495040065u64, 9645324075u64, 7u8, 386648768u64, 7533436045u64, 1188019136i64),
    ];

    for (j, c_exp, d_exp, mu_exp, c_z_exp, d_z_exp, beta_exp) in expected_records {
        let p = AcceleratedBranchParams::for_gap(j);
        assert!(p.verify_invariants());
        assert_eq!(p.t_coordinate_residue, BigUint::from(c_exp));
        assert_eq!(p.t_coordinate_endpoint, BigUint::from(d_exp));
        assert_eq!(p.mu_mod_11, mu_exp);
        assert_eq!(p.z_source_residue, BigUint::from(c_z_exp));
        assert_eq!(p.z_endpoint, BigUint::from(d_z_exp));
        assert_eq!(p.affine_intercept, num_bigint::BigInt::from(beta_exp));
    }
}

#[test]
fn test_eventually_periodic_ghost_source_density_bound() {
    let atlas = PeriodicGhostAtlas::new(vec![0]);
    let profile = atlas.eventually_periodic_ghost(&[1, 2, 4]);
    assert!(profile.total_high_zero_bits <= 5);
    assert!(profile.real_drift_ratio > 0.0);
}

#[test]
fn test_zero_output_scc_probe_classification() {
    let probe = ZeroOutputSccProbe::new(1, 8);
    let (outcome, nodes, _edges) = probe.analyze_zero_output_subgraph();
    assert!(nodes > 0);
    assert!(
        outcome == TransducerOutcome::FiniteEventualZeroQuotientFound
            || outcome == TransducerOutcome::ZeroOutputSccsClassified
    );
}

#[test]
fn test_single_symbol_canonical_word_identity_for_all_gaps() {
    for j in 0..=32 {
        let p = AcceleratedBranchParams::for_gap(j);
        let word = ExtremalSourceSearchEngine::base_guarded_word(j);
        assert_eq!(word.source_residue, p.z_source_residue, "Word source mismatch for j={}", j);
        assert_eq!(word.endpoint, p.z_endpoint, "Word endpoint mismatch for j={}", j);
    }
}

#[test]
fn test_perfect_two_gap_word_regressions() {
    // Vector 1: (5, 0) -> rho = 79,306,672,832, Lambda = [386648768, 147], B=38, ell=37, Z=1
    let w50 = ExtremalSourceSearchEngine::sequence_guarded_word(&[5, 0]);
    assert_eq!(w50.source_residue, BigUint::from(79306672832u64));
    let p50 = ZeroTailProfile::from_canonical_word(&w50);
    assert_eq!(p50.precision_bits, 38);
    assert_eq!(p50.source_bit_length, 37);
    assert_eq!(p50.total_high_zero_bits, 1);

    // Vector 2: (3, 5) -> rho = 889,509,904,050,880, Lambda = [1764032, 424151374], B=50, ell=50, Z=0
    let w35 = ExtremalSourceSearchEngine::sequence_guarded_word(&[3, 5]);
    assert_eq!(w35.source_residue, BigUint::from(889509904050880u64));
    let p35 = ZeroTailProfile::from_canonical_word(&w35);
    assert_eq!(p35.precision_bits, 50);
    assert_eq!(p35.source_bit_length, 50);
    assert_eq!(p35.total_high_zero_bits, 0);

    // Vector 3: (6, 2) -> rho = 3,175,741,115,072, Lambda = [6055250624, 369], B=50, ell=42, Z=8
    let w62 = ExtremalSourceSearchEngine::sequence_guarded_word(&[6, 2]);
    assert_eq!(w62.source_residue, BigUint::from(3175741115072u64));
    let p62 = ZeroTailProfile::from_canonical_word(&w62);
    assert_eq!(p62.precision_bits, 50);
    assert_eq!(p62.source_bit_length, 42);
    assert_eq!(p62.total_high_zero_bits, 8);

    // Vector 4: (13, 10) -> rho = 329,546,401,545,354,289,080,164,808,379,072, B=110, ell=109, Z=1
    let w13_10 = ExtremalSourceSearchEngine::sequence_guarded_word(&[13, 10]);
    assert_eq!(
        w13_10.source_residue,
        "329546401545354289080164808379072".parse::<BigUint>().unwrap()
    );
    let p13_10 = ZeroTailProfile::from_canonical_word(&w13_10);
    assert_eq!(p13_10.precision_bits, 110);
    assert_eq!(p13_10.source_bit_length, 109);
    assert_eq!(p13_10.total_high_zero_bits, 1);
}

#[test]
fn test_ghost_orbit_control_group_tail_bound() {
    let base_0 = ExtremalSourceSearchEngine::base_guarded_word(0);
    let mut w_rep = base_0.clone();

    for r in 1..=4 {
        let profile = ZeroTailProfile::from_canonical_word(&w_rep);
        assert!(
            profile.total_high_zero_bits <= 9,
            "Periodic ghost v repetition r={} exceeded bounded zero tail! Z={}",
            r,
            profile.total_high_zero_bits
        );
        w_rep = ExtremalSourceSearchEngine::extend_guarded_word(&w_rep, 0);
    }
}

#[test]
fn test_large_gap_scan_and_adaptive_beam_search() {
    let report = AdaptiveStressEngine::run_large_gap_scan(12);
    assert!(report.zeta_max_1_gap < 0.1);

    let beam = AdaptiveStressEngine::run_adaptive_beam_search(8, 50, 4);
    assert!(!beam.is_empty());
    assert!(beam.len() <= 50);

    for (_word, profile) in &beam {
        assert!(profile.verify_lift_block_invariants());
    }
}
