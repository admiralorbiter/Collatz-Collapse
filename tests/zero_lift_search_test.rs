use num_bigint::BigUint;

#[path = "../src/zero_lift_search.rs"]
mod zero_lift_search;

use zero_lift_search::{compute_prefix_signature, search_counterexample_q1_traces, search_orbit_first_zero_lift_runs, ZeroLiftSearchBounds};

#[test]
fn test_bounded_zero_lift_search_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty(), "Zero-lift search should find valid traces for Q1 sources <= 500");
    for trace in &traces {
        assert_eq!(&trace.anchor % 32u32, BigUint::from(7u32));
        assert!(trace.steps.len() >= 2);
    }
}

#[test]
fn test_counterexample_q1_traces_diagnostics() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let ce_traces = search_counterexample_q1_traces(&bounds);
    assert!(!ce_traces.is_empty());
    for trace in &ce_traces {
        assert_eq!(&trace.anchor % 32u32, BigUint::from(7u32));
        assert!(trace.endpoint_modulus3 > BigUint::from(1u32));
        assert!(&trace.endpoint_residue3 < &trace.endpoint_modulus3);
    }
}

#[test]
fn test_prefix_signature_exact_drift_computation() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty());
    let sig = compute_prefix_signature(&traces[0]);
    assert!(sig.step_time > 0);
    assert!(sig.exponent_sum > 0);
    assert!(sig.is_realizable);
}
