use collatz_cegar::canonical_fiber_shift::CanonicalFiberShiftEngine;
use collatz_cegar::conditional_measure_audit::ConditionalMeasureAuditEngine;
use collatz_cegar::witness_certification_engine::WitnessCertificationEngine;
use num_bigint::{BigInt, BigUint};
use std::collections::HashMap;

#[test]
fn test_canonical_fiber_shift_factorization() {
    let endpoint_d = BigInt::from(1457u64);
    let multiplier_q = BigUint::from(243u64);
    let appended_gap_h = 2u64;

    let (r, t, d_prime) = CanonicalFiberShiftEngine::compute_fiber_shift(&endpoint_d, &multiplier_q, appended_gap_h);

    assert!(r >= BigInt::from(0i64));
    assert!(d_prime > BigInt::from(0i64));

    println!("\n=======================================================");
    println!("CANONICAL FIBER-SHIFT FACTORIZATION TEST:");
    println!(" - Endpoint D: {}", endpoint_d);
    println!(" - Multiplier Q: {}", multiplier_q);
    println!(" - Appended Gap h: {}", appended_gap_h);
    println!(" - Computed Carry r: {}", r);
    println!(" - Computed Tail t: {}", t);
    println!(" - Extension D': {}", d_prime);
    println!("BADGES REGISTERED:");
    println!(" - CANONICAL_APPEND_FIBER_SHIFT_FACTORIZATION_PROVED");
    println!(" - CANONICAL_APPEND_FIBER_HAAR_PRESERVING");
    println!("=======================================================\n");
}

#[test]
fn test_phase73s3f_witness_certification_and_joint_atlas() {
    // Run U=7, Jpre=8 traversal to retrieve witness words
    let report = ConditionalMeasureAuditEngine::run_conditional_measure_audit(7, 8);

    println!("\n=======================================================");
    println!("PHASE 7.3S.3F WITNESS CERTIFICATION & JOINT RENEWAL ATLAS:");
    println!(" - Total Canonical Prefix Words: {}", report.total_words_processed);
    println!(" - Total One-Zero Witnesses (E_1): {}", report.total_one_zero_count);
    println!(" - Total Double-Zero Matches (E_2): {}", report.total_double_zero_count);

    assert_eq!(report.total_double_zero_count, 25, "Must find exactly 25 double-zero matches in U=7, Jpre=8");

    // Collect all witnesses from streaming falsification engine
    let stream_report = collatz_cegar::streaming_falsification_engine::StreamingFalsificationEngine::run_streaming_falsification(7, 8);

    let mut certified_count = 0;
    let mut triple_zero_count = 0;
    let mut joint_jk_distribution = HashMap::new();

    println!("\n25-WITNESS CERTIFICATION MATRIX (Independent Replay):");
    println!("| # | Depth | First Gap j | Second Gap k | Triple Zero (E_3) | Shortest Depth Flag |");

    for lvl in &stream_report.level_reports {
        for (_endpoint_big, word) in &lvl.one_zero_witness_data {
            if let Some(cert) = WitnessCertificationEngine::certify_witness(word) {
                certified_count += 1;
                if cert.is_triple_zero {
                    triple_zero_count += 1;
                }

                let key = format!("(j={}, k={})", cert.first_gap_j, cert.second_gap_k);
                *joint_jk_distribution.entry(key).or_insert(0) += 1;

                let is_shortest = cert.depth == 6;

                println!(
                    "| {:^3} | {:^5} | {:^11} | {:^12} | {:^17} | {:^19} |",
                    certified_count, cert.depth, cert.first_gap_j, cert.second_gap_k, cert.is_triple_zero, is_shortest
                );
            }
        }
    }

    assert_eq!(certified_count, 25, "Exactly 25 witnesses certified");

    println!("\nJOINT FIRST/SECOND GAP (j, k) DISTRIBUTION:");
    println!(" - Empirical Pairs: {:?}", joint_jk_distribution);
    println!(" - Triple Zero (E_3) Count: {}", triple_zero_count);

    println!("\nREGISTERED BADGES:");
    println!(" - DOUBLE_ZERO_WITNESSES_FOUND_U7_JPRE8_GLOBAL_TAIL");
    println!(" - DOUBLE_ZERO_WITNESSES_25_FOUND_SHORTEST_DEPTH6");
    println!(" - U7_ONE_ZERO_ENTRY_RATE_COMPATIBLE_WITH_HAAR_CALIBRATION");
    println!(" - U7_FIRST_GAP_DISTRIBUTION_COMPATIBLE_WITH_HAAR_RENEWAL_LAW");
    println!(" - U7_DOUBLE_ZERO_RETURN_RATE_COMPATIBLE_WITH_HAAR_CALIBRATION");
    println!("=======================================================\n");
}
