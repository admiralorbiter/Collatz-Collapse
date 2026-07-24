use crate::canonical_math::types::{LiveBlockConstant, OrdinaryOdd, ValuationWord};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Exact-word source cylinder result (\rho_w \pmod{2^{B+1}}).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactWordCylinder {
    pub residue: BigUint,
    pub modulus: BigUint,
}

/// Generic compiled semantic return cylinder result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSemanticReturnCylinder {
    pub word: ValuationWord,
    pub exact_word_residue: BigUint,
    pub exact_word_modulus: BigUint,
    pub destination_residue: u32,
    pub destination_bits: u32,
    pub refined_source_residue: BigUint,
    pub refined_source_modulus: BigUint,
    pub is_compatible: bool,
}

/// Computes the exact word block constant \alpha_w = \sum_{i=1}^k 3^{k-i} 2^{\sum_{j=1}^{i-1} a_j}.
pub fn compute_alpha(word: &ValuationWord) -> BigInt {
    let k = word.k_steps();
    let exponents = word.exponents();
    let mut sum_alpha = BigInt::zero();
    let mut cum_exponent = 0u32;

    for i in 0..k as usize {
        let term_3 = BigInt::from(3u32).pow((k - 1 - i as u32) as u32);
        let term_2 = BigInt::from(1u32) << cum_exponent;
        sum_alpha += term_3 * term_2;

        cum_exponent += exponents[i];
    }

    sum_alpha
}

/// Computes the exact live quotient shift \eta_w(n, n') = (\alpha_w + 3^k r(n) - 2^B r(n')) / 32.
pub fn compute_eta_for_transition(
    word: &ValuationWord,
    source_odd: &OrdinaryOdd,
    target_odd: &OrdinaryOdd,
) -> Result<LiveBlockConstant, String> {
    let k = word.k_steps();
    let b = word.total_exponent_b();
    let alpha = compute_alpha(word);

    let q = BigInt::from(3u32).pow(k);
    let m = BigInt::from(1u32) << b;

    let r_source = BigInt::from(source_odd.section_residue_mod32());
    let r_target = BigInt::from(target_odd.section_residue_mod32());

    let numerator = alpha + (&q * r_source) - (&m * r_target);
    let thirty_two = BigInt::from(32u32);

    let (quot, rem) = (&numerator / &thirty_two, &numerator % &thirty_two);
    if !rem.is_zero() {
        Err(format!(
            "Numerator {} is not divisible by 32 (rem = {}) for word {:?}",
            numerator, rem, word
        ))
    } else {
        Ok(LiveBlockConstant(quot))
    }
}

/// Computes the generic destination pullback cylinder \sigma_{w, r_t} = Q_w^{-1} (2^B r_t - \alpha_w) \pmod{2^{B + q}}.
pub fn compute_word_affine_destination_pullback(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
) -> Result<(BigInt, BigInt), String> {
    let k = word.k_steps();
    let b = word.total_exponent_b();
    let alpha = compute_alpha(word);

    let q_w = BigInt::from(3u32).pow(k);
    let m_w = BigInt::from(1u32) << b;

    let target_modulus_big = BigInt::from(1u32) << q_bits;
    let r_norm = BigInt::from(r_target) % &target_modulus_big;

    let modulus_bits = b.checked_add(q_bits).ok_or_else(|| "Precision overflow".to_string())?;
    let modulus = BigInt::from(1u32) << modulus_bits;

    // Modular inverse Q_w^{-1} \pmod{2^{B + q}}
    let q_inv = mod_inverse_power_of_two(&q_w, modulus_bits)?;

    let rhs = (m_w * r_norm) - alpha;
    let sigma = (q_inv * rhs) % &modulus;
    let positive_sigma = if sigma < BigInt::zero() {
        sigma + &modulus
    } else {
        sigma
    };

    Ok((positive_sigma, modulus))
}

/// Generic exact-word source cylinder compiler: computes \rho_w \pmod{2^{B+1}} via backward pullback.
pub fn compile_exact_word_cylinder(word: &ValuationWord) -> Result<ExactWordCylinder, String> {
    let k = word.k_steps() as usize;
    if k == 0 {
        return Err("Cannot compile exact-word cylinder for empty word".to_string());
    }

    let exponents = word.exponents();
    let total_b = word.total_exponent_b();
    let final_bits = total_b + 1;
    let final_modulus = BigInt::from(1u32) << final_bits;

    // Backward pullback starting from y_k \equiv 1 \pmod 2
    let mut current_y = BigInt::one();
    let mut cum_exponent = 1u32;

    for i in (0..k).rev() {
        let a_i = exponents[i];
        cum_exponent += a_i;
        let mod_i = BigInt::from(1u32) << cum_exponent;

        let inv_3 = mod_inverse_power_of_two(&BigInt::from(3u32), cum_exponent)?;
        let term = ((BigInt::from(1u32) << a_i) * current_y) - BigInt::one();
        current_y = (inv_3 * term) % &mod_i;
        if current_y < BigInt::zero() {
            current_y += &mod_i;
        }
    }

    let res_biguint = current_y.to_biguint().ok_or_else(|| "Negative residue".to_string())?;
    let mod_biguint = final_modulus.to_biguint().ok_or_else(|| "Invalid modulus".to_string())?;

    Ok(ExactWordCylinder {
        residue: res_biguint,
        modulus: mod_biguint,
    })
}

/// Generic semantic return compiler combining exact-word cylinder and destination pullback.
pub fn compile_semantic_return(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
) -> Result<CompiledSemanticReturnCylinder, String> {
    let exact_cyl = compile_exact_word_cylinder(word)?;
    let (pullback_sigma, pullback_mod) = compute_word_affine_destination_pullback(word, r_target, q_bits)?;

    let refined_res_biguint = pullback_sigma.to_biguint().ok_or_else(|| "Invalid pullback".to_string())?;
    let refined_mod_biguint = pullback_mod.to_biguint().ok_or_else(|| "Invalid pullback mod".to_string())?;

    // Compatibility check: refined_residue \equiv exact_residue \pmod{exact_modulus}
    let is_compatible = (&refined_res_biguint % &exact_cyl.modulus) == exact_cyl.residue;

    Ok(CompiledSemanticReturnCylinder {
        word: word.clone(),
        exact_word_residue: exact_cyl.residue,
        exact_word_modulus: exact_cyl.modulus,
        destination_residue: r_target,
        destination_bits: q_bits,
        refined_source_residue: refined_res_biguint,
        refined_source_modulus: refined_mod_biguint,
        is_compatible,
    })
}

/// Computes modular inverse of odd integer m modulo 2^e using Newton's method.
pub fn mod_inverse_power_of_two(m: &BigInt, e: u32) -> Result<BigInt, String> {
    if (m % 2u32) == BigInt::zero() {
        return Err("Cannot invert even integer modulo power of two".to_string());
    }
    if e == 0 {
        return Ok(BigInt::zero());
    }

    let mut inv = BigInt::one();
    let two = BigInt::from(2u32);
    let modulus = BigInt::one() << e;

    for _ in 0..e {
        inv = (&inv * (&two - (m * &inv))) % &modulus;
        if inv < BigInt::zero() {
            inv += &modulus;
        }
    }
    Ok(inv)
}

/// Evaluates exact composition of affine cocycles: c_{uv} = Q_v c_u + M_u c_v.
pub fn compose_cocycles(
    c_u: &BigInt,
    c_v: &BigInt,
    q_v: &BigInt,
    m_u: &BigInt,
) -> BigInt {
    (q_v * c_u) + (m_u * c_v)
}
