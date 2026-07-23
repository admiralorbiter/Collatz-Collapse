use collatz_affine::{
    recover_broad_cylinder, recover_exact_cylinder, recover_sequence_cylinder, AffineInteraction,
    ExactWordCylinder, MacrostepData, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;
use num_traits::Zero;
use proptest::prelude::*;

fn gen_valuation_word() -> impl Strategy<Value = ValuationWord> {
    prop::collection::vec(1u32..=4u32, 1..=3).prop_map(|v| ValuationWord::from_u32_slice(&v).unwrap())
}

#[test]
fn test_benchmark_uv_interaction() {
    let u = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u.clone()).unwrap();
    let m_v = MacrostepData::from_word(v.clone()).unwrap();

    assert_eq!(m_u.odd_steps(), 3);
    assert_eq!(m_u.total_valuation(), 4);
    assert_eq!(m_u.d(), &BigInt::from(-11i32));

    assert_eq!(m_v.odd_steps(), 6);
    assert_eq!(m_v.total_valuation(), 9);
    assert_eq!(m_v.d(), &BigInt::from(-217i32));

    let interaction = AffineInteraction::new(&m_u, &m_v);
    assert_eq!(interaction.delta(), &BigInt::from(-5568i32));
    assert_eq!(interaction.delta_v2(), TwoAdicValuation::Finite(6));
    assert!(!interaction.is_common_center());

    // Identities
    assert!(interaction.same_form_identity_holds());
    assert!(interaction.cross_form_identity_holds());
    assert!(interaction.commutator_identity_holds());

    // Cross-form cylinder recovery yields exact v cylinder: 935 mod 1024
    let exact_v_recovered = recover_exact_cylinder(&interaction).unwrap();
    let direct_exact_v = ExactWordCylinder::from_valuation_word(v.clone()).unwrap();
    assert_eq!(exact_v_recovered.residue, direct_exact_v.source.residue);
    assert_eq!(exact_v_recovered.modulus_exponent, direct_exact_v.source.modulus_exponent);
    assert_eq!(exact_v_recovered.residue, num_bigint::BigUint::from(935u32));
    assert_eq!(exact_v_recovered.modulus_exponent, 10);

    // Sequence cylinder yields concatenated exact word: 1767 mod 16384
    let seq_recovered = recover_sequence_cylinder(&u, &v).unwrap();
    assert_eq!(seq_recovered.residue, num_bigint::BigUint::from(1767u32));
    assert_eq!(seq_recovered.modulus_exponent, 14);
}

#[test]
fn test_shallow_regression_pair() {
    let w1 = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let w2 = ValuationWord::from_u32_slice(&[1, 2, 2]).unwrap();

    let m_w1 = MacrostepData::from_word(w1).unwrap();
    let m_w2 = MacrostepData::from_word(w2).unwrap();

    assert_eq!(m_w1.d(), &BigInt::from(-11i32));
    assert_eq!(m_w2.d(), &BigInt::from(5i32)); // 32 - 27 = 5 > 0

    let interaction = AffineInteraction::new(&m_w1, &m_w2);
    assert_eq!(interaction.delta(), &BigInt::from(-348i32));
    assert_eq!(interaction.delta_v2(), TwoAdicValuation::Finite(2));

    assert!(interaction.same_form_identity_holds());
    assert!(interaction.cross_form_identity_holds());
    assert!(interaction.commutator_identity_holds());
}

#[test]
fn test_reference_invariance_property() {
    let u = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let w2 = ValuationWord::from_u32_slice(&[1, 2, 2]).unwrap();
    let v = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u).unwrap();
    let m_w2 = MacrostepData::from_word(w2).unwrap();
    let m_v = MacrostepData::from_word(v.clone()).unwrap();

    let interaction1 = AffineInteraction::new(&m_u, &m_v);
    let interaction2 = AffineInteraction::new(&m_w2, &m_v);

    let rec1 = recover_exact_cylinder(&interaction1).unwrap();
    let rec2 = recover_exact_cylinder(&interaction2).unwrap();
    let direct = ExactWordCylinder::from_valuation_word(v).unwrap();

    assert_eq!(rec1, rec2);
    assert_eq!(rec1.residue, direct.source.residue);
    assert_eq!(rec1.modulus_exponent, direct.source.modulus_exponent);
}

