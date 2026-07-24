use collatz_affine::canonical_math::{
    canonical_branch, compile_exact_word_cylinder, compile_semantic_return, compute_alpha,
    compute_eta_for_transition, compute_word_affine_destination_pullback,
    verify_coboundary_reconciliation, verify_core_intertwining,
    verify_live_quotient_intertwining, CertifiedJ0Q1Return, J0CertificationError,
    LiveBlockConstant, OrdinaryOdd, QuotientRegisterState, ValuationWord,
};
use collatz_affine::{
    verify_canonical_return, verify_prefix_cylinder_fidelity, CaptureEvent,
    OrdinaryToCanonicalPrefixExtractor, ReturnFailure,
};
use num_bigint::{BigInt, BigUint};

#[test]
fn test_compile_exact_word_cylinder_j0() {
    // Valuation word w_0 = [1, 1, 2, 1, 2, 2] (k=6, B=9).
    // Backward pullback derives exact-word source cylinder \rho_{w_0} = 935 \pmod{1024}.
    let word_w0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let exact_cyl = compile_exact_word_cylinder(&word_w0).unwrap();

    assert_eq!(exact_cyl.residue, BigUint::from(935u32));
    assert_eq!(exact_cyl.modulus, BigUint::from(1024u32));
}

#[test]
fn test_compile_semantic_return_j0_q1_compatible() {
    // Word w_0 = [1, 1, 2, 1, 2, 2], destination Q1: r_t = 7, q = 5 bits.
    // Exact word cylinder = 935 mod 1024. Pullback cylinder = 1959 mod 16384.
    // 1959 mod 1024 = 935 => COMPATIBLE! Refined cylinder = 1959 mod 16384.
    let word_w0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let compiled = compile_semantic_return(&word_w0, 7, 5).unwrap();

    assert!(compiled.is_compatible);
    assert_eq!(compiled.exact_word_residue, BigUint::from(935u32));
    assert_eq!(compiled.exact_word_modulus, BigUint::from(1024u32));
    assert_eq!(compiled.refined_source_residue, BigUint::from(1959u32));
    assert_eq!(compiled.refined_source_modulus, BigUint::from(16384u32));
}

#[test]
fn test_compile_semantic_return_j0_even_target_incompatible() {
    // Word w_0 = [1, 1, 2, 1, 2, 2], target section r_t = 2 (even target residue!).
    // 18 s \equiv 11 (mod 32) has NO solutions! => INCOMPATIBLE!
    let word_w0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let compiled = compile_semantic_return(&word_w0, 2, 5).unwrap();

    assert!(!compiled.is_compatible);
}

#[test]
fn test_generic_destination_pullback_cylinder_j0() {
    // Valuation word w_0 = [1, 1, 2, 1, 2, 2] (k=6, B=9, \alpha_{w_0} = 881).
    // Destination section Q1: r_t = 7, q = 5 bits (mod 32).
    // Formula: \sigma_{w, r_t} = Q_w^{-1} (2^B r_t - \alpha_w) \pmod{2^{B + q}}
    let word_w0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let (sigma, modulus) = compute_word_affine_destination_pullback(&word_w0, 7, 5).unwrap();

    assert_eq!(sigma, BigInt::from(1959u32));
    assert_eq!(modulus, BigInt::from(16384u32));
}

#[test]
fn test_certified_j0_q1_return_try_from_source_t0_to_t3() {
    // Deterministic source-only constructor test for n(t) = 1959 + 16384 * t
    for t in 0..=3 {
        let n_val = 1959u64 + 16384u64 * (t as u64);
        let n_prime_val = 2791u64 + 23328u64 * (t as u64);

        let source_odd = OrdinaryOdd::new(BigUint::from(n_val)).unwrap();
        let cert = CertifiedJ0Q1Return::try_from_source(&source_odd);

        assert!(cert.is_ok(), "Certification failed for t = {}: {:?}", t, cert.err());

        let cert_val = cert.unwrap();
        assert_eq!(*cert_val.target().value(), BigUint::from(n_prime_val));
        assert_eq!(cert_val.word().exponents(), &[1, 1, 2, 1, 2, 2]);
        assert_eq!(cert_val.canonical_source().0, BigInt::from(342u64 + 512u64 * (t as u64)));
        assert_eq!(cert_val.canonical_target().0, BigInt::from(487u64 + 729u64 * (t as u64)));
    }
}

