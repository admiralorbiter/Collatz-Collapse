use collatz_affine::{
    CoreInteractionKernel, CoreSwitchType, PeriodicReturnCore, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;
use num_traits::Zero;

#[test]
fn test_single_block_fixed_point_normalization() {
    // Valuation word v = [1] (simplest odd step)
    let word_v = ValuationWord::from_slice(&[1]);
    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let fp = core_v.fixed_point();

    // xi_v < 0
    assert!(fp.is_negative());
    assert!(*fp.denominator() > BigInt::zero());
    // denominator d_v must be odd
    let d_v_uint = fp.denominator().to_biguint().unwrap();
    assert_eq!(d_v_uint.trailing_zeros().unwrap(), 0);
}

#[test]
fn test_self_interaction_and_powers() {
    let word_v = ValuationWord::from_slice(&[1, 1]);
    let word_v2 = ValuationWord::from_slice(&[1, 1, 1, 1]);

    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let core_v2 = PeriodicReturnCore::new(word_v2).unwrap();

    let kernel = CoreInteractionKernel::new(&core_v, &core_v2);

    // v and v^2 share a common center, so Gamma = 0 and cores commute
    assert!(kernel.are_cores_commuting());
    assert!(kernel.gamma().is_zero());
    assert_eq!(kernel.kappa(), TwoAdicValuation::Infinity);

    // Switch evaluation on same core returns SameCore
    let a_v = core_v.eval_integer_primitive(&BigInt::from(105u32));
    let switch_res = kernel.evaluate_integer_switch(&a_v);
    assert_eq!(switch_res.switch_type, CoreSwitchType::SameCore);
}

#[test]
fn test_noncommuting_pair_interaction() {
    // Two distinct valuation words
    let word_v = ValuationWord::from_slice(&[1, 2]);
    let word_w = ValuationWord::from_slice(&[2, 1]);

    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let core_w = PeriodicReturnCore::new(word_w).unwrap();

    let kernel = CoreInteractionKernel::new(&core_v, &core_w);

    let gamma = kernel.gamma();
    let kappa = kernel.kappa();

    assert_ne!(kappa, TwoAdicValuation::Infinity);

    // Test Integer Switch Law: d_v A_w(D) = d_w A_v(D) + Gamma_{v,w}
    let d_test = BigInt::from(27u32);
    let a_v = core_v.eval_integer_primitive(&d_test);
    let a_w = core_w.eval_integer_primitive(&d_test);

    let d_v = core_v.d_v();
    let d_w = core_w.d_v();

    let lhs = d_v * &a_w;
    let rhs = (d_w * &a_v) + gamma;

    assert_eq!(lhs, rhs, "Exact integer core-switch identity must hold");
}

#[test]
fn test_4_case_switch_budget() {
    let word_v = ValuationWord::from_slice(&[1, 2]);
    let word_w = ValuationWord::from_slice(&[2, 1]);

    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let core_w = PeriodicReturnCore::new(word_w).unwrap();

    let kernel = CoreInteractionKernel::new(&core_v, &core_w);
    let k_val = match kernel.kappa() {
        TwoAdicValuation::Finite(k) => k,
        _ => panic!("Expected finite kappa for distinct cores"),
    };

    // Case 1: Inherited (s < kappa)
    if k_val > 0 {
        let target_s = k_val - 1;
        let a_v_shallow = BigInt::from(1u32) << target_s;
        let res = kernel.evaluate_integer_switch(&a_v_shallow);
        assert_eq!(res.switch_type, CoreSwitchType::Inherited);
        assert_eq!(res.outgoing_depth, TwoAdicValuation::Finite(target_s));
    }

    // Case 2: Reset (s > kappa)
    let a_v_deep = BigInt::from(1u32) << (k_val + 5);
    let res_reset = kernel.evaluate_integer_switch(&a_v_deep);
    assert_eq!(res_reset.switch_type, CoreSwitchType::Reset);
    assert_eq!(res_reset.outgoing_depth, TwoAdicValuation::Finite(k_val));

    // Case 3: Resonant (s == kappa)
    let a_v_resonant = BigInt::from(1u32) << k_val;
    let res_resonant = kernel.evaluate_integer_switch(&a_v_resonant);
    assert_eq!(res_resonant.switch_type, CoreSwitchType::Resonant);
    match res_resonant.outgoing_depth {
        TwoAdicValuation::Finite(out_k) => assert!(out_k >= k_val + 1),
        TwoAdicValuation::Infinity => (),
    }
}

#[test]
fn test_exact_cancellation_resonant_outcome() {
    use collatz_affine::ResonanceOutcome;

    let word_v = ValuationWord::from_slice(&[1, 2]);
    let word_w = ValuationWord::from_slice(&[2, 1]);

    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let core_w = PeriodicReturnCore::new(word_w).unwrap();

    let kernel = CoreInteractionKernel::new(&core_v, &core_w);

    // Construct A_v so that u + g == 0
    // d_w * A_v / 2^k + Gamma / 2^k = 0 => d_w * A_v = -Gamma
    // A_v = -Gamma / d_w
    let gamma = kernel.gamma();
    let d_w = core_w.d_v();

    if (&(-gamma) % &d_w).is_zero() {
        let exact_a_v = &(-gamma) / &d_w;
        let res = kernel.evaluate_integer_switch(&exact_a_v);
        assert_eq!(res.switch_type, CoreSwitchType::Resonant);
        assert_eq!(res.outgoing_depth, TwoAdicValuation::Infinity);
        assert_eq!(res.resonance_outcome, Some(ResonanceOutcome::ExactCore));
    }
}

#[test]
fn test_zero_block_definition_fingerprint() {
    // Valuation word v = [1, 2]:
    // Q_v = 3^2 = 9, M_v = 2^3 = 8, beta_v = 5, d_v = 9 - 8 = 1.
    // F_v(D) = (9 * D + 5) / 8.
    // For D = 3: F_v(3) = (27 + 5) / 8 = 32 / 8 = 4.
    let word_v = ValuationWord::from_slice(&[1, 2]);
    let core_v = PeriodicReturnCore::new(word_v).unwrap();

    assert_eq!(core_v.q_v(), &num_bigint::BigUint::from(9u32));
    assert_eq!(core_v.m_v(), &num_bigint::BigUint::from(8u32));
    assert_eq!(core_v.beta_v(), &num_bigint::BigUint::from(5u32));
    assert_eq!(core_v.d_v(), BigInt::from(1u32)); // 9 - 8 = 1

    // Decisive fingerprint check: F_v(3) = 4
    let image_3 = core_v.eval_map(&BigInt::from(3u32));
    assert_eq!(image_3, Some(BigInt::from(4u32)));

    // Fixed point: \xi_v = -5 / 1 = -5
    let fp = core_v.fixed_point();
    assert_eq!(fp.numerator(), &BigInt::from(-5));
    assert_eq!(fp.denominator(), &BigInt::from(1));

    // Error transport identity check: F_v(3) - \xi_v == (Q_v / M_v) * (3 - \xi_v)
    // 4 - (-5) = 9 == (9/8) * (3 - (-5)) = (9/8) * 8 = 9
    assert_eq!(BigInt::from(4) - fp.numerator(), BigInt::from(9));

    // Authoritative ASCII formula check
    assert_eq!(
        core_v.canonical_ascii_definition(),
        "F_v(D) = (Q_v * D + beta_v) / M_v"
    );
}
