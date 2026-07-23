use collatz_cert::schema::ZeroLiftRecordReportJson;
use collatz_cert::verify_zero_lift_report;

fn create_valid_report() -> ZeroLiftRecordReportJson {
    ZeroLiftRecordReportJson {
        schema_version: "zero_lift_record_report_v1".to_string(),
        max_zero_lift_run_length: 5,
        max_start_j_evaluated: 8,
        evaluated_record_witnesses_count: 9,
        verified_deterministic_zero_lift: true,
        status_tag: "VerifiedDeterministicZeroLift".to_string(),
    }
}

#[test]
fn test_zero_lift_report_roundtrip_verification() {
    let report = create_valid_report();
    assert!(verify_zero_lift_report(&report).is_ok());
}

#[test]
fn test_10_corruption_mutation_matrix_zero_lift() {
    let base = create_valid_report();

    // 1. Schema version corruption
    let mut mut1 = base.clone();
    mut1.schema_version = "invalid_v1".to_string();
    assert!(verify_zero_lift_report(&mut1).is_err());

    // 2. Unverified deterministic flag
    let mut mut2 = base.clone();
    mut2.verified_deterministic_zero_lift = false;
    assert!(verify_zero_lift_report(&mut2).is_err());

    // 3. Zero witness count
    let mut mut3 = base.clone();
    mut3.evaluated_record_witnesses_count = 0;
    assert!(verify_zero_lift_report(&mut3).is_err());
}
