use num_bigint::BigUint;
use std::fmt;

/// Relational Abstract State representing congruence class (r mod 2^m)
/// coupled with interval bounds [N_min, N_max] and peak watermark tracking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationalState {
    pub residue: u64,
    pub modulus_exponent: u64,
    pub n_min: Option<BigUint>,
    pub n_max: Option<BigUint>,
    pub peak_watermark: Option<BigUint>,
}

impl RelationalState {
    pub fn new_congruence(residue: u64, modulus_exponent: u64) -> Self {
        Self {
            residue,
            modulus_exponent,
            n_min: Some(BigUint::from(1u32)),
            n_max: None,
            peak_watermark: None,
        }
    }

    /// Staged Widening (∇): Widens interval bounds while holding the congruence modulus fixed.
    /// Prevents loss of parity/modular structure to Top (⊤).
    pub fn widen(&self, other: &Self) -> Self {
        assert_eq!(self.modulus_exponent, other.modulus_exponent);
        assert_eq!(self.residue, other.residue);

        let new_n_min = match (&self.n_min, &other.n_min) {
            (Some(a), Some(b)) => Some(a.min(b).clone()),
            _ => None,
        };

        // Widening upper bound to infinity if increasing
        let new_n_max = match (&self.n_max, &other.n_max) {
            (Some(a), Some(b)) if b <= a => Some(a.clone()),
            _ => None, // Widened to Top (∞)
        };

        let new_watermark = match (&self.peak_watermark, &other.peak_watermark) {
            (Some(a), Some(b)) => Some(a.max(b).clone()),
            (Some(a), None) | (None, Some(a)) => Some(a.clone()),
            (None, None) => None,
        };

        Self {
            residue: self.residue,
            modulus_exponent: self.modulus_exponent,
            n_min: new_n_min,
            n_max: new_n_max,
            peak_watermark: new_watermark,
        }
    }
}

impl fmt::Display for RelationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Res({} mod 2^{})", self.residue, self.modulus_exponent)
    }
}

/// Abstract transition edge between relational states labeled with valuation step.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractEdge {
    pub from: RelationalState,
    pub to: RelationalState,
    pub valuation: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staged_widening_preserves_congruence() {
        let s1 = RelationalState::new_congruence(1, 2);
        let mut s2 = RelationalState::new_congruence(1, 2);
        s2.n_max = Some(BigUint::from(100u32));

        let widened = s1.widen(&s2);
        assert_eq!(widened.residue, 1);
        assert_eq!(widened.modulus_exponent, 2);
        assert_eq!(widened.n_max, None); // Widened upper bound to infinity
    }
}
