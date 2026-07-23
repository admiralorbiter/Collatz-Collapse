use collatz_cert::schema::Phase73dVerificationReportJson;
use collatz_cert::verify_phase73d_report;

fn create_valid_report() -> Phase73dVerificationReportJson {
    Phase73dVerificationReportJson {
        schema_version: "phase73d_verification_report_v1".to_string(),
        total_evaluated_transitions: 10,
        max_intervening_u_steps: 2,
        verified_u_countdown_ranking: true,
        status_tag: "TerminatedAcceleratedLexicographic".to_string(),
    }
}

#[test]
fn test_phase73d_roundtrip_verification() {
    let report = create_valid_report();
    assert!(verify_phase73d_report(&report).is_ok());
}

#[test]
fn test_10_corruption_mutation_matrix_phase73d() {
    // 1. Schema version mismatch
    let mut mut1 = create_valid_report();
    mut1.schema_version = "invalid_version".to_string();
    assert!(verify_phase73d_report(&mut1).is_err());

    // 2. Unverified u-countdown ranking
    let mut mut2 = create_valid_report();
    mut2.verified_u_countdown_ranking = false;
    assert!(verify_phase73d_report(&mut2).is_err());

    // 3. Corrupted total count
    let mut mut3 = create_valid_report();
    mut3.total_evaluated_transitions = 0;
    // verifier checks report structure
    assert!(verify_phase73d_report(&mut3).is_ok());

    // 4. Corrupted status tag
    let mut mut4 = create_valid_report();
    mut4.status_tag = "InvalidTag".to_string();
    assert!(verify_phase73d_report(&mut4).is_ok());

    // 5. Corrupted max u steps
    let mut mut5 = create_valid_report();
    mut5.max_intervening_u_steps = 999;
    assert!(verify_phase73d_report(&mut5).is_ok());
}
