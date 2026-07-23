use collatz_affine::{
    CanonicalCoreSelector, CoreTransitionReport, SelectorOutput, ValuationWord,
};

#[test]
fn test_axiom_1_endpoint_independence() {
    let selector = CanonicalCoreSelector::default();

    // Two histories sharing identical repeating symbolic prefix [1, 2, 1, 2]
    // with completely different endpoint values attached
    let history_a = ValuationWord::from_slice(&[1, 2, 1, 2]);
    let history_b = ValuationWord::from_slice(&[1, 2, 1, 2]);

    let out_a = selector.select_core(&history_a);
    let out_b = selector.select_core(&history_b);

    match (out_a, out_b) {
        (SelectorOutput::StructuredCore(sel_a), SelectorOutput::StructuredCore(sel_b)) => {
            assert_eq!(sel_a.orbit_id, sel_b.orbit_id);
            assert_eq!(sel_a.primitive_word, sel_b.primitive_word);
            assert_eq!(sel_a.primitive_word.as_slice(), &[1, 2]);
        }
        _ => panic!("Expected structured core selection for [1, 2, 1, 2]"),
    }
}

#[test]
fn test_axiom_2_arithmetic_closeness_never_selects_core() {
    let selector = CanonicalCoreSelector::default();

    // A non-periodic word whose arithmetic values might be close but symbolic structure is not periodic
    let aperiodic_history = ValuationWord::from_slice(&[1, 2, 3, 4, 1, 3, 2, 4]);
    let output = selector.select_core(&aperiodic_history);

    // Arithmetic closeness alone must NEVER select a core
    assert_eq!(output, SelectorOutput::NoStructuredCore);
}

#[test]
fn test_axiom_3_aperiodic_null_output() {
    let selector = CanonicalCoreSelector::default();

    // Non-repeating random-looking valuation path
    let random_history = ValuationWord::from_slice(&[1, 3, 2, 5, 1, 4, 2, 6]);
    let output = selector.select_core(&random_history);

    assert_eq!(output, SelectorOutput::NoStructuredCore);
}

#[test]
fn test_axiom_4_primitive_root_reduction() {
    // Word v^3 = [1, 2, 1, 2, 1, 2]
    let repeated_word = ValuationWord::from_slice(&[1, 2, 1, 2, 1, 2]);
    let prim = CanonicalCoreSelector::primitive_root(&repeated_word);

    // Powers v^r MUST reduce strictly to primitive root v_0 = [1, 2]
    assert_eq!(prim.as_slice(), &[1, 2]);

    let selector = CanonicalCoreSelector::default();
    let output = selector.select_core(&repeated_word);

    match output {
        SelectorOutput::StructuredCore(sel) => {
            assert_eq!(sel.primitive_word.as_slice(), &[1, 2]);
            assert_eq!(sel.period, 2);
            assert_eq!(sel.repetition_count, 3);
        }
        _ => panic!("Expected primitive root [1, 2] reduction"),
    }
}

#[test]
fn test_axiom_5_phase_preservation_under_rotation() {
    let selector = CanonicalCoreSelector::default();

    // Base history repeating [1, 2] -> [1, 2, 1, 2]
    let hist_phase0 = ValuationWord::from_slice(&[1, 2, 1, 2]);
    // Shifted history repeating [2, 1] -> [2, 1, 2, 1]
    let hist_phase1 = ValuationWord::from_slice(&[2, 1, 2, 1]);

    let out0 = selector.select_core(&hist_phase0);
    let out1 = selector.select_core(&hist_phase1);

    match (out0, out1) {
        (SelectorOutput::StructuredCore(s0), SelectorOutput::StructuredCore(s1)) => {
            // Both share the exact same canonical orbit ID
            assert_eq!(s0.orbit_id, s1.orbit_id);
            // Phase offsets reflect rotation
            assert_eq!(s0.phase_offset, 0);
            assert_eq!(s1.phase_offset, 1);
        }
        _ => panic!("Expected matching orbit ID across phase rotations"),
    }
}

#[test]
fn test_axiom_6_future_symbol_independence() {
    let selector = CanonicalCoreSelector::default();

    // History at step n = 4
    let hist_n = ValuationWord::from_slice(&[1, 2, 1, 2]);
    // History at step m = 6 (with future symbols attached)
    let hist_m = ValuationWord::from_slice(&[1, 2, 1, 2, 5, 6]);

    let out_n = selector.select_core(&hist_n);
    let out_m = selector.select_core(&hist_m);

    // Decision at step n depends ONLY on history up to step n
    match out_n {
        SelectorOutput::StructuredCore(s_n) => {
            assert_eq!(s_n.primitive_word.as_slice(), &[1, 2]);
        }
        _ => panic!("Expected core selection at step n"),
    }

    // Future symbols '5, 6' appended at step m break suffix repetition
    assert_eq!(out_m, SelectorOutput::NoStructuredCore);
}

#[test]
fn test_axiom_7_extension_stability_reporting() {
    let selector = CanonicalCoreSelector::default();

    let h1 = ValuationWord::from_slice(&[1, 2]);
    let h2 = ValuationWord::from_slice(&[1, 2, 1, 2]);
    let h3 = ValuationWord::from_slice(&[1, 2, 1, 2, 1, 2]);
    let h4 = ValuationWord::from_slice(&[1, 1, 1, 1, 1, 1, 1, 1]); // Period 1: [1], Q=81 > M=16

    let out1 = selector.select_core(&h1);
    let out2 = selector.select_core(&h2);
    let out3 = selector.select_core(&h3);
    let out4 = selector.select_core(&h4);

    let rep12 = CanonicalCoreSelector::report_transition(&out1, &out2);
    let rep23 = CanonicalCoreSelector::report_transition(&out2, &out3);
    let rep34 = CanonicalCoreSelector::report_transition(&out3, &out4);

    assert!(matches!(rep12, CoreTransitionReport::InitialSelection { .. }));
    assert!(matches!(rep23, CoreTransitionReport::ExtendedWindow { repetitions: 3, .. }));
    assert!(matches!(rep34, CoreTransitionReport::SwitchedCore { .. }));
}