#[test]
fn test_certified_j0_q1_return_structured_rejections() {
    let word_w0 = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();

    // Rejection 1: n = 935 -> 1333 (target is 1333 = 21 mod 32, not 7!)
    let src_935 = OrdinaryOdd::new(BigUint::from(935u64)).unwrap();
    let tgt_1333 = OrdinaryOdd::new(BigUint::from(1333u64)).unwrap();
    let res1 = CertifiedJ0Q1Return::try_from_transition(&src_935, &tgt_1333, &word_w0);
    assert!(matches!(res1, Err(J0CertificationError::RefinedCylinderMismatch { .. }) | Err(J0CertificationError::DestinationPhaseMismatch { .. })));

    // Rejection 2: n = 423 -> 635 (423 = 423 mod 512, but 423 = 423 mod 1024 != 935 mod 1024!)
    let src_423 = OrdinaryOdd::new(BigUint::from(423u64)).unwrap();
    let tgt_635 = OrdinaryOdd::new(BigUint::from(635u64)).unwrap();
    let res2 = CertifiedJ0Q1Return::try_from_transition(&src_423, &tgt_635, &word_w0);
    assert!(matches!(res2, Err(J0CertificationError::RefinedCylinderMismatch { .. })));

    // Rejection 3: Wrong valuation word
    let src_1959 = OrdinaryOdd::new(BigUint::from(1959u64)).unwrap();
    let tgt_2791 = OrdinaryOdd::new(BigUint::from(2791u64)).unwrap();
    let wrong_word = ValuationWord::new(vec![1, 1, 1, 1, 1, 1]).unwrap();
    let res3 = CertifiedJ0Q1Return::try_from_transition(&src_1959, &tgt_2791, &wrong_word);
    assert!(matches!(res3, Err(J0CertificationError::ExactWordMismatch { .. })));
}

#[test]
fn test_1959_to_2791_live_quotient_intertwining_t0() {
    let source_odd = OrdinaryOdd::new(BigUint::from(1959u64)).unwrap();
    let target_odd = OrdinaryOdd::new(BigUint::from(2791u64)).unwrap();

    let source_k = QuotientRegisterState::from_ordinary_odd(&source_odd);
    let target_k = QuotientRegisterState::from_ordinary_odd(&target_odd);

    assert_eq!(*source_k.quotient(), BigInt::from(61u32));
    assert_eq!(*target_k.quotient(), BigInt::from(87u32));

    let word = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let alpha = compute_alpha(&word);
    assert_eq!(alpha, BigInt::from(881u32));

    let eta = compute_eta_for_transition(&word, &source_odd, &target_odd).unwrap();
    assert_eq!(eta.0, BigInt::from(75u32));

    assert!(verify_live_quotient_intertwining(&source_k, &target_k, &word, &eta).is_ok());
}

#[test]
fn test_18343_to_26119_live_quotient_intertwining_t1() {
    let source_odd = OrdinaryOdd::new(BigUint::from(18343u64)).unwrap();
    let target_odd = OrdinaryOdd::new(BigUint::from(26119u64)).unwrap();

    let source_k = QuotientRegisterState::from_ordinary_odd(&source_odd);
    let target_k = QuotientRegisterState::from_ordinary_odd(&target_odd);

    assert_eq!(*source_k.quotient(), BigInt::from(573u32));
    assert_eq!(*target_k.quotient(), BigInt::from(816u32));

    let word = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let eta = compute_eta_for_transition(&word, &source_odd, &target_odd).unwrap();
    assert_eq!(eta.0, BigInt::from(75u32));

    assert!(verify_live_quotient_intertwining(&source_k, &target_k, &word, &eta).is_ok());
}

#[test]
fn test_j0_to_j3_core_intertwining() {
    for gap in 0..=3 {
        let branch = canonical_branch(gap).unwrap();
        assert!(verify_core_intertwining(&branch).is_ok());
    }
}

#[test]
fn test_coboundary_reconciliation_j0() {
    let eta = LiveBlockConstant(BigInt::from(75u32));
    let branch = canonical_branch(0).unwrap();

    let m = BigInt::from(branch.modulus.clone());
    let q = BigInt::from(branch.multiplier.clone());

    let a = BigInt::from(1u32);
    let b_s = BigInt::from(281u32);
    let b_t = BigInt::from(400u32);

    assert!(verify_coboundary_reconciliation(&eta, &branch.beta, &m, &q, &a, &b_s, &b_t).is_ok());
}

