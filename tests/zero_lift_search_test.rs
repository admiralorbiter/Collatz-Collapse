use num_bigint::BigUint;

#[path = "../src/zero_lift_search.rs"]
mod zero_lift_search;

use zero_lift_search::{compute_diophantine_defect_diagnostic, compute_endpoint_compression_diagnostic, compute_guarded_top_ternary_window_diagnostic, compute_pathwise_defect_forcing_diagnostic, compute_prefix_signature, compute_scc_cycle_cone_diagnostic, compute_telescoping_defect_diagnostic, compute_top_ternary_window_diagnostic, compute_universal_certificate_diagnostic, export_concrete_defect_census, export_quotient_artifacts, export_reachable_scc_census_summary, export_recurrent_zero_lift_scc_census, search_counterexample_q1_traces, search_height_controlled_recurrent_defects, search_orbit_first_zero_lift_runs, CycleConeClassification, DiophantineDefectBounds, ZeroLiftSearchBounds};

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

#[test]
fn test_universal_certificate_diagnostic_execution() {
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
    let diag = compute_universal_certificate_diagnostic(&traces[0]);
    assert!(diag.is_source_congruence_satisfied);
    assert!(diag.is_endpoint_residue_satisfied);
}

#[test]
fn test_export_quotient_artifacts_execution() {
    let (candidate, cert) = export_quotient_artifacts();
    assert!(candidate.lean_soundness_verified);
    assert_eq!(candidate.states.len(), 2);
    assert_eq!(cert.schema_version, "v1.0");
}

#[test]
fn test_diophantine_defect_diagnostic_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    let defect_bounds = DiophantineDefectBounds {
        convergent_p: 19,
        convergent_q: 12,
        max_step_time: 12,
    };
    let diag = compute_diophantine_defect_diagnostic(&traces[0], &defect_bounds);
    assert!(diag.step_time_t > 0);
    assert!(diag.exponent_sum_a > 0);
}

#[test]
fn test_search_height_controlled_recurrent_defects_execution() {
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
    let defect_bounds = DiophantineDefectBounds {
        convergent_p: 19,
        convergent_q: 12,
        max_step_time: 12,
    };
    let trace = search_height_controlled_recurrent_defects(&traces[0], &defect_bounds);
    assert!(trace.step_time_t > 0);
    assert_eq!(trace.defect_bound_k, "10");
}

#[test]
fn test_export_concrete_defect_census_execution() {
    let census = export_concrete_defect_census();
    assert_eq!(census.len(), 11);
    let excluded: Vec<_> = census.iter().filter(|e| e.allowed_time_mod_12 == 6).collect();
    assert!(excluded.is_empty());
}

#[test]
fn test_compute_telescoping_defect_diagnostic_execution() {
    let diag = compute_telescoping_defect_diagnostic(7, 11, 2, 3, 5);
    assert!(diag.shifted_height_bound_verified);
}

#[test]
fn test_export_recurrent_zero_lift_scc_census_execution() {
    let census = export_recurrent_zero_lift_scc_census(2, 3, 12, 18);
    assert_eq!(census.len(), 1);
    assert!(census[0].is_zero_lift_accepted);
}

#[test]
fn test_compute_scc_cycle_cone_diagnostic_execution() {
    let diag = compute_scc_cycle_cone_diagnostic(12, 19);
    assert_eq!(diag.classification, CycleConeClassification::IntersectsNeutralBand);
    let diag2 = compute_scc_cycle_cone_diagnostic(12, 18);
    assert_eq!(diag2.classification, CycleConeClassification::StrictlyBelowNeutralBand);
}

#[test]
fn test_export_reachable_scc_census_summary_execution() {
    let summary = export_reachable_scc_census_summary(2, 3, 12, 18);
    assert_eq!(summary.reachable_zero_lift_sccs, 1);
}

#[test]
fn test_compute_pathwise_defect_forcing_diagnostic_execution() {
    let diag = compute_pathwise_defect_forcing_diagnostic(0, 100, 12, 18);
    assert!(diag.is_pathwise_defect_linear_verified);
}

#[test]
fn test_compute_endpoint_compression_diagnostic_execution() {
    let diag = compute_endpoint_compression_diagnostic(12, 5, 2);
    assert!(diag.is_endpoint_compressed_verified);
}

#[test]
fn test_compute_top_ternary_window_diagnostic_execution() {
    let diag = compute_top_ternary_window_diagnostic(2, 12, 5);
    assert!(diag.is_top_window_zero_verified);
}

#[test]
fn test_compute_guarded_top_ternary_window_diagnostic_execution() {
    let diag = compute_guarded_top_ternary_window_diagnostic(2, 12, 5);
    assert!(diag.is_guarded_and_zero_verified);
}











