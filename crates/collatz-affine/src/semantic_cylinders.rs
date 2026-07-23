use crate::affine::{AffinePrefix, ValuationSemantics};
use crate::valuation::ValuationWord;
use crate::AffineError;
use num_bigint::BigUint;
use num_traits::One;

/// Canonical residue class cylinder n = r mod 2^m.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalCylinder {
    pub residue: BigUint,
    pub modulus_exponent: u64,
}

impl CanonicalCylinder {
    pub fn new(residue: BigUint, modulus_exponent: u64) -> Self {
        let modulus = BigUint::one() << modulus_exponent;
        let normalized = &residue % &modulus;
        Self {
            residue: normalized,
            modulus_exponent,
        }
    }

    pub fn modulus(&self) -> BigUint {
        BigUint::one() << self.modulus_exponent
    }

    pub fn contains(&self, n: &BigUint) -> bool {
        let mod_val = self.modulus();
        (n % &mod_val) == self.residue
    }
}

/// Represents the exact valuation-word source cylinder.
/// Proves that any starting value n in `source` forces the exact execution of `valuation_word`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactWordCylinder {
    pub valuation_word: ValuationWord,
    pub source: CanonicalCylinder,
}

impl ExactWordCylinder {
    pub fn from_valuation_word(word: ValuationWord) -> Result<Self, AffineError> {
        let prefix = AffinePrefix::from_valuation_word(word.clone())?;
        let (residue, mod_exp) =
            prefix.starting_residue_for_semantics(ValuationSemantics::ExactWord)?;
        Ok(Self {
            valuation_word: word,
            source: CanonicalCylinder::new(residue, mod_exp),
        })
    }
}
