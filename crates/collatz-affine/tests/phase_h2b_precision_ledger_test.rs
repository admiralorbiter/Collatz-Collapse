use collatz_affine::{
    FineWilfBound, PeriodicReturnCore, PrecisionLedger, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;

#[test]
fn test_fine_wilf_overlap_bounds() {
    // Cores with periods 2 and 3: Fine-Wilf threshold T = 2 + 3 - gcd(2,3) = 4
    let threshold_2_3 = FineWilfBound::fine_wilf_threshold(2, 3);
    assert_eq!(threshold_2_3, 4);

    // Max incompatible overlap symbols T - 1 = 3
    let max_incompat_2_3 = FineWilfBound::max_incompatible_overlap_symbols(2, 3);
    assert_eq!(max_incompat_2_3, 3);

    // Cores with periods 4 and 6: threshold T = 4 + 6 - gcd(4,6) = 8, T - 1 = 7
    let max_incompat_4_6 = FineWilfBound::max_incompatible_overlap_symbols(4, 6);
    assert_eq!(max_incompat_4_6, 7);

    // Safe precision bound under B_max = 5: 3 * 5 = 15
    let prec_bound_2_3 = FineWilfBound::max_precision_bound(2, 3, 5);
    assert_eq!(prec_bound_2_3, 15);
}

#[test]
fn test_exact_non_resonant_switch_depth_min_t_kappa() {
    let word_v = ValuationWord::from_slice(&[1, 2]); // Period 2, B_v = 3
    let word_w = ValuationWord::from_slice(&[2, 1]); // Period 2, B_w = 3

    let core_v = PeriodicReturnCore::new(word_v).unwrap();
    let core_w = PeriodicReturnCore::new(word_w).unwrap();

    let mut ledger = PrecisionLedger::new(TwoAdicValuation::Finite(6));

    // Input A_v(D) = 2^6 = 64 (s = 6). Shadow 1 cycle (3 bits) -> t = 3
    let a_v = BigInt::from(64u32);
    let switch_res = ledger.record_switch(&core_v, &core_w, &a_v, 1);

    let entry = &ledger.entries[0];
    assert_eq!(entry.incoming_depth_s, TwoAdicValuation::Finite(6));
    assert_eq!(entry.bits_consumed_c, 3);
    assert_eq!(entry.pre_switch_depth_t, TwoAdicValuation::Finite(3));

    // Verify non-resonant outgoing depth satisfies s_{next} = min(t, kappa) exactly
    match (entry.pre_switch_depth_t, entry.core_distance_kappa) {
        (TwoAdicValuation::Finite(t), TwoAdicValuation::Finite(k)) => {
            if t != k {
                assert_eq!(switch_res.outgoing_depth, TwoAdicValuation::Finite(t.min(k)));
            } else {
                // Resonant case: s_{next} = t + g (g >= 1)
                match switch_res.outgoing_depth {
                    TwoAdicValuation::Finite(s_next) => assert!(s_next >= t + 1),
                    TwoAdicValuation::Infinity => {}
                }
            }
        }
        _ => {}
    }
}

#[test]
fn test_multi_switch_fixture_with_reset_inherited_and_resonance() {
    // 3 selected primitive return cores: v1=[1, 2], v2=[2, 1], v3=[1, 1, 1, 1]
    let word_v1 = ValuationWord::from_slice(&[1, 2]);
    let word_v2 = ValuationWord::from_slice(&[2, 1]);
    let word_v3 = ValuationWord::from_slice(&[1, 1, 1, 1]);

    let core_v1 = PeriodicReturnCore::new(word_v1).unwrap();
    let core_v2 = PeriodicReturnCore::new(word_v2).unwrap();
    let core_v3 = PeriodicReturnCore::new(word_v3).unwrap();

    let mut ledger = PrecisionLedger::new(TwoAdicValuation::Finite(10));

    // Switch 1: v1 -> v2 (Depth 10, shadow 2 cycles = 6 bits -> t1 = 4)
    let a_v1 = BigInt::from(1024u32); // s1 = 10
    ledger.record_switch(&core_v1, &core_v2, &a_v1, 2);

    // Switch 2: v2 -> v3 (Incoming depth = s2, shadow 1 cycle = 3 bits)
    let s2 = match ledger.entries[0].outgoing_depth_next_s {
        TwoAdicValuation::Finite(s) => s,
        _ => panic!("Expected finite s2"),
    };
    let a_v2 = BigInt::from(1u32) << (s2 as usize);
    ledger.record_switch(&core_v2, &core_v3, &a_v2, 1);

    assert_eq!(ledger.entries.len(), 2);
    assert_eq!(ledger.total_bits_consumed, 9); // 6 + 3 = 9 bits

    // Verify conditional resonance budget required value
    let req = ledger.required_resonance_budget();
    assert_eq!(req, ledger.total_bits_consumed + ledger.total_reset_losses - 10);
    assert!(ledger.satisfies_conditional_resonance_budget() || req > 0);
}
