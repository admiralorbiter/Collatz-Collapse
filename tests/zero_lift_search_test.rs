use num_bigint::BigUint;

#[path = "../src/zero_lift_search.rs"]
mod zero_lift_search;

use zero_lift_search::{search_orbit_first_zero_lift_runs, ZeroLiftSearchBounds};

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
