use crate::CoreError;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Represents a single odd-only step transition: from odd `from` to next odd `to`, with 2-adic valuation `valuation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OddStep<N> {
    pub from: N,
    pub to: N,
    pub valuation: u32,
}

/// Computes a single accelerated odd-only step S(n) for BigUint.
///
/// Precondition: `n` MUST be an odd positive integer (n & 1 == 1).
/// Returns CoreError::EvenInput if `n` is even.
pub fn odd_step(n: &BigUint) -> Result<OddStep<BigUint>, CoreError> {
    if n.is_zero() || (n & BigUint::one()).is_zero() {
        return Err(CoreError::EvenInput(n.to_string()));
    }

    let temp = (n * 3u32) + 1u32;
    let valuation = temp.trailing_zeros().unwrap_or(0) as u32;

    if valuation == 0 {
        return Err(CoreError::Overflow);
    }

    let next_odd = temp >> valuation;

    Ok(OddStep {
        from: n.clone(),
        to: next_odd,
        valuation,
    })
}

/// Fast Tier 0 u128 implementation of accelerated odd-only step.
///
/// Precondition: `n` MUST be an odd positive integer (n & 1 == 1).
#[allow(clippy::manual_is_multiple_of)]
pub fn odd_step_u128(n: u128) -> Result<OddStep<u128>, CoreError> {
    if n == 0 || n % 2 == 0 {
        return Err(CoreError::EvenInput(n.to_string()));
    }

    let temp = n
        .checked_mul(3)
        .and_then(|v| v.checked_add(1))
        .ok_or(CoreError::Overflow)?;

    let valuation = temp.trailing_zeros();
    if valuation == 0 {
        return Err(CoreError::Overflow);
    }

    let next_odd = temp >> valuation;

    Ok(OddStep {
        from: n,
        to: next_odd,
        valuation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_step_validation() {
        let even = BigUint::from(28u32);
        assert!(odd_step(&even).is_err());
        assert!(odd_step_u128(28).is_err());
    }

    #[test]
    fn test_odd_step_known_values() {
        // n = 27 -> 3(27)+1 = 82 = 2 * 41 -> valuation = 1, next = 41
        let step = odd_step(&BigUint::from(27u32)).unwrap();
        assert_eq!(step.from, BigUint::from(27u32));
        assert_eq!(step.to, BigUint::from(41u32));
        assert_eq!(step.valuation, 1);

        // n = 3 -> 3(3)+1 = 10 = 2 * 5 -> valuation = 1, next = 5
        let step_3 = odd_step(&BigUint::from(3u32)).unwrap();
        assert_eq!(step_3.to, BigUint::from(5u32));
        assert_eq!(step_3.valuation, 1);

        // n = 5 -> 3(5)+1 = 16 = 2^4 * 1 -> valuation = 4, next = 1
        let step_5 = odd_step(&BigUint::from(5u32)).unwrap();
        assert_eq!(step_5.to, BigUint::from(1u32));
        assert_eq!(step_5.valuation, 4);
    }
}
