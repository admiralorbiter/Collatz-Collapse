use crate::schema::{AcceleratedInvariantReportJson, Phase73dVerificationReportJson};
use num_bigint::BigUint;
use std::str::FromStr;

/// Independent verifier for Phase 7.3D Verification Report.
pub fn verify_phase73d_report(report: &Phase73dVerificationReportJson) -> Result<(), String> {
    if report.schema_version != "phase73d_verification_report_v1" {
        return Err(format!(
            "Unsupported schema version: expected phase73d_verification_report_v1, found {}",
            report.schema_version
        ));
    }

    if !report.verified_u_countdown_ranking {
        return Err("u-countdown ranking check failed".to_string());
    }

    // 1. Verify immediate v-to-v transition (j=0) for t = 3763 => 5358 (z = 342 => 487)
    let t_vv = BigUint::from_str("3763").unwrap();
    if (&t_vv % BigUint::from(11u32)) != BigUint::from(1u32) {
        return Err("t = 3763 is not positive-realizable (t mod 11 != 1)".to_string());
    }
    let val_expr_vv = BigUint::from(231u32) + (BigUint::from(729u32) * &t_vv);
    let delta_vv = val_expr_vv.trailing_zeros().unwrap_or(0);
    if delta_vv != 1 {
        return Err(format!("Expected delta = 1 for t = 3763, found {}", delta_vv));
    }
    let u_next_vv = &val_expr_vv / BigUint::from(2u32);
    if (&u_next_vv % BigUint::from(256u32)) != BigUint::from(81u32) {
        return Err("vv step u_next mod 256 != 81".to_string());
    }
    let next_t_vv = (&u_next_vv - BigUint::from(81u32)) / BigUint::from(256u32);
    if next_t_vv != BigUint::from_str("5358").unwrap() {
        return Err(format!("Expected next_t = 5358 for t = 3763, found {}", next_t_vv));
    }

    // 2. Verify vuv transition (j=1) for t = 81313 => 195372 (z = 7392 => 17761)
    let t_vuv = BigUint::from_str("81313").unwrap();
    if (&t_vuv % BigUint::from(11u32)) != BigUint::from(1u32) {
        return Err("t = 81313 is not positive-realizable (t mod 11 != 1)".to_string());
    }
    let val_expr_vuv = BigUint::from(231u32) + (BigUint::from(729u32) * &t_vuv);
    let delta_vuv = val_expr_vuv.trailing_zeros().unwrap_or(0);
    if delta_vuv != 5 {
        return Err(format!("Expected delta = 5 for t = 81313, found {}", delta_vuv));
    }
    let _j_vuv = (delta_vuv - 1) / 4; // j = 1
    let base_unit_vuv = &val_expr_vuv / BigUint::from(32u32);
    let u_next_vuv = base_unit_vuv * BigUint::from(27u32);
    if (&u_next_vuv % BigUint::from(256u32)) != BigUint::from(81u32) {
        return Err("vuv step u_next mod 256 != 81".to_string());
    }
    let next_t_vuv = (&u_next_vuv - BigUint::from(81u32)) / BigUint::from(256u32);
    if next_t_vuv != BigUint::from_str("195372").unwrap() {
        return Err(format!("Expected next_t = 195372 for t = 81313, found {}", next_t_vuv));
    }

    // 3. Verify exact normalized z-coordinates for j=2 (C_2 = 86208, D_2 = 349537)
    let c_2_norm = BigUint::from(86208u32);
    let d_2_norm = BigUint::from(349537u32);
    let m_2 = BigUint::from(131072u32);
    let q_2 = BigUint::from(531441u32);
    let n_sample = BigUint::from(0u32);

    let z_sample = &c_2_norm + (&m_2 * &n_sample);
    let t_sample = BigUint::from(1u32) + (BigUint::from(11u32) * &z_sample);
    let t_prime_expected = BigUint::from(1u32) + (BigUint::from(11u32) * (&d_2_norm + (&q_2 * &n_sample)));

    let val_expr_j2 = BigUint::from(231u32) + (BigUint::from(729u32) * &t_sample);
    let delta_j2 = val_expr_j2.trailing_zeros().unwrap_or(0);
    if delta_j2 != 9 {
        return Err(format!("Expected delta = 9 for j=2, found {}", delta_j2));
    }
    let base_unit_j2 = &val_expr_j2 / BigUint::from(512u32);
    let u_next_j2 = base_unit_j2 * BigUint::from(729u32);
    let next_t_j2 = (&u_next_j2 - BigUint::from(81u32)) / BigUint::from(256u32);
    if next_t_j2 != t_prime_expected {
        return Err("j=2 z-coordinate normalization test failed".to_string());
    }

    Ok(())
}

/// Independent verifier for Phase 7.3D-R Accelerated Invariant Report.
pub fn verify_accelerated_invariant_report(report: &AcceleratedInvariantReportJson) -> Result<(), String> {
    if report.schema_version != "accelerated_invariant_report_v1" {
        return Err(format!(
            "Unsupported schema version: expected accelerated_invariant_report_v1, found {}",
            report.schema_version
        ));
    }

    if !report.verified_bounded_analysis {
        return Err("verified_bounded_analysis check failed".to_string());
    }

    if report.survivor_measure_depth_1 != "1/480" {
        return Err(format!("Expected survivor_measure_depth_1 = 1/480, found {}", report.survivor_measure_depth_1));
    }

    let expected_edges = ((report.max_gap_evaluated + 1) * (report.max_gap_evaluated + 1)) as usize;
    if report.total_edges_verified != expected_edges {
        return Err(format!(
            "Edge count mismatch for max_j={}: expected {}, found {}",
            report.max_gap_evaluated, expected_edges, report.total_edges_verified
        ));
    }

    Ok(())
}