#[test]
fn test_two_adic_valuation_signed() {
    assert_eq!(
        TwoAdicValuation::from_bigint(&BigInt::from(-5568i32)),
        TwoAdicValuation::Finite(6)
    );
    assert_eq!(
        TwoAdicValuation::from_bigint(&BigInt::from(-348i32)),
        TwoAdicValuation::Finite(2)
    );
    assert_eq!(
        TwoAdicValuation::from_bigint(&BigInt::from(0i32)),
        TwoAdicValuation::Infinity
    );
    assert!(TwoAdicValuation::Infinity.at_least(100));
}

#[test]
fn test_empty_valuation_word_rejected() {
    let empty_word = ValuationWord::from_u32_slice(&[]).unwrap();
    let res = MacrostepData::from_word(empty_word);
    assert_eq!(res, Err(collatz_affine::AffineError::EmptyValuationWord));
}

#[test]
fn test_nonempty_word_implies_d_nonzero() {
    let words = vec![
        ValuationWord::from_u32_slice(&[1]).unwrap(),
        ValuationWord::from_u32_slice(&[2]).unwrap(),
        ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap(),
        ValuationWord::from_u32_slice(&[1, 2, 2]).unwrap(),
        ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap(),
    ];
    for w in words {
        let m = MacrostepData::from_word(w).unwrap();
        assert!(!m.d().is_zero(), "d_p must be non-zero for nonempty valuation word");
    }
}

#[test]
fn test_shared_fixed_point_common_center() {
    let u = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let uu = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 1, 2]).unwrap();

    let m_u = MacrostepData::from_word(u).unwrap();
    let m_uu = MacrostepData::from_word(uu).unwrap();

    let interaction = AffineInteraction::new(&m_u, &m_uu);
    assert_eq!(interaction.delta(), &BigInt::from(0i32));
    assert_eq!(interaction.delta_v2(), TwoAdicValuation::Infinity);
    assert!(interaction.is_common_center());
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 5000,
        max_shrink_iters: 1000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_generic_identities_hold(
        w1 in gen_valuation_word(),
        w2 in gen_valuation_word(),
        w3 in gen_valuation_word(),
        n_val in -10000i64..10000i64
    ) {
        let m1 = MacrostepData::from_word(w1).unwrap();
        let m2 = MacrostepData::from_word(w2).unwrap();
        let m3 = MacrostepData::from_word(w3).unwrap();

        let inter12 = AffineInteraction::new(&m1, &m2);
        let inter21 = AffineInteraction::new(&m2, &m1);
        let inter32 = AffineInteraction::new(&m3, &m2);

        // Identities hold symbolically
        prop_assert!(inter12.same_form_identity_holds());
        prop_assert!(inter12.cross_form_identity_holds());
        prop_assert!(inter12.commutator_identity_holds());

        // Antisymmetry
        prop_assert_eq!(inter12.delta(), &-inter21.delta());

        // Self interaction
        let self_inter = AffineInteraction::new(&m1, &m1);
        prop_assert!(self_inter.is_common_center());
        prop_assert_eq!(self_inter.delta_v2(), TwoAdicValuation::Infinity);

        // Concrete diagnostic evaluations match
        let n = BigInt::from(n_val);
        let (same_l, same_r) = inter12.evaluate_same_form_sides(&n);
        prop_assert_eq!(same_l, same_r);

        let (cross_l, cross_r) = inter12.evaluate_cross_form_sides(&n);
        prop_assert_eq!(cross_l, cross_r);

        // Recovery equivalence for q
        let rec_broad = recover_broad_cylinder(&inter12).unwrap();
        let rec_exact = recover_exact_cylinder(&inter12).unwrap();
        let rec_exact3 = recover_exact_cylinder(&inter32).unwrap();
        let direct_exact = ExactWordCylinder::from_valuation_word(m2.word().clone()).unwrap();

        prop_assert_eq!(rec_exact.residue.clone(), direct_exact.source.residue);
        prop_assert_eq!(rec_exact.modulus_exponent, direct_exact.source.modulus_exponent);
        prop_assert_eq!(rec_exact.clone(), rec_exact3);
        prop_assert_eq!(rec_broad.modulus_exponent + 1, rec_exact.modulus_exponent);
    }
}
