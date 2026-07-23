use collatz_affine::{
    classify_guarded_return, compose_guarded_path, CanonicalCylinder, MacrostepData, Q1Quotient,
    ValuationWord,
};
use collatz_affine::UltrametricMachine;
use num_bigint::BigUint;

#[test]
fn test_classify_u_and_v_guarded_return() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u_word).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let q1_base = CanonicalCylinder::new(BigUint::from(7u32), 5); // 7 mod 32

    // Classify u
    let class_u = classify_guarded_return(&m_u, &q1_base).unwrap();
    assert_eq!(class_u.exact_word_cylinder.residue, BigUint::from(7u32));
    assert_eq!(class_u.exact_word_cylinder.modulus_exponent, 5);
    assert_eq!(class_u.based_return_cylinder.residue, BigUint::from(231u32));
    assert_eq!(class_u.based_return_cylinder.modulus_exponent, 9); // 4 + 5 = 9 (512)
    assert_eq!(class_u.positive_image.start, BigUint::from(391u32));
    assert_eq!(class_u.positive_image.step, BigUint::from(864u32)); // 27 * 32

    // Classify v
    let class_v = classify_guarded_return(&m_v, &q1_base).unwrap();
    assert_eq!(class_v.exact_word_cylinder.residue, BigUint::from(935u32));
    assert_eq!(class_v.exact_word_cylinder.modulus_exponent, 10);
    assert_eq!(
        class_v.based_return_cylinder.residue,
        BigUint::from(1959u32)
    );
    assert_eq!(class_v.based_return_cylinder.modulus_exponent, 14); // 9 + 5 = 14 (16384)
    assert_eq!(class_v.positive_image.start, BigUint::from(2791u32));
    assert_eq!(class_v.positive_image.step, BigUint::from(23328u32)); // 729 * 32
}

#[test]
fn test_compose_guarded_path_uv_and_vu() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u_word).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let q1_base = CanonicalCylinder::new(BigUint::from(7u32), 5);

    // Path [u, v]
    let path_uv = compose_guarded_path(&[m_u.clone(), m_v.clone()], &q1_base).unwrap();
    assert_eq!(path_uv.source_cylinder.residue, BigUint::from(214759u32));
    assert_eq!(path_uv.source_cylinder.modulus_exponent, 18); // 262144
    assert_eq!(path_uv.quotient_guard.residue, BigUint::from(6711u32));
    assert_eq!(path_uv.quotient_guard.modulus_exponent, 13); // 8192

    // Path [v, u]
    let path_vu = compose_guarded_path(&[m_v.clone(), m_u.clone()], &q1_base).unwrap();
    assert_eq!(path_vu.source_cylinder.residue, BigUint::from(1959u32));
    assert_eq!(path_vu.source_cylinder.modulus_exponent, 18);
    assert_eq!(path_vu.quotient_guard.residue, BigUint::from(61u32));
    assert_eq!(path_vu.quotient_guard.modulus_exponent, 13);
}

#[test]
fn test_ultrametric_commuting_diagram_conformance() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u_word).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    for k in [0u32, 7, 12, 23, 29, 61, 87, 231, 1959, 6711, 175165] {
        let q = Q1Quotient::from_k(BigUint::from(k));

        assert!(
            UltrametricMachine::verify_commuting_diagram(&q, &m_u).unwrap(),
            "u commuting diagram failed for k={}",
            k
        );

        assert!(
            UltrametricMachine::verify_commuting_diagram(&q, &m_v).unwrap(),
            "v commuting diagram failed for k={}",
            k
        );
    }
}
