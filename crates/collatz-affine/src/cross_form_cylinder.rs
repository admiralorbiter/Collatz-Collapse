use crate::affine_interaction::AffineInteraction;
use crate::inversion::modular_inverse_3k_mod_2A;
use crate::semantic_cylinders::CanonicalCylinder;
use crate::{AffineError, ValuationWord};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed};

/// Solves cross-form congruences over exact affine arithmetic.
pub fn recover_broad_cylinder(
    interaction: &AffineInteraction,
) -> Result<CanonicalCylinder, AffineError> {
    let q = interaction.q();
    let mod_exp = q.total_valuation();
    let modulus = BigUint::one() << mod_exp;

    let k_q = q.odd_steps();
    let inv_3_kq = modular_inverse_3k_mod_2A(k_q, mod_exp);

    // a_q * n + c_q \equiv 0 (mod 2^{A_q})
    // n \equiv (2^{A_q} - c_q) * 3^{-k_q} (mod 2^{A_q})
    let rem = &modulus - (q.constant() % &modulus);
    let residue = (rem * inv_3_kq) % modulus;

    Ok(CanonicalCylinder::new(residue, mod_exp))
}

/// Solves exact cross-form cylinder recovery modulo 2^{A_q + 1}, preserving the -b_q * c_p parity term.
pub fn recover_exact_cylinder(
    interaction: &AffineInteraction,
) -> Result<CanonicalCylinder, AffineError> {
    let q = interaction.q();
    let mod_exp = q.total_valuation() + 1; // A_q + 1
    let modulus = BigUint::one() << mod_exp;

    let k_q = q.odd_steps();
    let inv_3_kq = modular_inverse_3k_mod_2A(k_q, mod_exp);

    // E(n) = d_p(a_q n + c_q) - b_q c_p \equiv 0 (mod 2^{A_q + 1})
    // a_q n + c_q \equiv d_p^{-1} * b_q c_p (mod 2^{A_q + 1})
    // Note b_q * c_p \equiv 2^{A_q} (mod 2^{A_q + 1}) for odd c_p, and d_p^{-1} * 2^{A_q} \equiv 2^{A_q} (mod 2^{A_q + 1}).
    // Thus a_q n + c_q \equiv 2^{A_q} (mod 2^{A_q + 1}).
    let b_q = BigUint::one() << q.total_valuation();
    let target_val = &b_q % &modulus;

    // a_q n \equiv target_val - c_q (mod 2^{A_q + 1})
    let target_bigint = BigInt::from_biguint(Sign::Plus, target_val);
    let c_q_bigint = q.constant_int();

    let diff = (target_bigint - c_q_bigint) % BigInt::from_biguint(Sign::Plus, modulus.clone());
    let positive_diff = if diff.is_negative() {
        diff + BigInt::from_biguint(Sign::Plus, modulus.clone())
    } else {
        diff
    };

    let diff_biguint = positive_diff.to_biguint().unwrap();
    let residue = (diff_biguint * inv_3_kq) % modulus;

    Ok(CanonicalCylinder::new(residue, mod_exp))
}

/// Solves direct exact cylinder for concatenated sequence p ++ q.
pub fn recover_sequence_cylinder(
    p_word: &ValuationWord,
    q_word: &ValuationWord,
) -> Result<CanonicalCylinder, AffineError> {
    let mut combined = Vec::new();
    combined.extend_from_slice(p_word.as_slice());
    combined.extend_from_slice(q_word.as_slice());

    let concat_word = ValuationWord::new(combined)?;
    let total_val = concat_word.total_valuation();
    let mod_exp = total_val + 1; // A + 1
    let modulus = BigUint::one() << mod_exp;
    let target_b = BigUint::one() << total_val; // 2^A

    let k_tot = concat_word.len();
    let inv_3_ktot = modular_inverse_3k_mod_2A(k_tot, mod_exp);

    let pref = crate::AffinePrefix::from_valuation_word(concat_word)?;

    // a * n \equiv 2^A - c (mod 2^{A+1})
    let target_bigint = BigInt::from_biguint(Sign::Plus, target_b % &modulus);
    let c_bigint = BigInt::from_biguint(Sign::Plus, &pref.constant % &modulus);

    let diff = (target_bigint - c_bigint) % BigInt::from_biguint(Sign::Plus, modulus.clone());
    let positive_diff = if diff.is_negative() {
        diff + BigInt::from_biguint(Sign::Plus, modulus.clone())
    } else {
        diff
    };

    let diff_biguint = positive_diff.to_biguint().unwrap();
    let residue = (diff_biguint * inv_3_ktot) % modulus;

    Ok(CanonicalCylinder::new(residue, mod_exp))
}
