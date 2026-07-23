use collatz_affine::{
    detect_constant_suffix, verify_compatible_pair, verify_stabilization_certificate, LiftBlock,
    PrecisionSchedule, ProjectiveResidue, StabilizationCertificate,
};
use num_bigint::{BigInt, BigUint};
use num_traits::One;

fn inv_mod_2_pow(val: &BigInt, h: u64) -> BigInt {
    let mod_val = BigInt::one() << (h as usize);
    let mut x = BigInt::one();
    for _ in 0..(h + 4) {
        x = (&x * (BigInt::from(2) - val * &x)) % &mod_val;
        x = (x + &mod_val) % &mod_val;
    }
    x
}

#[test]
fn test_precision_schedule_strictly_increasing() {
    let valid_schedule = PrecisionSchedule::new(vec![4, 8, 12, 16]);
    assert!(valid_schedule.is_ok());

    let invalid_schedule = PrecisionSchedule::new(vec![4, 8, 8, 16]);
    assert!(invalid_schedule.is_err());
}

#[test]
fn test_ordinary_natural_integer_stabilization() {
    // Ordinary natural integer N = 13
    let n_val = BigInt::from(13);
    let schedule = PrecisionSchedule::new(vec![2, 4, 8, 12, 16]).unwrap();

    let residues: Vec<ProjectiveResidue> = schedule
        .levels
        .iter()
        .map(|&h| ProjectiveResidue::from_bigint(&n_val, h))
        .collect();

    // Representatives:
    // 13 mod 4 = 1
    // 13 mod 16 = 13
    // 13 mod 256 = 13
    // 13 mod 4096 = 13
    assert_eq!(residues[0].least_representative, BigUint::from(1u32));
    assert_eq!(residues[1].least_representative, BigUint::from(13u32));
    assert_eq!(residues[2].least_representative, BigUint::from(13u32));
    assert_eq!(residues[3].least_representative, BigUint::from(13u32));

    // Verify monotonicity R_{n+1} >= R_n
    for i in 0..residues.len() - 1 {
        assert!(residues[i + 1].least_representative >= residues[i].least_representative);
    }

    // Verify compatible pairs and lift blocks
    let lift_01 = verify_compatible_pair(&residues[0], &residues[1]).unwrap();
    assert_eq!(lift_01, LiftBlock {
        from_precision: 2,
        to_precision: 4,
        value: BigUint::from(3u32), // (13 - 1) / 4 = 3
    });

    let lift_12 = verify_compatible_pair(&residues[1], &residues[2]).unwrap();
    assert_eq!(lift_12, LiftBlock {
        from_precision: 4,
        to_precision: 8,
        value: BigUint::from(0u32), // (13 - 13) / 16 = 0 (Zero Lift Tail)
    });

    // Detect constant suffix
    let seq_biguint: Vec<BigUint> = residues.iter().map(|r| r.least_representative.clone()).collect();
    let (stage, val) = detect_constant_suffix(&seq_biguint).unwrap();
    assert_eq!(stage, 1);
    assert_eq!(val, BigUint::from(13u32));

    // Verify certificate
    let cert = StabilizationCertificate {
        stabilization_stage: 1,
        stabilized_value: BigUint::from(13u32),
    };
    assert!(verify_stabilization_certificate(&seq_biguint, &cert));
}

#[test]
fn test_negative_integer_unbounded_representatives() {
    // Negative integer N = -1
    let n_val = BigInt::from(-1);
    let schedule = PrecisionSchedule::new(vec![4, 8, 12, 16]).unwrap();

    let residues: Vec<ProjectiveResidue> = schedule
        .levels
        .iter()
        .map(|&h| ProjectiveResidue::from_bigint(&n_val, h))
        .collect();

    // R_n = 2^H_n - 1 -> Grows unboundedly with modulus
    assert_eq!(residues[0].least_representative, BigUint::from(15u32));  // 2^4 - 1
    assert_eq!(residues[1].least_representative, BigUint::from(255u32)); // 2^8 - 1
    assert_eq!(residues[2].least_representative, BigUint::from(4095u32));// 2^12 - 1

    // Verify compatibility holds
    let lift_01 = verify_compatible_pair(&residues[0], &residues[1]).unwrap();
    assert_eq!(lift_01.value, BigUint::from(15u32)); // Non-zero lift block

    // Constant suffix detection returns None for unbounded growth
    let seq_biguint: Vec<BigUint> = residues.iter().map(|r| r.least_representative.clone()).collect();
    let cert = StabilizationCertificate {
        stabilization_stage: 0,
        stabilized_value: BigUint::from(15u32),
    };
    assert!(!verify_stabilization_certificate(&seq_biguint, &cert));
}

#[test]
fn test_non_natural_2adic_rationals() {
    // Test 1: Pole singularity -1/3 (3 * x = -1)
    let schedule = PrecisionSchedule::new(vec![4, 8, 12, 16]).unwrap();
    let num_pole = BigInt::from(-1);
    let den_pole = BigInt::from(3);

    let residues_pole: Vec<ProjectiveResidue> = schedule
        .levels
        .iter()
        .map(|&h| {
            let mod_val = BigInt::one() << (h as usize);
            let inv = inv_mod_2_pow(&den_pole, h);
            let r = ((&num_pole * &inv) % &mod_val + &mod_val) % &mod_val;
            ProjectiveResidue::from_bigint(&r, h)
        })
        .collect();

    // Representatives for -1/3 mod 2^h grow unboundedly
    assert_ne!(residues_pole[0].least_representative, residues_pole[1].least_representative);
    assert_ne!(residues_pole[1].least_representative, residues_pole[2].least_representative);

    // Test 2: Rational periodic core -26/217
    let num_core = BigInt::from(-26);
    let den_core = BigInt::from(217);

    let residues_core: Vec<ProjectiveResidue> = schedule
        .levels
        .iter()
        .map(|&h| {
            let mod_val = BigInt::one() << (h as usize);
            let inv = inv_mod_2_pow(&den_core, h);
            let r = ((&num_core * &inv) % &mod_val + &mod_val) % &mod_val;
            ProjectiveResidue::from_bigint(&r, h)
        })
        .collect();

    // Representatives for -26/217 also grow unboundedly
    assert_ne!(residues_core[0].least_representative, residues_core[1].least_representative);
    assert_ne!(residues_core[1].least_representative, residues_core[2].least_representative);
}
