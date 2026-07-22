use crate::traits::{PrefixSieve, PrefixState, RejectionReason, SieveResult};
use num_bigint::BigUint;
use num_traits::One;

/// Search Diagnostic evaluating proximity of residues to negative 2-adic integers (e.g., -1 mod 2^A_k).
/// 
/// Mathematical Safeguard: In N+, proximity to -1 mod 2^A_k corresponds to positive integer 2^{A_k} - 1.
/// Therefore, this sieve acts as a search scoring diagnostic or triggers formal rejection strictly when
/// 2^{A_k} - 1 exceeds the minimal counterexample bound.
pub struct TwoAdicImpostorDiagnostic;

impl PrefixSieve for TwoAdicImpostorDiagnostic {
    fn name(&self) -> &'static str {
        "TwoAdicImpostorDiagnostic"
    }

    fn evaluate(&self, state: &PrefixState) -> SieveResult {
        let modulus = BigUint::one() << state.affine.modulus_exponent;
        let diff = &modulus - &state.affine.starting_residue;

        // Diagnostic flag for beam search scoring: residue is 1 away from 2^A_k (e.g., 2^A_k - 1)
        if diff.is_one() {
            // Check if positive representative 2^A_k - 1 exceeds minimal counterexample threshold
            if let Some(threshold) = state.affine.compute_descent_threshold() {
                if &state.affine.starting_residue > &threshold {
                    return SieveResult::Reject {
                        reason: RejectionReason::TwoAdicImpostor,
                    };
                }
            }
        }

        SieveResult::Keep
    }
}
