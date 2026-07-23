use collatz_affine::{
    AffinePrefix, CanonicalCylinder, ExecutionSequence, ThenSequence, ValuationSemantics,
    ValuationWord,
};
use num_bigint::BigUint;
use num_traits::One;
use proptest::prelude::*;

fn gen_valuation_word() -> impl Strategy<Value = ValuationWord> {
    prop::collection::vec(1u32..=4u32, 1..=3)
        .prop_map(|v| ValuationWord::from_u32_slice(&v).unwrap())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5000))]

    #[test]
    fn prop_composition_left_to_right_pairs(
        w1 in gen_valuation_word(),
        w2 in gen_valuation_word(),
        offset in 0u64..50u64
    ) {
        let seq = w1.clone().then(w2.clone());
        let combined = seq.combined_affine_prefix().unwrap();

        let (res, mod_exp) = combined.starting_residue_for_semantics(ValuationSemantics::ExactWord).unwrap();
        let modulus = BigUint::from(1u64) << mod_exp;
        let n0 = res + (BigUint::from(offset) * modulus);

        let pref1 = AffinePrefix::from_valuation_word(w1).unwrap();
        let pref2 = AffinePrefix::from_valuation_word(w2).unwrap();

        let res1 = pref1.apply(&n0).unwrap();
        let res_manual = pref2.apply(&res1).unwrap();

        let res_combined = combined.apply(&n0).unwrap();

        prop_assert_eq!(res_manual, res_combined);
    }

    #[test]
    fn prop_flatten_concatenation(
        w1 in gen_valuation_word(),
        w2 in gen_valuation_word(),
    ) {
        let seq = w1.clone().then(w2.clone());
        let flattened = seq.flatten_valuation_word();

        let mut expected = Vec::new();
        expected.extend_from_slice(w1.as_slice());
        expected.extend_from_slice(w2.as_slice());

        prop_assert_eq!(flattened.as_slice(), expected.as_slice());
    }

    #[test]
    fn prop_composition_associativity_triples(
        w1 in gen_valuation_word(),
        w2 in gen_valuation_word(),
        w3 in gen_valuation_word(),
        offset in 0u64..50u64
    ) {
        let seq_w1_w2 = w1.clone().then(w2.clone());
        let seq1 = seq_w1_w2.then(w3.clone());

        let seq_w2_w3 = w2.clone().then(w3.clone());
        let seq2 = ExecutionSequence::new(vec![w1, seq_w2_w3.flatten_valuation_word()]);

        let comb1 = seq1.combined_affine_prefix().unwrap();
        let comb2 = seq2.combined_affine_prefix().unwrap();

        let (res, mod_exp) = comb1.starting_residue_for_semantics(ValuationSemantics::ExactWord).unwrap();
        let modulus = BigUint::from(1u64) << mod_exp;
        let n0 = res + (BigUint::from(offset) * modulus);

        prop_assert_eq!(
            comb1.apply(&n0).unwrap(),
            comb2.apply(&n0).unwrap()
        );
    }

    #[test]
    fn prop_cylinder_normalization_and_bounds(
        r in 0u64..10000u64,
        exp in 1u64..20u64
    ) {
        let res = BigUint::from(r);
        let cyl = CanonicalCylinder::new(res, exp);

        // 0 <= residue < modulus
        let mod_val = BigUint::one() << exp;
        prop_assert!(cyl.residue < mod_val);

        // Modulus is power of two
        prop_assert_eq!(cyl.modulus(), mod_val.clone());

        // Contains starting residue
        prop_assert!(cyl.contains(&cyl.residue));

        // Equivalent cylinders canonicalize identically
        let shift_res = &cyl.residue + &mod_val;
        let cyl2 = CanonicalCylinder::new(shift_res, exp);
        prop_assert_eq!(cyl, cyl2);
    }
}

#[test]
fn test_empty_sequence_identity() {
    let empty_seq: ExecutionSequence<ValuationWord> = ExecutionSequence::new(vec![]);
    assert!(empty_seq.is_empty());
    assert_eq!(empty_seq.flatten_valuation_word().len(), 0);
}
