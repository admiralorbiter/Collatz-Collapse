use collatz_affine::{
    MacrostepData, Q1Quotient, QuotientRegisterMachine, ReturnTransitionOutcome, ValuationWord,
};
use num_bigint::{BigInt, BigUint};

#[test]
fn test_u_quotient_register_machine() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word).unwrap();

    let eta_u = QuotientRegisterMachine::compute_eta(&m_u).unwrap();
    assert_eq!(eta_u, BigInt::from(3i32));

    let rule_u = QuotientRegisterMachine::derive_rule(&m_u).unwrap();
    assert_eq!(rule_u.eta, BigInt::from(3i32));
    assert_eq!(rule_u.guard_residue, BigUint::from(7u32));
    assert_eq!(rule_u.guard_modulus_exponent, 4);

    // Test k = 7 (n = 231) => BasedReturn k' = 12, image = 391
    let q_231 = Q1Quotient::from_integer(&BigUint::from(231u32)).unwrap();
    assert_eq!(q_231.value(), &BigUint::from(7u32));

    let outcome_gen = QuotientRegisterMachine::eval_transition(&m_u, &q_231).unwrap();
    let outcome_fast = QuotientRegisterMachine::eval_u_transition(&q_231);
    assert_eq!(outcome_gen, outcome_fast);

    match outcome_fast {
        ReturnTransitionOutcome::BasedReturn { next_k, image } => {
            assert_eq!(next_k.value(), &BigUint::from(12u32));
            assert_eq!(image, BigUint::from(391u32));
        }
        _ => panic!("Expected BasedReturn for u at k = 7"),
    }

    // Test k = 1 (n = 39) => ExactButLeavesBase image = 67
    let q_39 = Q1Quotient::from_integer(&BigUint::from(39u32)).unwrap();
    let outcome_leave = QuotientRegisterMachine::eval_u_transition(&q_39);
    match outcome_leave {
        ReturnTransitionOutcome::ExactButLeavesBase { image } => {
            assert_eq!(image, BigUint::from(67u32));
        }
        _ => panic!("Expected ExactButLeavesBase for u at k = 1"),
    }
}

#[test]
fn test_v_quotient_register_machine() {
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let eta_v = QuotientRegisterMachine::compute_eta(&m_v).unwrap();
    assert_eq!(eta_v, BigInt::from(75i32));

    let rule_v = QuotientRegisterMachine::derive_rule(&m_v).unwrap();
    assert_eq!(rule_v.eta, BigInt::from(75i32));
    assert_eq!(rule_v.guard_residue, BigUint::from(61u32));
    assert_eq!(rule_v.guard_modulus_exponent, 9);

    // Test k = 61 (n = 1959) => BasedReturn k' = 87, image = 2791
    let q_1959 = Q1Quotient::from_integer(&BigUint::from(1959u32)).unwrap();
    assert_eq!(q_1959.value(), &BigUint::from(61u32));

    let outcome_gen = QuotientRegisterMachine::eval_transition(&m_v, &q_1959).unwrap();
    let outcome_fast = QuotientRegisterMachine::eval_v_transition(&q_1959);
    assert_eq!(outcome_gen, outcome_fast);

    match outcome_fast {
        ReturnTransitionOutcome::BasedReturn { next_k, image } => {
            assert_eq!(next_k.value(), &BigUint::from(87u32));
            assert_eq!(image, BigUint::from(2791u32));
        }
        _ => panic!("Expected BasedReturn for v at k = 61"),
    }

    // Test k = 29 (n = 935) => ExactButLeavesBase image = 1333
    let q_935 = Q1Quotient::from_integer(&BigUint::from(935u32)).unwrap();
    let outcome_leave = QuotientRegisterMachine::eval_v_transition(&q_935);
    match outcome_leave {
        ReturnTransitionOutcome::ExactButLeavesBase { image } => {
            assert_eq!(image, BigUint::from(1333u32));
        }
        _ => panic!("Expected ExactButLeavesBase for v at k = 29"),
    }

    // Test k = 0 (n = 7) => NotExactWord
    let q_7 = Q1Quotient::from_integer(&BigUint::from(7u32)).unwrap();
    let outcome_not_exact = QuotientRegisterMachine::eval_v_transition(&q_7);
    assert_eq!(outcome_not_exact, ReturnTransitionOutcome::NotExactWord);
}

#[test]
fn test_single_symbol_period_exhaustive_0_to_512() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let mut u_return_count = 0;
    let mut v_exact_count = 0;
    let mut v_return_count = 0;

    for k_val in 0u32..512u32 {
        let q = Q1Quotient::from_k(BigUint::from(k_val));

        // u is exact for all k
        let u_out = QuotientRegisterMachine::eval_transition(&m_u, &q).unwrap();
        assert_ne!(u_out, ReturnTransitionOutcome::NotExactWord);
        if let ReturnTransitionOutcome::BasedReturn { .. } = u_out {
            assert_eq!(k_val % 16, 7);
            u_return_count += 1;
        }

        // v status
        let v_out = QuotientRegisterMachine::eval_transition(&m_v, &q).unwrap();
        if v_out != ReturnTransitionOutcome::NotExactWord {
            assert_eq!(k_val % 32, 29);
            v_exact_count += 1;
        }
        if let ReturnTransitionOutcome::BasedReturn { .. } = v_out {
            assert_eq!(k_val, 61);
            v_return_count += 1;
        }
    }

    assert_eq!(u_return_count, 32); // 512 / 16 = 32
    assert_eq!(v_exact_count, 16); // 512 / 32 = 16
    assert_eq!(v_return_count, 1); // exactly 1 in 0..512
}

#[test]
fn test_preimage_guard_inverse_composition() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word).unwrap();
    let rule_u = QuotientRegisterMachine::derive_rule(&m_u).unwrap();

    // Preimage of v guard (g' = 61 mod 512) under u rule
    let (pred_res, pred_exp) =
        QuotientRegisterMachine::preimage_guard(&rule_u, &BigUint::from(61u32), 9);
    assert_eq!(pred_res, BigUint::from(6711u32));
    assert_eq!(pred_exp, 13); // 4 + 9 = 13 (2^13 = 8192)
}
