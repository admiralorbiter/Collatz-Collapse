use crate::traits::{PrefixSieve, PrefixState, RejectionReason, SieveResult};

pub struct DescentSieve;

impl PrefixSieve for DescentSieve {
    fn name(&self) -> &'static str {
        "DescentSieve"
    }

    fn evaluate(&self, state: &PrefixState) -> SieveResult {
        if state.affine.is_multiplicative_contracting() {
            if let Some(threshold) = state.affine.compute_descent_threshold() {
                if state.affine.starting_residue >= threshold {
                    return SieveResult::Reject {
                        reason: RejectionReason::DescentCertified,
                    };
                }
            }
        }
        SieveResult::Keep
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collatz_affine::{AffinePrefix, ValuationWord};
    use smallvec::smallvec;

    #[test]
    fn test_descent_sieve_rejection() {
        // Valuation word (1, 1, 2, 1, 3) -> k=5, A_k=8. 2^8 = 256 > 243 = 3^5.
        // starting_residue = 39 >= B = 17. Should be rejected as DescentCertified.
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let affine = AffinePrefix::from_valuation_word(word).unwrap();
        let state = PrefixState {
            valuations: smallvec![1, 1, 2, 1, 3],
            affine,
            growth_debt: -0.1,
        };

        let sieve = DescentSieve;
        assert_eq!(sieve.evaluate(&state), SieveResult::Reject { reason: RejectionReason::DescentCertified });
    }
}
