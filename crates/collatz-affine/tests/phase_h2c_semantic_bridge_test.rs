use collatz_affine::{
    CanonicalCoreSelector, CoreTransitionReport, PeriodicReturnCore, PrecisionLedger,
    SemanticCoreDistanceBridge, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;

#[test]
fn test_semantic_core_distance_lcp_interval_bound() {
    // Two valuation words with common prefix of length L = 2: [1, 2, 1] vs [1, 2, 2]
    // Word 1: [1, 2, 1, 1, 1, 1] (Q=729, M=128, return block)
    // Word 2: [1, 2, 2, 1, 1, 1] (Q=729, M=256, return block)
    let word_1 = ValuationWord::from_slice(&[1, 2, 1, 1, 1, 1]);
    let word_2 = ValuationWord::from_slice(&[1, 2, 2, 1, 1, 1]);

    let core_1 = PeriodicReturnCore::new(word_1.clone()).unwrap();
    let core_2 = PeriodicReturnCore::new(word_2.clone()).unwrap();

    let l = SemanticCoreDistanceBridge::longest_common_prefix(&word_1, &word_2);
    assert_eq!(l, 2); // Common prefix [1, 2]

    let (h_l, kappa, _h_l_next) =
        SemanticCoreDistanceBridge::verify_weighted_lcp_interval(&core_1, &core_2);

    // H_L = 1 + 2 = 3 valuation units
    assert_eq!(h_l, 3);

    // Core distance kappa = v_2(\Gamma_{v,w}) must satisfy \kappa >= H_L = 3
    match kappa {
        TwoAdicValuation::Finite(k) => assert!(k >= h_l),
        TwoAdicValuation::Infinity => panic!("Expected finite kappa for distinct cores"),
    }
}

#[test]
fn test_positive_ordinary_state_never_exact_core() {
    let word_v = ValuationWord::from_slice(&[1, 2]);
    let core_v = PeriodicReturnCore::new(word_v).unwrap();

    // For any positive integer D > 0, A_v(D) = d_v * D + \beta_v > 0 strictly
    for d in 1..=100u32 {
        let d_big = BigInt::from(d);
        assert!(SemanticCoreDistanceBridge::positive_state_never_exact_core(
            &core_v, &d_big
        ));
    }
}

#[test]
fn test_genuine_incompatible_3_primitive_orbit_fixture() {
    let selector = CanonicalCoreSelector::default();

    // 3 genuinely incompatible primitive periodic orbits (no pair related by rotation or power):
    // Orbit 1: [1, 2] -> Primitive root [1, 2] (Period 2)
    // Orbit 2: [1, 1, 1, 1, 1, 1] -> Primitive root [1] (Period 1)
    // Orbit 3: [1, 1, 2, 1, 1, 2] -> Primitive root [1, 1, 2] (Period 3)
    let w1 = ValuationWord::from_slice(&[1, 2, 1, 2]);
    let w2 = ValuationWord::from_slice(&[1, 1, 1, 1, 1, 1]);
    let w3 = ValuationWord::from_slice(&[1, 1, 2, 1, 1, 2]);

    let out1 = selector.select_core(&w1);
    let out2 = selector.select_core(&w2);
    let out3 = selector.select_core(&w3);

    let (sel1, sel2, sel3) = match (out1, out2, out3) {
        (
            collatz_affine::SelectorOutput::StructuredCore(s1),
            collatz_affine::SelectorOutput::StructuredCore(s2),
            collatz_affine::SelectorOutput::StructuredCore(s3),
        ) => (s1, s2, s3),
        _ => panic!("Expected structured core selection for all 3 words"),
    };

    // 1. Verify 3 distinct primitive words
    assert_eq!(sel1.primitive_word.as_slice(), &[1, 2]);
    assert_eq!(sel2.primitive_word.as_slice(), &[1]);
    assert_eq!(sel3.primitive_word.as_slice(), &[1, 1, 2]);

    // 2. Verify 3 distinct orbit IDs (no rotation equivalence)
    assert_ne!(sel1.orbit_id, sel2.orbit_id);
    assert_ne!(sel2.orbit_id, sel3.orbit_id);
    assert_ne!(sel1.orbit_id, sel3.orbit_id);

    // 3. Verify core transitions emit SwitchedCore across distinct primitive orbits
    let tr12 = CanonicalCoreSelector::report_transition(
        &collatz_affine::SelectorOutput::StructuredCore(sel1.clone()),
        &collatz_affine::SelectorOutput::StructuredCore(sel2.clone()),
    );
    let tr23 = CanonicalCoreSelector::report_transition(
        &collatz_affine::SelectorOutput::StructuredCore(sel2.clone()),
        &collatz_affine::SelectorOutput::StructuredCore(sel3.clone()),
    );

    assert!(matches!(tr12, CoreTransitionReport::SwitchedCore { .. }));
    assert!(matches!(tr23, CoreTransitionReport::SwitchedCore { .. }));

    // 4. Record precision ledger transitions across the 3 distinct primitive cores
    let mut ledger = PrecisionLedger::new(TwoAdicValuation::Finite(12));
    let a_v = BigInt::from(1024u32);

    ledger.record_switch(&sel1.core, &sel2.core, &a_v, 1);
    let s2 = match ledger.entries[0].outgoing_depth_next_s {
        TwoAdicValuation::Finite(s) => s,
        _ => panic!("Expected finite s2"),
    };
    let a_v2 = BigInt::from(1u32) << (s2 as usize);
    ledger.record_switch(&sel2.core, &sel3.core, &a_v2, 1);

    assert_eq!(ledger.entries.len(), 2);
}

#[test]
fn test_dedicated_phase_versus_primitive_orbit_change_fixture() {
    let selector = CanonicalCoreSelector::default();

    // Phase change: [1, 2, 1, 2] (Phase 0) -> [2, 1, 2, 1] (Phase 1)
    // Same primitive orbit [1, 2], different phase offset!
    let w_phase0 = ValuationWord::from_slice(&[1, 2, 1, 2]);
    let w_phase1 = ValuationWord::from_slice(&[2, 1, 2, 1]);

    let out0 = selector.select_core(&w_phase0);
    let out1 = selector.select_core(&w_phase1);

    let (s0, s1) = match (out0, out1) {
        (
            collatz_affine::SelectorOutput::StructuredCore(s0),
            collatz_affine::SelectorOutput::StructuredCore(s1),
        ) => (s0, s1),
        _ => panic!("Expected structured core selection"),
    };

    // Same orbit ID, distinct phase offset!
    assert_eq!(s0.orbit_id, s1.orbit_id);
    assert_eq!(s0.phase_offset, 0);
    assert_eq!(s1.phase_offset, 1);
}
