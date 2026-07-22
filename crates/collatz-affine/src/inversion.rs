use crate::AffineError;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Base seed for 3^-1 mod 2^64: 0xAAAAAAAAAAAAAAAA_u64 * 3 + 1 = ... 
/// 3 * 0xAAAA_AAAA_AAAA_AAAB = 0x2_0000_0000_0000_0001 = 1 mod 2^64
pub const INVERSE_3_MOD_2_64: u64 = 0xAAAAAAAAAAAAAAAA_u64 + 1; // 0xAAAAAAAAAAAAAAAA + 1 = 0xAAAAAAAAAAAAAAAA + 1? Wait:
// Let's verify: 3 * 0xAAAAAAAA_AAAA_AAAB = 3 * 12297829382473034411 = 36893488147419103233 = 2^64 * 2 + 1. Correct!
pub const INVERSE_3_MOD_2_64_VAL: u64 = 0xAAAAAAAAAAAAAAAA_u64 + 1;

/// Computes (3^-1) mod 2^A_k using Hensel Lifting (quadratic Newton iteration).
/// x_{i+1} = x_i * (2 - 3 * x_i) mod 2^{2^i}
pub fn hensel_inverse_3_pow(a_k: u64) -> BigUint {
    if a_k == 0 {
        return BigUint::one();
    }

    // Base seed for up to 64 bits
    let seed = 0xAAAAAAAAAAAAAAAA_u64.wrapping_add(1); // 12297829382473034411
    if a_k <= 64 {
        let mask = if a_k == 64 { u64::MAX } else { (1u64 << a_k) - 1 };
        return BigUint::from(seed & mask);
    }

    // Double bit precision starting from 64 bits up to a_k
    let mut x = BigUint::from(seed);
    let mut current_bits = 64u64;

    let two = BigUint::from(2u32);
    let three = BigUint::from(3u32);

    while current_bits < a_k {
        let next_bits = (current_bits * 2).min(a_k);
        let modulus = BigUint::one() << next_bits;
        
        // Newton step: x_next = (x * (2 - 3 * x)) mod 2^next_bits
        let term = &three * &x;
        // In unsigned mod 2^next_bits: (2 - term) mod 2^next_bits = (2 + modulus - (term % modulus)) % modulus
        let term_mod = &term % &modulus;
        let diff = if term_mod <= two {
            &two - &term_mod
        } else {
            (&modulus + &two) - &term_mod
        };

        x = (&x * &diff) % &modulus;
        current_bits = next_bits;
    }

    x
}

/// Computes (3^k)^-1 mod 2^A_k = ((3^-1)^k) mod 2^A_k via binary exponentiation.
#[allow(non_snake_case)]
pub fn modular_inverse_3k_mod_2A(k: usize, a_k: u64) -> BigUint {
    if a_k == 0 {
        return BigUint::zero();
    }

    let inv_3 = hensel_inverse_3_pow(a_k);
    let modulus = BigUint::one() << a_k;

    // Modular exponentiation: (inv_3)^k mod 2^A_k
    inv_3.modpow(&BigUint::from(k), &modulus)
}

/// Solves the closed-form starting residue n_0 mod 2^A_k for the broad class (terminal valuation >= a_{k-1}):
/// n_0 = (-c_k * (3^k)^-1) mod 2^A_k
pub fn solve_starting_residue_broad(c_k: &BigUint, k: usize, a_k: u64) -> Result<BigUint, AffineError> {
    if a_k == 0 {
        return Ok(BigUint::zero());
    }

    let inv_3k = modular_inverse_3k_mod_2A(k, a_k);
    let modulus = BigUint::one() << a_k;
    let mask = &modulus - 1u32;

    // Direct unsigned calculation of -c_k * inv_3k mod 2^A_k
    let prod = (c_k * inv_3k) & &mask;

    if prod.is_zero() {
        Ok(BigUint::zero())
    } else {
        Ok(&modulus - prod)
    }
}

/// Solves the closed-form starting residue n_0 mod 2^{A_k + 1} for the exact valuation cylinder (terminal valuation == a_{k-1}):
/// n_0 = ((2^A_k - c_k) * (3^k)^-1) mod 2^{A_k + 1}
pub fn solve_starting_residue_exact(c_k: &BigUint, k: usize, a_k: u64) -> Result<BigUint, AffineError> {
    if a_k == 0 {
        return Ok(BigUint::zero());
    }

    let mod_exponent = a_k + 1;
    let inv_3k = modular_inverse_3k_mod_2A(k, mod_exponent);
    let modulus = BigUint::one() << mod_exponent;
    let pow2_a = BigUint::one() << a_k;

    // target = (2^A_k - c_k) mod 2^{A_k + 1}
    let target = if &pow2_a >= c_k {
        (&pow2_a - c_k) % &modulus
    } else {
        let diff = c_k - &pow2_a;
        let rem = diff % &modulus;
        if rem.is_zero() {
            BigUint::zero()
        } else {
            &modulus - rem
        }
    };

    Ok((target * inv_3k) % &modulus)
}

/// Solves the starting residue mod 2^A_k (broad class default).
pub fn solve_starting_residue(c_k: &BigUint, k: usize, a_k: u64) -> Result<BigUint, AffineError> {
    solve_starting_residue_broad(c_k, k, a_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hensel_inverse_3() {
        // 3 * (3^-1 mod 2^64) = 1 mod 2^64
        let inv64 = hensel_inverse_3_pow(64);
        let mod64 = BigUint::one() << 64;
        assert_eq!((BigUint::from(3u32) * inv64) % mod64, BigUint::one());

        // Test 128 bits
        let inv128 = hensel_inverse_3_pow(128);
        let mod128 = BigUint::one() << 128;
        assert_eq!((BigUint::from(3u32) * inv128) % mod128, BigUint::one());
    }

    #[test]
    fn test_modular_inverse_3k() {
        // For k=3, 3^3 = 27. Test mod 2^8 = 256. 27 * 19 = 513 = 2 * 256 + 1. Inverse is 19.
        let inv = modular_inverse_3k_mod_2A(3, 8);
        assert_eq!((BigUint::from(27u32) * inv) % 256u32, BigUint::one());
    }

    #[test]
    fn test_solve_starting_residue_known() {
        // For valuation word (1, 1), k=2, A_k=2.
        // c_1 = 1, c_2 = 3(1) + 2^1 = 5.
        // 3^2 n_0 + 5 = 0 mod 4 => 9 n_0 + 5 = 0 mod 4 => n_0 + 1 = 0 mod 4 => n_0 = 3 mod 4.
        let c_2 = BigUint::from(5u32);
        let res = solve_starting_residue(&c_2, 2, 2).unwrap();
        assert_eq!(res, BigUint::from(3u32));
    }

    #[test]
    fn test_solve_starting_residue_broad_and_exact_1_1_2_1_3() {
        // Word (1, 1, 2, 1, 3), k=5, A_k=8, c_5=251.
        let c_5 = BigUint::from(251u32);
        let broad_res = solve_starting_residue_broad(&c_5, 5, 8).unwrap();
        let exact_res = solve_starting_residue_exact(&c_5, 5, 8).unwrap();

        assert_eq!(broad_res, BigUint::from(39u32));
        assert_eq!(exact_res, BigUint::from(295u32));
    }
}
