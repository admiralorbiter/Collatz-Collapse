use crate::CoreError;
use num_bigint::BigUint;
use num_traits::One;

/// Unified abstraction trait for integer backends in Collatz arithmetic.
pub trait CollatzInt: Sized + Clone + Eq + Ord + std::fmt::Display {
    fn is_odd(&self) -> bool;
    fn is_even(&self) -> bool {
        !self.is_odd()
    }
    fn is_unit_one(&self) -> bool;
    fn is_zero_val(&self) -> bool;

    fn v2_valuation_of_3n_plus_1(&self) -> Result<(u32, Self), CoreError>;
}

impl CollatzInt for u128 {
    #[allow(clippy::manual_is_multiple_of)]
    fn is_odd(&self) -> bool {
        self % 2 != 0
    }
    fn is_unit_one(&self) -> bool {
        *self == 1
    }
    fn is_zero_val(&self) -> bool {
        *self == 0
    }

    fn v2_valuation_of_3n_plus_1(&self) -> Result<(u32, Self), CoreError> {
        if self.is_zero_val() || self.is_even() {
            return Err(CoreError::EvenInput(self.to_string()));
        }

        let temp = self
            .checked_mul(3)
            .and_then(|v| v.checked_add(1))
            .ok_or(CoreError::Overflow)?;
        let val = temp.trailing_zeros();
        Ok((val, temp >> val))
    }
}

impl CollatzInt for BigUint {
    fn is_odd(&self) -> bool {
        !num_traits::Zero::is_zero(&(self & BigUint::one()))
    }
    fn is_unit_one(&self) -> bool {
        num_traits::One::is_one(self)
    }
    fn is_zero_val(&self) -> bool {
        num_traits::Zero::is_zero(self)
    }

    fn v2_valuation_of_3n_plus_1(&self) -> Result<(u32, Self), CoreError> {
        if self.is_zero_val() || self.is_even() {
            return Err(CoreError::EvenInput(self.to_string()));
        }

        let temp = (self * 3u32) + 1u32;
        let val = temp.trailing_zeros().unwrap_or(0) as u32;
        let next = &temp >> val;
        Ok((val, next))
    }
}
