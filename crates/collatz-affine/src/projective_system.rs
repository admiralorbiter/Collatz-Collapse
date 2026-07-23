use crate::AffineError;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Verification engine for Phase H.1 Projective Inverse System & Minimal Pointwise Reduction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectiveInverseSystem;

impl ProjectiveInverseSystem {
    /// Computes least non-negative representative R_n = r_n mod 2^{H_n} in [0, 2^{H_n} - 1].
    pub fn least_representative(r_n: &BigInt, h_n: u64) -> BigUint {
        let mod_val = BigInt::one() << (h_n as usize);
        let rem = ((r_n % &mod_val) + &mod_val) % &mod_val;
        rem.to_biguint().unwrap()
    }

    /// Verifies compatibility under projection: R_{n+1} == R_n mod 2^{H_n}.
    pub fn is_compatible_projection(
        r_prev: &BigUint,
        h_prev: u64,
        r_curr: &BigUint,
        h_curr: u64,
    ) -> bool {
        if h_curr < h_prev {
            return false;
        }
        let mod_prev = BigUint::one() << (h_prev as usize);
        (r_curr % &mod_prev) == *r_prev
    }

    /// Computes non-negative lift digit \lambda_{n+1} = (R_{n+1} - R_n) / 2^{H_n}.
    pub fn compute_lift_digit(
        r_prev: &BigUint,
        h_prev: u64,
        r_curr: &BigUint,
    ) -> Result<BigUint, AffineError> {
        if r_curr < r_prev {
            return Err(AffineError::Overflow);
        }
        let diff = r_curr - r_prev;
        let mod_prev = BigUint::one() << (h_prev as usize);
        if !(&diff % &mod_prev).is_zero() {
            return Err(AffineError::Overflow);
        }
        Ok(diff >> (h_prev as usize))
    }

    /// Checks if a sequence of least representatives R_n eventually stabilizes to a fixed integer n_0.
    pub fn verify_eventual_stabilization(r_sequence: &[BigUint]) -> Option<BigUint> {
        if r_sequence.is_empty() {
            return None;
        }
        let last = r_sequence.last().unwrap();
        let tail_start = r_sequence
            .iter()
            .rposition(|r| r != last)
            .map(|i| i + 1)
            .unwrap_or(0);
        if r_sequence.len() - tail_start >= 2 {
            Some(last.clone())
        } else {
            None
        }
    }
}
