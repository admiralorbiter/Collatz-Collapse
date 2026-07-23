use collatz_affine::{AffinePrefix, ExactWordCylinder, ThenSequence, ValuationWord};
use collatz_cegar::{BasedReturnCylinder, GuardedPathCylinder, StateId, StateMembership};
use num_bigint::BigUint;

#[test]
fn test_group_b_complete_guarded_path_semantics() {
    let u = ValuationWord::from_slice(&[1, 1, 2]);
    let v = ValuationWord::from_slice(&[1, 1, 2, 1, 2, 2]);

    let seq_uv = u.clone().then(v.clone());
    let seq_vu = v.clone().then(u.clone());

    let q1 = StateMembership::q1();

    let guarded_uv = GuardedPathCylinder::compute(seq_uv, q1.clone()).unwrap();
    let guarded_vu = GuardedPathCylinder::compute(seq_vu, q1).unwrap();

    // Complete guarded return cylinders
    assert_eq!(guarded_uv.source.residue, BigUint::from(214759u32));
    assert_eq!(guarded_uv.source.modulus_exponent, 18); // mod 262144

    assert_eq!(guarded_vu.source.residue, BigUint::from(1959u32));
    assert_eq!(guarded_vu.source.modulus_exponent, 18); // mod 262144
}

#[test]
fn test_negative_regression_exact_vs_guarded() {
    let u = ValuationWord::from_slice(&[1, 1, 2]);
    let v = ValuationWord::from_slice(&[1, 1, 2, 1, 2, 2]);

    let seq_uv = u.clone().then(v.clone());
    let seq_vu = v.clone().then(u.clone());

    let q1 = StateMembership::q1();
    let pref_uv = seq_uv.combined_affine_prefix().unwrap();
    let pref_vu = seq_vu.combined_affine_prefix().unwrap();

    let exact_uv = ExactWordCylinder::from_valuation_word(seq_uv.flatten_valuation_word()).unwrap();
    let exact_vu = ExactWordCylinder::from_valuation_word(seq_vu.flatten_valuation_word()).unwrap();

    let guarded_vu = GuardedPathCylinder::compute(seq_vu, q1.clone()).unwrap();

    // 1767 mod 16384 executes exact word [u, v], but final image is 4249 != 7 mod 32
    let n1767 = BigUint::from(1767u32);
    assert!(exact_uv.source.contains(&n1767));
    let img1767 = pref_uv.apply(&n1767).unwrap();
    assert_eq!(img1767, BigUint::from(4249u32));
    assert_eq!(&img1767 % 32u32, BigUint::from(25u32));
    assert!(!q1.contains(&img1767));

    // 1959 is in both exact_vu and guarded_vu
    let n1959 = BigUint::from(1959u32);
    assert!(exact_vu.source.contains(&n1959));
    assert!(guarded_vu.source.contains(&n1959));

    // 18343 = 1959 + 16384 is in exact_vu, but NOT in guarded_vu because final image is 44077 = 13 mod 32
    let n18343 = BigUint::from(18343u32);
    assert!(exact_vu.source.contains(&n18343));
    let img18343 = pref_vu.apply(&n18343).unwrap();
    assert_eq!(img18343, BigUint::from(44077u32));
    assert_eq!(&img18343 % 32u32, BigUint::from(13u32));
    assert!(!q1.contains(&img18343));
    assert!(!guarded_vu.source.contains(&n18343));
}

#[test]
fn test_strict_inclusion_exact_v_vs_based_v_return() {
    let v = ValuationWord::from_slice(&[1, 1, 2, 1, 2, 2]);
    let q1 = StateMembership::q1();

    let exact_v = ExactWordCylinder::from_valuation_word(v.clone()).unwrap();
    // v exact execution: n = 935 mod 1024 (k = 29 mod 32, U = 1 mod 16)
    assert_eq!(exact_v.source.residue, BigUint::from(935u32));
    assert_eq!(exact_v.source.modulus_exponent, 10); // mod 1024

    // v based return: n = 1959 mod 16384 (k = 61 mod 512, U = 81 mod 256)
    let based_v = BasedReturnCylinder {
        base_state: StateId::Q1,
        word: v.clone(),
        source: collatz_affine::CanonicalCylinder::new(BigUint::from(1959u32), 14),
        target: q1.clone(),
    };

    assert_eq!(based_v.source.residue, BigUint::from(1959u32));
    assert_eq!(based_v.source.modulus_exponent, 14); // mod 16384

    // 1959 is in exact_v.source (1959 = 935 + 1024)
    assert!(exact_v.source.contains(&based_v.source.residue));

    // Strict containment: based return is strictly smaller than exact word execution
    assert!(based_v.source.modulus_exponent > exact_v.source.modulus_exponent);

    // Witness check: 935 executes exact word v, yielding 1333 == 21 mod 32 != 7 mod 32
    let pref_v = AffinePrefix::from_valuation_word(v).unwrap();
    let n935 = BigUint::from(935u32);
    assert!(exact_v.source.contains(&n935));
    let img935 = pref_v.apply(&n935).unwrap();
    assert_eq!(img935, BigUint::from(1333u32));
    assert_eq!(&img935 % 32u32, BigUint::from(21u32));
    assert!(!q1.contains(&img935));
}

#[test]
fn test_strict_inclusion_exact_u_vs_based_u_return() {
    let u = ValuationWord::from_slice(&[1, 1, 2]);
    let q1 = StateMembership::q1();

    let exact_u = ExactWordCylinder::from_valuation_word(u.clone()).unwrap();
    // u exact execution: n = 7 mod 32 (whole Q1 state)
    assert_eq!(exact_u.source.residue, BigUint::from(7u32));
    assert_eq!(exact_u.source.modulus_exponent, 5); // mod 32

    // u based return: n = 231 mod 512 (k = 7 mod 16)
    let based_u = BasedReturnCylinder {
        base_state: StateId::Q1,
        word: u.clone(),
        source: collatz_affine::CanonicalCylinder::new(BigUint::from(231u32), 9),
        target: q1.clone(),
    };

    assert_eq!(based_u.source.residue, BigUint::from(231u32));
    assert_eq!(based_u.source.modulus_exponent, 9); // mod 512

    // 231 is in exact_u.source
    assert!(exact_u.source.contains(&based_u.source.residue));

    // Strict containment: based return (mod 512) is strictly smaller than exact word execution (mod 32)
    assert!(based_u.source.modulus_exponent > exact_u.source.modulus_exponent);

    // Negative witness check: n = 7 in ExactWord(u), but 7 -> 13 = 13 mod 32 != 7 mod 32
    let pref_u = AffinePrefix::from_valuation_word(u).unwrap();
    let n7 = BigUint::from(7u32);
    assert!(exact_u.source.contains(&n7));
    let img7 = pref_u.apply(&n7).unwrap();
    assert_eq!(img7, BigUint::from(13u32));
    assert_eq!(&img7 % 32u32, BigUint::from(13u32));
    assert!(!q1.contains(&img7));
}
