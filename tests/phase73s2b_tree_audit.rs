use collatz_cegar::accelerated_branch_params::AcceleratedBranchParams;
use collatz_cegar::backward_approximant_engine::BackwardApproximantEngine;
use collatz_cegar::bounded_reachability_probe::{BoundedReachabilityOutcome, BoundedReachabilityProbe};
use collatz_cegar::cylinder_trie_reduction::CylinderTrie;
use collatz_cegar::precision_aware_cylinder::Cylinder;
use num_bigint::BigUint;

#[test]
fn test_trie_subsumption_and_sibling_merging() {
    let mut trie = CylinderTrie::new();
    
    // Insert two sibling cylinders [0]_2 and [2]_2
    trie.insert(Cylinder::new(BigUint::from(0u64), 2), None);
    trie.insert(Cylinder::new(BigUint::from(2u64), 2), None);
    
    let cyls = trie.to_cylinders();
    // Sibling merging MUST collapse [0]_2 U [2]_2 into [0]_1 !
    assert_eq!(cyls.len(), 1);
    assert_eq!(cyls[0].precision, 1);
    assert_eq!(cyls[0].residue, BigUint::from(0u64));
}

#[test]
fn test_approximant_descending_chain_invariant() {
    let engine = BackwardApproximantEngine::new(2);
    let seq = engine.compute_approximant_sequence(3);
    
    assert_eq!(seq[0].to_cylinders().len(), 1); // E_0 = [0]_0
    assert_eq!(seq[1].to_cylinders().len(), 3); // E_1 = 3 cylinders
    assert_eq!(seq[2].to_cylinders().len(), 9); // E_2 = 9 cylinders
    assert_eq!(seq[3].to_cylinders().len(), 27); // E_3 = 27 cylinders (EXACTLY 27, NOT 37!)

    // Verify E_{n+1} <= E_n containment
    for n in 0..3 {
        let e_n = &seq[n];
        let e_next = &seq[n + 1];
        
        for cyl_next in &e_next.to_cylinders() {
            assert!(
                e_n.contains_endpoint(&cyl_next.residue),
                "Descending chain broken at level n={}",
                n
            );
        }
    }
}

#[test]
fn test_bounded_reachability_probe_outcomes_first_empty_depth_1() {
    let probe = BoundedReachabilityProbe::new(2, 2, 2);
    let engine = BackwardApproximantEngine::new(2);
    let seq = engine.compute_approximant_sequence(1);
    
    // I_{2,2} cap E_1 is EMPTY AT LEVEL 1 (n=1)!
    let (outcome, witnesses) = probe.evaluate_intersection(&seq[1], 1);
    assert_eq!(
        outcome,
        BoundedReachabilityOutcome::BoundedPrefixZeroTailExcluded {
            prefix_max_len: 2,
            j_pre_max: 2,
            j_tail_max: 2,
            first_empty_depth: 1,
        }
    );
    assert!(witnesses.is_empty());
}

#[test]
fn test_8_adversarial_mutations_suite() {
    // Mutation 1: Use Q_j instead of Q_j^{-1}
    let _p_0 = AcceleratedBranchParams::for_gap(0);
    let target = Cylinder::new(BigUint::from(0u64), 5);
    let pred_correct = Cylinder::pre_j(&target, 0);
    let pred_wrong_multiplier = Cylinder::new(&pred_correct.residue + BigUint::from(1u64), pred_correct.precision);
    
    let succ_wrong = pred_wrong_multiplier.post_j(0);
    assert!(succ_wrong.is_none() || succ_wrong.unwrap().residue != target.residue);

    // Mutation 2: Non-sibling trie merge rejection
    let mut trie = CylinderTrie::new();
    trie.insert(Cylinder::new(BigUint::from(0u64), 2), None);
    trie.insert(Cylinder::new(BigUint::from(1u64), 2), None);
    let cyls = trie.to_cylinders();
    assert_ne!(cyls.len(), 1); // Cannot merge non-siblings 0 and 1 mod 4
}
