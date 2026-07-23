use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Computes a single ordinary Collatz step for an arbitrary-precision integer.
///
/// T(n) = n / 2 if n is even, or 3n + 1 if n is odd.
pub fn collatz_step(n: &BigUint) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }

    if (n & BigUint::one()).is_zero() {
        n >> 1
    } else {
        (n * 3u32) + 1u32
    }
}

/// Fast u64 implementation of ordinary Collatz step with overflow checking.
#[allow(clippy::manual_is_multiple_of)]
pub fn collatz_step_u64(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    if n % 2 == 0 {
        Some(n / 2)
    } else {
        n.checked_mul(3)?.checked_add(1)
    }
}

/// Computes total stopping time (number of steps to reach 1) for a given starting integer.
/// Returns None if limit is reached before 1.
pub fn stopping_time(n: &BigUint, max_steps: u64) -> Option<u64> {
    if n.is_zero() {
        return None;
    }

    let mut current = n.clone();
    let mut steps = 0u64;

    while !current.is_one() && steps < max_steps {
        current = collatz_step(&current);
        steps += 1;
    }

    if current.is_one() {
        Some(steps)
    } else {
        None
    }
}

/// Generates a prefix trajectory of ordinary Collatz values up to `limit` steps or until 1 is reached.
pub fn trajectory_prefix(n: &BigUint, limit: usize) -> Vec<BigUint> {
    let mut trajectory = Vec::with_capacity(limit.min(100));
    if n.is_zero() {
        return trajectory;
    }

    let mut current = n.clone();
    trajectory.push(current.clone());

    for _ in 0..limit {
        if current.is_one() {
            break;
        }
        current = collatz_step(&current);
        trajectory.push(current.clone());
    }

    trajectory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_trajectory_27() {
        let start = BigUint::from(27u32);
        let prefix = trajectory_prefix(&start, 10);
        let expected: Vec<u32> = vec![27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214];
        let expected_big: Vec<BigUint> = expected.into_iter().map(BigUint::from).collect();
        assert_eq!(prefix, expected_big);
    }

    #[test]
    fn test_stopping_time() {
        assert_eq!(stopping_time(&BigUint::from(1u32), 100), Some(0));
        assert_eq!(stopping_time(&BigUint::from(2u32), 100), Some(1));
        assert_eq!(stopping_time(&BigUint::from(4u32), 100), Some(2));
        assert_eq!(stopping_time(&BigUint::from(16u32), 100), Some(4));
    }
}
