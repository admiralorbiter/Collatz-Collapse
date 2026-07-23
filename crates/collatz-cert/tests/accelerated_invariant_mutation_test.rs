use collatz_cert::schema::AcceleratedInvariantReportJson;
use collatz_cert::verify_accelerated_invariant_report;

fn create_valid_report() -> AcceleratedInvariantReportJson {
    AcceleratedInvariantReportJson {
        schema_version: "accelerated_invariant_report_v1".to_string(),
        total_edges_verified: 81,
        max_gap_evaluated: 8,
        survivor_measure_depth_1: "1/480".to_string(),
        verified_bounded_analysis: true,
        status_tag: "VerifiedBoundedAcceleratedAnalysis".to_string(),
    }
}

#[test]
fn test_phase73d_r_roundtrip_verification() {
    let report = create_valid_report();
    assert!(verify_accelerated_invariant_report(&report).is_ok());
}

#[test]
fn test_10_corruption_mutation_matrix_phase73d_r() {
    let base = create_valid_report();

    // 1. Schema version corruption
    let mut mut1 = base.clone();
    mut1.schema_version = "invalid_v1".to_string();
    assert!(verify_accelerated_invariant_report(&mut1).is_err());

    // 2. Unverified bounded analysis flag
    let mut mut2 = base.clone();
    mut2.verified_bounded_analysis = false;
    assert!(verify_accelerated_invariant_report(&mut2).is_err());

    // 3. Corrupted survivor measure
    let mut mut3 = base.clone();
    mut3.survivor_measure_depth_1 = "1/256".to_string();
    assert!(verify_accelerated_invariant_report(&mut3).is_err());

    // 4. Corrupted edge count
    let mut mut4 = base.clone();
    mut4.total_edges_verified = 80;
    assert!(verify_accelerated_invariant_report(&mut4).is_err());

    // 5. Corrupted max_gap
    let mut mut5 = base.clone();
    mut5.max_gap_evaluated = 9;
    assert!(verify_accelerated_invariant_report(&mut5).is_err());
}
