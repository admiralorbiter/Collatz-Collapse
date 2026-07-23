use crate::AffineError;

/// Memory-efficient representation of a valuation word storing individual 2-adic valuations a_i as u8.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ValuationWord {
    valuations: Vec<u8>,
}

impl ValuationWord {
    /// Creates a new ValuationWord from a slice of u8 values.
    /// Returns AffineError::ZeroValuation if any valuation is zero.
    pub fn new(vals: Vec<u8>) -> Result<Self, AffineError> {
        for (idx, &v) in vals.iter().enumerate() {
            if v == 0 {
                return Err(AffineError::ZeroValuation(idx));
            }
        }
        Ok(Self { valuations: vals })
    }

    /// Helper constructor taking a slice of u8 values.
    pub fn from_slice(vals: &[u8]) -> Self {
        Self::new(vals.to_vec()).unwrap()
    }

    /// Creates a ValuationWord from u32 values, asserting each fits in u8.
    pub fn from_u32_slice(vals: &[u32]) -> Result<Self, AffineError> {
        let mut bytes = Vec::with_capacity(vals.len());
        for (idx, &v) in vals.iter().enumerate() {
            if v == 0 {
                return Err(AffineError::ZeroValuation(idx));
            }
            if v > 255 {
                return Err(AffineError::Overflow);
            }
            bytes.push(v as u8);
        }
        Ok(Self { valuations: bytes })
    }

    pub fn is_empty(&self) -> bool {
        self.valuations.is_empty()
    }

    pub fn len(&self) -> usize {
        self.valuations.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.valuations
    }

    /// Returns the total 2-adic valuation sum A_k = sum(a_i).
    pub fn total_valuation(&self) -> u64 {
        self.valuations.iter().map(|&a| a as u64).sum()
    }

    /// Pushes a new valuation to the word.
    pub fn push(&mut self, val: u8) -> Result<(), AffineError> {
        if val == 0 {
            return Err(AffineError::ZeroValuation(self.valuations.len()));
        }
        self.valuations.push(val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valuation_word_creation() {
        let word = ValuationWord::new(vec![1, 2, 1, 4]).unwrap();
        assert_eq!(word.len(), 4);
        assert_eq!(word.total_valuation(), 8);
    }

    #[test]
    fn test_zero_valuation_rejected() {
        assert!(ValuationWord::new(vec![1, 0, 2]).is_err());
    }
}
