use collatz_affine::{extend_prefix_state, PrefixLiftState, UBlockAccelerator};
use collatz_cegar::{AcceleratedRankingEngine, InducedVMapEngine};
use num_bigint::BigUint;

#[test]
fn test_prefix_lift_state_and_extend() {
    let empty = PrefixLiftState::empty();
    assert_eq!(empty.total_valuation, 0);

    let u_trans = extend_prefix_state(&empty, true).unwrap();
    assert!(u_trans.symbol_is_u);
    assert_eq!(u_trans.child_state.guard_residue, BigUint::from(7u32));

    let v_trans = extend_prefix_state(&empty, false).unwrap();
    assert!(!v_trans.symbol_is_u);
    assert_eq!(v_trans.child_state.guard_residue, BigUint::from(61u32));
}

#[test]
fn test_u_block_acceleration_countdown() {
    let k = BigUint::from(7u32); // k = 7 => 11*7+3 = 80 = 2^4 * 5 => x = 9
    let val_x = UBlockAccelerator::compute_valuation_x(&k);
    assert_eq!(val_x, 9);

    let l = UBlockAccelerator::arbitrary_u_countdown_length(val_x);
    assert_eq!(l, 1); // l_u(9) = (9-5)/4 = 1

    let res = UBlockAccelerator::accelerate(&k).unwrap();
    assert_eq!(res.arbitrary_u_count_l, 1);
}

#[test]
fn test_induced_v_map_valid_transitions() {
    // 1. Immediate vv step (t=3763 => 5358)
    let t_vv = BigUint::from(3763u32);
    assert!(InducedVMapEngine::is_positive_realizable(&t_vv));
    let trans_vv = InducedVMapEngine::eval_step(&t_vv).unwrap();
    assert_eq!(trans_vv.valuation_delta, 1);
    assert_eq!(trans_vv.u_step_count_j, 0);
    assert!(trans_vv.is_valid_v_return);
    assert_eq!(trans_vv.next_t, Some(BigUint::from(5358u32)));
    assert!(trans_vv.next_t_realizable);

    // 2. Intervening u-step vuv (t=81313 => 195372)
    let t_vuv = BigUint::from(81313u32);
    assert!(InducedVMapEngine::is_positive_realizable(&t_vuv));
    let trans_vuv = InducedVMapEngine::eval_step(&t_vuv).unwrap();
    assert_eq!(trans_vuv.valuation_delta, 5);
    assert_eq!(trans_vuv.u_step_count_j, 1);
    assert!(trans_vuv.is_valid_v_return);
    assert_eq!(trans_vuv.next_t, Some(BigUint::from(195372u32)));
    assert!(trans_vuv.next_t_realizable);

    // Sample ranking analysis
    let initial_t_values = vec![t_vv, t_vuv];
    let proof = AcceleratedRankingEngine::analyze_sample_trajectories(&initial_t_values, 5).unwrap();
    assert!(proof.evaluated_transitions_count > 0);
}
