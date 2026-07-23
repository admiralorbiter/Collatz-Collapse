use collatz_cegar::{
    InducedVMapEngine, ZeroLiftContinuationEngine, ZeroLiftExplorer,
};
use num_bigint::BigUint;

#[test]
fn test_canonical_child_from_initial_block() {
    let child_0 = ZeroLiftContinuationEngine::canonical_child_from_initial_block(0).unwrap();
    assert_eq!(child_0.source_residue, BigUint::from(342u32)); // C_0 = 342
    assert_eq!(child_0.endpoint_z, BigUint::from(487u32)); // D_0 = 487
    assert_eq!(child_0.total_precision, 9);
    assert_eq!(child_0.odd_multiplier, BigUint::from(729u32));

    // D_0 = 487 mod 512 = 487 != C_j => exact successor is None!
    let succ = ZeroLiftContinuationEngine::find_exact_successor(&child_0.endpoint_z).unwrap();
    assert_eq!(succ, None);
}

#[test]
fn test_exact_regression_witness_vectors() {
    // r=1 regression vector: z_0 = 200,534 => exactly 2 steps of j=0 (residues mod 512: 342, 342, 12)
    let z0_r1 = BigUint::from(200534u32);
    let n0_r1 = (&z0_r1 - BigUint::from(342u32)) / BigUint::from(512u32);
    let gaps_r1 = ZeroLiftExplorer::evaluate_concrete_orbit(0, n0_r1, 5).unwrap();
    assert_eq!(gaps_r1, vec![0, 0]);

    // r=3 regression vector: z_0 = 23,750,971,222 => exactly 4 steps of j=0 (residues mod 512: 342, 342, 342, 342, 441)
    let z0_r3 = BigUint::from(23750971222u64);
    let n0_r3 = (&z0_r3 - BigUint::from(342u32)) / BigUint::from(512u32);
    let gaps_r3 = ZeroLiftExplorer::evaluate_concrete_orbit(0, n0_r3, 5).unwrap();
    assert_eq!(gaps_r3, vec![0, 0, 0, 0]);
}

#[test]
fn test_parameterized_pure_zero_gap_witness_generator() {
    for length in [1, 3, 5, 10] {
        let z_0 = ZeroLiftContinuationEngine::pure_zero_gap_witness(length).unwrap();
        let n_0 = (&z_0 - BigUint::from(342u32)) / BigUint::from(512u32);

        let gaps = ZeroLiftExplorer::evaluate_concrete_orbit(0, n_0, length + 5).unwrap();
        assert_eq!(gaps.len(), length + 1);
        for &j in &gaps {
            assert_eq!(j, 0);
        }
    }
}

#[test]
fn test_exact_successor_solver_large_gap_j13() {
    // Large gap test j = 13: check C_13 is correctly solved by find_exact_successor
    let branch_13 = InducedVMapEngine::get_branch_normal_form(13).unwrap();
    let succ_c13 = ZeroLiftContinuationEngine::find_exact_successor(&branch_13.c_j_normalized).unwrap();
    assert_eq!(succ_c13, Some(13));
}

#[test]
fn test_exact_successor_solver_equivalence() {
    // C_0 = 342 => exact successor is 0
    let succ_c0 = ZeroLiftContinuationEngine::find_exact_successor(&BigUint::from(342u32)).unwrap();
    assert_eq!(succ_c0, Some(0));

    // C_1 = 7392 => exact successor is 1
    let succ_c1 = ZeroLiftContinuationEngine::find_exact_successor(&BigUint::from(7392u32)).unwrap();
    assert_eq!(succ_c1, Some(1));

    // Non-domain point y = 100 => exact successor is None
    let succ_100 = ZeroLiftContinuationEngine::find_exact_successor(&BigUint::from(100u32)).unwrap();
    assert_eq!(succ_100, None);
}
