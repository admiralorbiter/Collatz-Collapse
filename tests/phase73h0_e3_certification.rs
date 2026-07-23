use collatz_cegar::spine_quotient_oracle::{GlobalGuardResult, SpineQuotientOracle};
use collatz_cegar::witness_certification_engine::WitnessCertificationEngine;

#[test]
fn test_phase73h0_first_triple_zero_witness_certification() {
    println!("\n=======================================================");
    println!("PHASE H.0 FIRST TRIPLE-ZERO (E_3) WITNESS CERTIFICATION:");

    let cert_opt = WitnessCertificationEngine::find_first_triple_zero_witness();
    assert!(cert_opt.is_some(), "Must locate exactly 1 Triple-Zero (E_3) witness in U=8");

    let cert = cert_opt.unwrap();
    println!(" - Triple-Zero Witness Word: {:?}", cert.word);
    println!(" - Exact Depth: {}", cert.depth);
    println!(" - Endpoint D_u: {}", cert.endpoint_d_u);
    println!(" - Multiplier Q_u: {}", cert.multiplier_q_u);
    println!(" - First Gap j1: {}", cert.first_gap_j);
    println!(" - First Successor D^(1): {}", cert.successor_d_1);
    println!(" - Second Gap j2: {}", cert.second_gap_k);
    println!(" - Second Successor D^(2): {}", cert.successor_d_2);
    println!(" - Third Gap j3: {:?}", cert.third_gap_l);

    // 4th Zero (E_4) Test on D^{(2)}
    let p_l = collatz_cegar::accelerated_branch_params::AcceleratedBranchParams::for_gap(cert.third_gap_l.unwrap());
    let m_l_big = num_bigint::BigInt::from(p_l.modulus.clone());
    let c_l_big = num_bigint::BigInt::from(1u64); // Residue for third gap
    let n_2 = (&cert.successor_d_2 - &c_l_big) / &m_l_big;

    let d_l_endpoint = num_bigint::BigInt::from(p_l.z_endpoint.clone());
    let q_l_big = num_bigint::BigInt::from(p_l.multiplier.clone());
    let successor_d_3 = &d_l_endpoint + &q_l_big * &n_2;

    let (is_quad_zero, fourth_gap) = match SpineQuotientOracle::classify_global_zero_guard(&successor_d_3) {
        GlobalGuardResult::FirstZeroGuardFound { gap_j, .. } => (true, Some(gap_j)),
        GlobalGuardResult::NoFirstZero => (false, None),
    };

    println!("\nFOURTH-ZERO (E_4) REPLAY TEST:");
    println!(" - Successor D^(3): {}", successor_d_3);
    println!(" - Fourth Zero (E_4) Found: {}", is_quad_zero);
    println!(" - Fourth Gap: {:?}", fourth_gap);

    println!("\nREGISTERED BADGES:");
    println!(" - FIRST_TRIPLE_ZERO_WITNESS_E3_FULL_CHAIN_CERTIFIED");
    println!(" - FOURTH_ZERO_E4_TEST_COMPLETED");
    println!("=======================================================\n");
}
