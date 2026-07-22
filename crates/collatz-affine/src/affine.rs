use crate::{solve_starting_residue_broad, solve_starting_residue_exact, AffineError, ValuationWord};
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Explicit valuation semantics for symbolic residue classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ValuationSemantics {
    /// Terminal valuation >= a_{k-1}. Modulus 2^A_k, 2-adic measure 2^{-(A_k-1)}.
    #[default]
    TerminalAtLeast,
    /// Complete exact valuation word. Modulus 2^{A_k + 1}, 2-adic measure 2^{-A_k}.
    ExactWord,
}

/// Encapsulates the exact affine transformation n_k = (3^k * n_0 + c_k) / 2^{A_k} for a valuation prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffinePrefix {
    pub valuations: ValuationWord,
    pub odd_steps: usize,
    pub total_twos: u64,
    pub constant: BigUint,
    pub starting_residue: BigUint,
    pub modulus_exponent: u64,
}

impl AffinePrefix {
    /// Constructs an AffinePrefix from a ValuationWord, computing c_k and closed-form starting residue.
    pub fn from_valuation_word(word: ValuationWord) -> Result<Self, AffineError> {
        if word.is_empty() {
            return Err(AffineError::EmptyValuationWord);
        }

        let k = word.len();
        let a_k = word.total_valuation();
        let mut c_k = BigUint::zero();
        let mut partial_sum = 0u64;

        // Recurrence: c_0 = 0, c_{i+1} = 3 * c_i + 2^{A_i}
        for &a_i in word.as_slice() {
            c_k = (&c_k * 3u32) + (BigUint::one() << partial_sum);
            partial_sum += a_i as u64;
        }

        let starting_residue = solve_starting_residue_broad(&c_k, k, a_k)?;

        Ok(Self {
            valuations: word,
            odd_steps: k,
            total_twos: a_k,
            constant: c_k,
            starting_residue,
            modulus_exponent: a_k,
        })
    }

    /// Solves broad residue class starting_residue mod 2^A_k (terminal valuation >= a_{k-1}).
    pub fn starting_residue_broad(&self) -> Result<BigUint, AffineError> {
        solve_starting_residue_broad(&self.constant, self.odd_steps, self.total_twos)
    }

    /// Solves exact cylinder residue class starting_residue mod 2^{A_k + 1} (terminal valuation == a_{k-1}).
    pub fn starting_residue_exact(&self) -> Result<BigUint, AffineError> {
        solve_starting_residue_exact(&self.constant, self.odd_steps, self.total_twos)
    }

    /// Solves starting residue for the specified ValuationSemantics.
    pub fn starting_residue_for_semantics(&self, semantics: ValuationSemantics) -> Result<(BigUint, u64), AffineError> {
        match semantics {
            ValuationSemantics::TerminalAtLeast => {
                let res = self.starting_residue_broad()?;
                Ok((res, self.total_twos))
            }
            ValuationSemantics::ExactWord => {
                let res = self.starting_residue_exact()?;
                Ok((res, self.total_twos + 1))
            }
        }
    }

    /// Evaluates the affine transformation n_k = (3^k * n_0 + c_k) / 2^{A_k} for a concrete starting value n_0.
    pub fn apply(&self, n_0: &BigUint) -> Result<BigUint, AffineError> {
        let pow3_k = BigUint::from(3u32).pow(self.odd_steps as u32);
        let numerator = (pow3_k * n_0) + &self.constant;
        let denominator = BigUint::one() << self.total_twos;

        if (&numerator % &denominator).is_zero() {
            Ok(numerator >> self.total_twos)
        } else {
            Err(AffineError::Overflow)
        }
    }

    /// Checks if multiplicative contraction holds: 2^{A_k} > 3^k
    pub fn is_multiplicative_contracting(&self) -> bool {
        let pow3_k = BigUint::from(3u32).pow(self.odd_steps as u32);
        let pow2_a = BigUint::one() << self.total_twos;
        pow2_a > pow3_k
    }

    /// Computes the exact integer descent threshold B = floor(c_k / (2^{A_k} - 3^k)) + 1.
    /// Returns None if 2^{A_k} <= 3^k.
    pub fn compute_descent_threshold(&self) -> Option<BigUint> {
        compute_descent_threshold(&self.constant, self.odd_steps, self.total_twos)
    }

    /// Computes the signed representative of the starting residue in [-2^{A_k-1}, 2^{A_k-1}].
    pub fn signed_representative(&self) -> num_bigint::BigInt {
        use num_bigint::BigInt;
        let res_bi = BigInt::from(self.starting_residue.clone());
        let mod_bi = BigInt::from(BigUint::one() << self.total_twos);
        let half_mod = &mod_bi >> 1;

        if res_bi > half_mod {
            res_bi - mod_bi
        } else {
            res_bi
        }
    }

