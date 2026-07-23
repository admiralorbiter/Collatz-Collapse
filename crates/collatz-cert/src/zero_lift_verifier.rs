use crate::schema::ZeroLiftRecordReportJson;

/// Independent verifier for Phase 7.3D-R2 Zero-Lift Record Report.
pub fn verify_zero_lift_report(report: &ZeroLiftRecordReportJson) -> Result<(), String> {
    if report.schema_version != "zero_lift_record_report_v1" {
        return Err(format!(
            "Unsupported schema version: expected zero_lift_record_report_v1, found {}",
            report.schema_version
        ));
    }

    if !report.verified_deterministic_zero_lift {
        return Err("verified_deterministic_zero_lift check failed".to_string());
    }

    if report.evaluated_record_witnesses_count == 0 {
        return Err("Record witnesses count cannot be zero".to_string());
    }

    Ok(())
}