#[test]
fn test_differential_validation_small_integers() {
    for n in (3..1000).step_by(2) {
        let base = BigUint::from(n as u64);
        let mut extractor = OrdinaryToCanonicalPrefixExtractor::new(base.clone());

        for _ in 0..10 {
            let event = extractor.next_event(200);
            match &event {
                CaptureEvent::HitOne => break,
                CaptureEvent::DescendedBelowBase { value, base } => {
                    assert!(value < base);
                    break;
                }
                CaptureEvent::Return { witness, gap, .. } => {
                    assert!(verify_canonical_return(witness).is_ok());
                    assert_eq!(witness.ordinary_exponents.len() as u32, 6 + 3 * gap);
                    assert_eq!(*gap, (witness.ordinary_exponents.iter().sum::<u32>() - 9) / 4);
                }
                CaptureEvent::CandidateRejected { candidate } => {
                    assert!(matches!(
                        candidate.reason,
                        ReturnFailure::IncorrectOddStepCount { .. } | ReturnFailure::SourceResidueMismatch { .. }
                    ));
                }
                CaptureEvent::Escape { witness } => {
                    assert!(matches!(witness.failure_reason, ReturnFailure::SourceResidueMismatch { .. }));
                    break;
                }
                CaptureEvent::SearchLimitReached { .. } => {
                    break;
                }
            }
        }
    }
}

#[test]
fn test_27_candidate_rejections_and_search_limit() {
    let base = BigUint::from(27u64);
    let mut extractor = OrdinaryToCanonicalPrefixExtractor::new(base);
    let event = extractor.next_event(27);

    match event {
        CaptureEvent::SearchLimitReached { steps_evaluated, rejections } => {
            assert_eq!(steps_evaluated, 27);
            assert!(!rejections.is_empty());
            assert_eq!(rejections[0].gap, 0);
            assert!(matches!(
                rejections[0].reason,
                ReturnFailure::IncorrectOddStepCount { expected_k: 6, actual_k: 7 }
            ));
        }
        other => panic!("Expected SearchLimitReached for n = 27, got {:?}", other),
    }
}

#[test]
fn test_canonical_ready_integer_1959_return() {
    let base = BigUint::from(1959u64);
    let mut extractor = OrdinaryToCanonicalPrefixExtractor::new(base.clone());
    let trace = extractor.extract_prefix(1, 500);

    assert!(!trace.events.is_empty());
    let mut return_witnesses = Vec::new();
    if let CaptureEvent::Return { witness, gap, .. } = &trace.events[0] {
        assert_eq!(*gap, 0);
        assert_eq!(witness.ordinary_exponents.len(), 6);
        assert!(verify_canonical_return(witness).is_ok());
        return_witnesses.push(witness.clone());
    }

    assert!(verify_prefix_cylinder_fidelity(&base, &return_witnesses));
}

#[test]
fn test_engine_generated_baseline_and_adversarial_mutations() {
    let base = BigUint::from(1959u64);
    let mut extractor = OrdinaryToCanonicalPrefixExtractor::new(base);
    let trace = extractor.extract_prefix(1, 500);

    assert!(!trace.events.is_empty());
    let baseline_witness = match &trace.events[0] {
        CaptureEvent::Return { witness, .. } => witness.clone(),
        other => panic!("Expected Return for n = 1959, got {:?}", other),
    };

    assert!(verify_canonical_return(&baseline_witness).is_ok());

    let mut mutated_exponent = baseline_witness.clone();
    mutated_exponent.gap += 1;
    assert!(matches!(
        verify_canonical_return(&mutated_exponent),
        Err(ReturnFailure::IncorrectTotalExponent { .. })
            | Err(ReturnFailure::IncorrectOddStepCount { .. })
    ));

    let mut mutated_valuation = baseline_witness.clone();
    mutated_valuation.ordinary_exponents[0] += 1;
    assert!(matches!(
        verify_canonical_return(&mutated_valuation),
        Err(ReturnFailure::InvalidValuationExponent { .. })
            | Err(ReturnFailure::IncorrectTotalExponent { .. })
    ));

    let mut mutated_target = baseline_witness.clone();
    mutated_target.target_odd += 2u32;
    assert!(matches!(
        verify_canonical_return(&mutated_target),
        Err(ReturnFailure::EndpointIntertwiningMismatch { .. })
    ));

    let mut mutated_k = baseline_witness.clone();
    mutated_k.ordinary_exponents.push(1);
    assert!(matches!(
        verify_canonical_return(&mutated_k),
        Err(ReturnFailure::IncorrectTotalExponent { .. })
            | Err(ReturnFailure::IncorrectOddStepCount { .. })
    ));
}
