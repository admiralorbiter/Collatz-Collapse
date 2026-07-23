use collatz_affine::{
    positive_integer_realization, ConcreteUltrametricState, MacrostepData, Q1Quotient,
    QuotientRegisterMachine, ReturnTransitionOutcome, UltrametricMachine, UltrametricStepOutcome,
    ValuationWord,
};
use num_bigint::BigUint;

#[test]
fn test_state_correspondence_and_commuting_theorem() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    for k in 0u32..1000 {
        let q = Q1Quotient::from_k(BigUint::from(k));
        let state = ConcreteUltrametricState::from_q1_quotient(&q);

        // 1. Check isomorphism on positive realizability
        let k_recovered = positive_integer_realization(&state).expect("Must be realizable");
        assert_eq!(k_recovered, BigUint::from(k));

        // 2. Check commuting diagram for u and v
        assert!(
            UltrametricMachine::verify_commuting_diagram(&q, &m_u).unwrap(),
            "u failed for k={}",
            k
        );
        assert!(
            UltrametricMachine::verify_commuting_diagram(&q, &m_v).unwrap(),
            "v failed for k={}",
            k
        );
    }
}

#[test]
fn test_v_four_outcomes_benchmarks() {
    let test_cases = vec![
        (1u32, 7u64, "Nonintegral"),
        (13u32, 73u64, "IntegralEvenOutsideQ1"),
        (29u32, 161u64, "ExactButLeavesQ1"),
        (61u32, 337u64, "BasedReturn"),
    ];

    for (k, expected_u, expected_name) in test_cases {
        let q = Q1Quotient::from_k(BigUint::from(k));
        let state = ConcreteUltrametricState::from_q1_quotient(&q);

        if let ConcreteUltrametricState::Finite { unit, .. } = &state {
            assert_eq!(*unit, BigUint::from(expected_u));
        } else {
            panic!("Expected finite state");
        }

        let outcome = UltrametricMachine::step_v_resonant(&state);
        match (expected_name, outcome) {
            ("Nonintegral", UltrametricStepOutcome::NonIntegral) => {}
            ("IntegralEvenOutsideQ1", UltrametricStepOutcome::IntegralEvenOutsideQ1 { image_x }) => {
                assert_eq!(image_x, 0);
            }
            (
                "ExactButLeavesQ1",
                UltrametricStepOutcome::ExactButLeavesQ1 {
                    image_x,
                    image_unit,
                },
            ) => {
                assert_eq!(image_x, 1);
                assert_eq!(image_unit, BigUint::from(7341u32));
            }
            (
                "BasedReturn",
                UltrametricStepOutcome::BasedReturn {
                    next_state: ConcreteUltrametricState::Finite { x, unit },
                },
            ) => {
                assert_eq!(x, 11);
                assert_eq!(unit, BigUint::from(15u32));
            }
            (name, out) => panic!("Mismatch for k={}: expected {}, got {:?}", k, name, out),
        }
    }
}

#[test]
fn test_two_step_traces_uu_uv_vu_vv() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let m_u = MacrostepData::from_word(u_word).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let paths = vec![
        (
            "uu",
            23u32,
            vec![m_u.clone(), m_u.clone()],
            vec![39u32, 66u32],
        ),
        (
            "uv",
            6711u32,
            vec![m_u.clone(), m_v.clone()],
            vec![11325u32, 16125u32],
        ),
        (
            "vu",
            61u32,
            vec![m_v.clone(), m_u.clone()],
            vec![87u32, 147u32],
        ),
        (
            "vv",
            175165u32,
            vec![m_v.clone(), m_v.clone()],
            vec![249405u32, 355110u32],
        ),
    ];

    for (label, k_start, steps, expected_k_trace) in paths {
        let mut curr_q = Q1Quotient::from_k(BigUint::from(k_start));
        let mut curr_ultra = ConcreteUltrametricState::from_q1_quotient(&curr_q);

        for (idx, step_m) in steps.iter().enumerate() {
            let q_out = QuotientRegisterMachine::eval_transition(step_m, &curr_q).unwrap();
            let ultra_out = if step_m.odd_steps() == 3 {
                UltrametricMachine::step_u(&curr_ultra)
            } else {
                UltrametricMachine::step_v_resonant(&curr_ultra)
            };

            match (q_out, ultra_out) {
                (
                    ReturnTransitionOutcome::BasedReturn { next_k, .. },
                    UltrametricStepOutcome::BasedReturn { next_state },
                ) => {
                    let expected_k = BigUint::from(expected_k_trace[idx]);
                    assert_eq!(
                        next_k.value(),
                        &expected_k,
                        "Path {} step {} k mismatch",
                        label,
                        idx
                    );

                    let expected_ultra = ConcreteUltrametricState::from_q1_quotient(&next_k);
                    assert_eq!(
                        next_state, expected_ultra,
                        "Path {} step {} ultrametric state mismatch",
                        label, idx
                    );
                    curr_q = next_k;
                    curr_ultra = next_state;
                }
                _ => panic!("Path {} step {} failed based return", label, idx),
            }
        }
    }
}

#[test]
fn test_infinity_behavior() {
    let inf = ConcreteUltrametricState::Infinity;
    // u(Infinity) = Infinity
    assert_eq!(
        UltrametricMachine::step_u(&inf),
        UltrametricStepOutcome::BasedReturn {
            next_state: ConcreteUltrametricState::Infinity
        }
    );
    // v(Infinity) = NonIntegral
    assert_eq!(
        UltrametricMachine::step_v_resonant(&inf),
        UltrametricStepOutcome::NonIntegral
    );
}

#[test]
fn test_refinement_counterexamples_mod_512_and_4096() {
    // Pair 1: U1 = 337 (k = 61), U2 = 5969 (k = 1085)
    let q1 = Q1Quotient::from_k(BigUint::from(61u32)); // U1 = 337
    let q2 = Q1Quotient::from_k(BigUint::from(61u32 + 256u32 * 4u32)); // U2 = 5969

    let s1 = ConcreteUltrametricState::from_q1_quotient(&q1);
    let s2 = ConcreteUltrametricState::from_q1_quotient(&q2);

    let out1 = UltrametricMachine::step_v_resonant(&s1);
    let out2 = UltrametricMachine::step_v_resonant(&s2);

    let x1 = match out1 {
        UltrametricStepOutcome::BasedReturn {
            next_state: ConcreteUltrametricState::Finite { x, .. },
        } => x,
        _ => panic!(),
    };

    let x2 = match out2 {
        UltrametricStepOutcome::BasedReturn {
            next_state: ConcreteUltrametricState::Finite { x, .. },
        } => x,
        _ => panic!(),
    };

    assert_eq!(x1, 11);
    assert_eq!(x2, 6);

    // Pair 2: U1 = 337 (k = 61), U2 = 45393 (k = 8253)
    let q3 = Q1Quotient::from_k(BigUint::from(61u32 + 256u32 * 32u32)); // U2 = 45393
    let s3 = ConcreteUltrametricState::from_q1_quotient(&q3);
    let out3 = UltrametricMachine::step_v_resonant(&s3);

    let x3 = match out3 {
        UltrametricStepOutcome::BasedReturn {
            next_state: ConcreteUltrametricState::Finite { x, .. },
        } => x,
        _ => panic!(),
    };

    assert_eq!(x1, 11); // X11
    assert_eq!(x3, 9); // X9
}
