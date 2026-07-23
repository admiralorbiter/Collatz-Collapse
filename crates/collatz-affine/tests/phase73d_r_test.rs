use collatz_cegar::{
    AcceleratedInvariantEngine, AcceleratedLiftEngine, InducedVMapEngine,
    AcceleratedTransitionSystemEngine,
};
use num_bigint::{BigInt, BigUint};

#[test]
fn test_bounded_transition_system_81_edges() {
    let edges = AcceleratedTransitionSystemEngine::build_bounded_transition_system(8).unwrap();
    assert_eq!(edges.len(), 81); // 9^2 = 81 complete edges for j, j' \in {0..8}

    // Verify h_min == 0 for all 81 edges (Theorem: S_{j,j'} >= 0)
    for edge in edges {
        assert_eq!(edge.minimum_free_parameter, BigUint::from(0u32));
        assert!(edge.target_parameter_offset >= BigInt::from(0i32));
    }
}

#[test]
fn test_edge_normal_form_conformance() {
    let edge_0_0 = AcceleratedTransitionSystemEngine::compute_complete_edge(0, 0).unwrap();
    assert_eq!(edge_0_0.source_parameter_residue, BigUint::from(391u32));
    assert_eq!(edge_0_0.target_parameter_offset, BigInt::from(557i32));

    let edge_0_1 = AcceleratedTransitionSystemEngine::compute_complete_edge(0, 1).unwrap();
    assert_eq!(edge_0_1.source_parameter_residue, BigUint::from(1313u32));
    assert_eq!(edge_0_1.target_parameter_offset, BigInt::from(116i32));

    let edge_1_0 = AcceleratedTransitionSystemEngine::compute_complete_edge(1, 0).unwrap();
    assert_eq!(edge_1_0.source_parameter_residue, BigUint::from(327u32));
    assert_eq!(edge_1_0.target_parameter_offset, BigInt::from(12605i32));

    let edge_1_1 = AcceleratedTransitionSystemEngine::compute_complete_edge(1, 1).unwrap();
    assert_eq!(edge_1_1.source_parameter_residue, BigUint::from(2485u32));
    assert_eq!(edge_1_1.target_parameter_offset, BigInt::from(5972i32));

    let edge_2_3 = AcceleratedTransitionSystemEngine::compute_complete_edge(2, 3).unwrap();
    assert_eq!(edge_2_3.source_parameter_residue, BigUint::from(1201743u32));
    assert_eq!(edge_2_3.target_parameter_offset, BigInt::from(304534i32));
}

#[test]
fn test_3way_accelerated_guard_agreement() {
    // 1. Single-step empty prefix test: \Lambda_j(\epsilon) == C_j
    let init = AcceleratedLiftEngine::initial_state();
    for j in 0..=3 {
        let branch_j = InducedVMapEngine::get_branch_normal_form(j).unwrap();
        let ext = AcceleratedLiftEngine::extend(&init, j).unwrap();
        assert_eq!(ext.emitted_block_lift_digit, branch_j.c_j_normalized);
    }

    // 2. Two-step sequence test: \Lambda_{j'}(j) == R_{j,j'}
    for j in 0..=3 {
        let ext_1 = AcceleratedLiftEngine::extend(&init, j).unwrap();
        for j_next in 0..=3 {
            let edge = AcceleratedTransitionSystemEngine::compute_complete_edge(j, j_next).unwrap();
            let ext_2 = AcceleratedLiftEngine::extend(&ext_1.child_state, j_next).unwrap();
            assert_eq!(ext_2.emitted_block_lift_digit, edge.source_parameter_residue);
        }
    }
}

#[test]
fn test_unaccelerated_guard_alignment_k_modulus() {
    let init = AcceleratedLiftEngine::initial_state();
    let test_sequences = vec![
        vec![0], vec![1], vec![2],
        vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 3],
    ];

    for seq in test_sequences {
        let mut curr = init.clone();
        for &j in &seq {
            let ext = AcceleratedLiftEngine::extend(&curr, j).unwrap();
            curr = ext.child_state;
        }

        // Quotient guard k \equiv 61 + 512 * \rho_r \pmod{2^{B_r + 9}}
        let k_guard = BigUint::from(61u32) + (BigUint::from(512u32) * &curr.source_residue);
        let mod_exp = curr.total_precision + 9;
        let modulus = BigUint::from(1u32) << mod_exp;
        assert_eq!((&k_guard % &modulus), k_guard);
    }
}

#[test]
fn test_bounded_invariant_search_result() {
    let res = AcceleratedInvariantEngine::analyze_bounded_system(8).unwrap();
    assert_eq!(res.total_edges_verified, 81);
    assert_eq!(res.survivor_measure_depth_1, "1/480");
    assert!(res.verified_bounded_analysis);
}