    /// Computes the number of matching trailing 2-adic bits between starting_residue and -1/3 mod 2^{A_k}.
    /// Distance is 2^{-m} where m is the returned bit count.
    pub fn distance_to_minus_one_third(&self) -> u32 {
        if self.total_twos == 0 {
            return 0;
        }

        let inv_3 = crate::inversion::hensel_inverse_3_pow(self.total_twos);
        let modulus = BigUint::one() << self.total_twos;
        let target = &modulus - inv_3;

        let diff = &self.starting_residue ^ &target;
        let mask = &modulus - 1u32;
        let diff_masked = diff & mask;

        if diff_masked.is_zero() {
            self.total_twos as u32
        } else {
            let bytes = diff_masked.to_bytes_le();
            let mut zeros = 0u32;
            for &b in &bytes {
                if b == 0 {
                    zeros += 8;
                } else {
                    zeros += b.trailing_zeros();
                    break;
                }
            }
            zeros.min(self.total_twos as u32)
        }
    }
}

/// Helper function to compute threshold B = floor(c_k / (2^{A_k} - 3^k)) + 1.
pub fn compute_descent_threshold(c_k: &BigUint, k: usize, a_k: u64) -> Option<BigUint> {
    let pow3_k = BigUint::from(3u32).pow(k as u32);
    let pow2_a = BigUint::one() << a_k;

    if pow2_a <= pow3_k {
        return None;
    }

    let diff = pow2_a - pow3_k;
    let b = (c_k / diff) + 1u32;
    Some(b)
}


/// Diagnostic metadata measuring affine growth vs paradoxical sequence discrepancy.
#[derive(Debug, Clone, PartialEq)]
pub struct AffineDiagnostics {
    pub growth_debt: f64,
    pub is_contracting: bool,
    pub descent_threshold: Option<BigUint>,
    pub pole_distance_bits: u32,
    pub signed_representative: num_bigint::BigInt,
}

impl AffineDiagnostics {
    pub fn from_prefix(prefix: &AffinePrefix) -> Self {
        let k_float = prefix.odd_steps as f64;
        let a_float = prefix.total_twos as f64;
        let growth_debt = k_float * 3.0f64.log2() - a_float;

        Self {
            growth_debt,
            is_contracting: prefix.is_multiplicative_contracting(),
            descent_threshold: prefix.compute_descent_threshold(),
            pole_distance_bits: prefix.distance_to_minus_one_third(),
            signed_representative: prefix.signed_representative(),
        }
    }
}


/// u128 Fast-Path Safety Guard:
/// Promotes to BigUint if k > 80 or total_valuation > 126.
/// Computes c_k using checked operations on u128.
pub fn compute_affine_constant_u128(word: &ValuationWord) -> Option<u128> {
    let k = word.len();
    let total_a = word.total_valuation();
    if k > 80 || total_a > 126 {
        return None;
    }

    let mut c_k = 0u128;
    let mut partial_sum = 0u32;
    for &a_i in word.as_slice() {
        let term = 1u128.checked_shl(partial_sum)?;
        c_k = c_k.checked_mul(3)?.checked_add(term)?;
        partial_sum = partial_sum.checked_add(a_i as u32)?;
    }

    Some(c_k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_affine_constant_u128_safe() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let c_k_u128 = compute_affine_constant_u128(&word);
        assert_eq!(c_k_u128, Some(251u128));
    }

    #[test]
    fn test_compute_affine_constant_u128_overflow_promotes() {
        // Valuation requiring total_valuation > 126 promotes to BigUint (returns None for u128 fast path)
        let vals = vec![10u8; 13]; // 130 twos total
        let word = ValuationWord::new(vals).unwrap();
        assert_eq!(compute_affine_constant_u128(&word), None);
    }


    #[test]
    fn test_signed_representative_and_pole_distance() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        let diagnostics = AffineDiagnostics::from_prefix(&prefix);

        // starting_residue is 39 mod 256
        assert_eq!(prefix.starting_residue, BigUint::from(39u32));
        assert_eq!(diagnostics.signed_representative, num_bigint::BigInt::from(39i32));
        assert!(diagnostics.pole_distance_bits <= prefix.total_twos as u32);
    }

    #[test]
    fn test_affine_prefix_creation() {

        // Valuation word (1, 1), k=2, A_k=2.
        // c_1 = 3(0) + 2^0 = 1. c_2 = 3(1) + 2^1 = 5.
        // Residue mod 4 is 3.
        let word = ValuationWord::new(vec![1, 1]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        assert_eq!(prefix.constant, BigUint::from(5u32));
        assert_eq!(prefix.starting_residue, BigUint::from(3u32));
        assert_eq!(prefix.total_twos, 2);
    }

    #[test]
    fn test_apply_affine() {
        let word = ValuationWord::new(vec![1, 4]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        assert_eq!(prefix.apply(&BigUint::from(3u32)).unwrap(), BigUint::from(1u32));
    }
}

