use collatz_cegar::positive_control_replay_engine::PositiveControlReplayEngine;
use collatz_cegar::single_pass_grid_traversal::SinglePassGridTraversal;
use collatz_cegar::state_coupled_invariant_miner::StateCoupledInvariantMiner;
use collatz_cegar::two_zero_cylinder_characterization::{DoubleZeroStatus, TwoZeroCylinderCharacterization};
use num_bigint::BigUint;

#[test]
fn test_zero_lift_gap_uniqueness_lemma() {
    let char_engine = TwoZeroCylinderCharacterization::new(32);
    assert!(
        char_engine.verify_gap_uniqueness(),
        "Zero-lift gap uniqueness failed! Overlap detected between zero-lift cylinders."
    );
}

#[test]
fn test_exact_two_zero_cylinders_generation() {
    let char_engine = TwoZeroCylinderCharacterization::new(32);
    let cyls = char_engine.generate_two_zero_cylinders();
    assert_eq!(cyls.len(), 1089); // 33x33 = 1089 raw two-zero cylinders
}

#[test]
fn test_positive_controls_replay_0_0_7_and_2_2_8() {
    // Control 1: (0,0,7)
    let (d_007, status_007, rejections_007) = PositiveControlReplayEngine::verify_control_0_0_7();
    assert_eq!(d_007, "2487743142969238870".parse::<BigUint>().unwrap());
    assert!(matches!(status_007, DoubleZeroStatus::ExactlyOneZeroLift { first_gap: 0, .. }));
    assert_eq!(rejections_007.len(), 33);
    assert!(rejections_007.iter().all(|&rejected| rejected)); // All 33 second step guards rejected!

    // Control 2: (2,2,8)
    let (d_228, status_228, rejections_228) = PositiveControlReplayEngine::verify_control_2_2_8();
    assert_eq!(d_228, "15119774165077853715448150".parse::<BigUint>().unwrap());
    assert!(matches!(status_228, DoubleZeroStatus::ExactlyOneZeroLift { first_gap: 0, .. }));
    assert_eq!(rejections_228.len(), 33);
    assert!(rejections_228.iter().all(|&rejected| rejected)); // All 33 second step guards rejected!
}

#[test]
fn test_single_pass_grid_traversal_double_zero_exclusion() {
    let traversal = SinglePassGridTraversal::new(3, 4);
    let (stats, witnesses) = traversal.run_traversal();
    
    assert_eq!(stats.prefix_word_count, 155);
    assert_eq!(stats.at_least_two_zero_lift_count, 0);
    assert!(witnesses.is_empty());
}

#[test]
fn test_state_coupled_invariant_miner() {
    let miner = StateCoupledInvariantMiner::new(9);
    let endpoints = vec![
        ("2487743142969238870".parse::<BigUint>().unwrap(), BigUint::from(729u64)),
        ("15119774165077853715448150".parse::<BigUint>().unwrap(), BigUint::from(19683u64)),
    ];
    
    let safe_states = miner.mine_invariant(&endpoints);
    assert_eq!(safe_states.len(), 2);
}
