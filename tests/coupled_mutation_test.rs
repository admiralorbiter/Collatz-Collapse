use collatz_cegar::coupled_invariant_miner::CoupledInvariantMiner;
use collatz_cegar::extremal_source_search::ExtremalSourceSearchEngine;
use num_bigint::BigUint;

#[test]
fn test_mutation_1_drop_qu_causes_false_collision_merge() {
    let w0 = ExtremalSourceSearchEngine::base_guarded_word(0);
    let w03 = ExtremalSourceSearchEngine::extend_guarded_word(&w0, 3);
    let w031 = ExtremalSourceSearchEngine::extend_guarded_word(&w03, 1);

    let w3 = ExtremalSourceSearchEngine::base_guarded_word(3);
    let w31 = ExtremalSourceSearchEngine::extend_guarded_word(&w3, 1);

    // If Q_u is dropped from the state, (0,3,1) and (3,1) appear identical:
    assert_eq!(w031.endpoint, w31.endpoint);

    // But under exact canonical extension, their successors for h=0 diverge:
    let (d_031_0, _) = CoupledInvariantMiner::canonical_extension(&w031.endpoint, &w031.affine.multiplier, 0);
    let (d_31_0, _) = CoupledInvariantMiner::canonical_extension(&w31.endpoint, &w31.affine.multiplier, 0);
    assert_ne!(d_031_0, d_31_0, "Mutation 1 Caught: Dropping Q_u produces unsound trajectory merge!");
}

#[test]
fn test_mutation_2_fixed_precision_m_bits_causes_unsoundness() {
    let d1 = BigUint::from(487u64);
    let q1 = BigUint::from(729u64);
    let mod512 = BigUint::from(512u64);

    let d2 = &d1 + &mod512;
    let q2 = &q1 + &mod512;

    let (d1_next, _) = CoupledInvariantMiner::canonical_extension(&d1, &q1, 0);
    let (d2_next, _) = CoupledInvariantMiner::canonical_extension(&d2, &q2, 0);

    assert_ne!(&d1_next % &mod512, &d2_next % &mod512, "Mutation 2 Caught: Fixed precision m bits is unsound!");
}
