use crate::traits::{PrefixSieve, PrefixState, RejectionReason, SieveResult};
use collatz_affine::compute_descent_threshold;
use num_bigint::BigUint;
use num_traits::One;

pub struct MinimalCounterexampleSieve;

impl PrefixSieve for MinimalCounterexampleSieve {
    fn name(&self) -> &'static str {
        "MinimalCounterexampleSieve"
    }

    fn evaluate(&self, state: &PrefixState) -> SieveResult {
        let word = state.affine.valuations.as_slice();
        let mut c_j = BigUint::ZERO;
        let mut partial_sum = 0u64;

        for (j_idx, &a_j) in word.iter().enumerate() {
            c_j = (&c_j * 3u32) + (BigUint::one() << partial_sum);
            partial_sum += a_j as u64;

            let j = j_idx + 1;
            if let Some(bound) = compute_descent_threshold(&c_j, j, partial_sum) {
                // If smallest positive representative r_k > bound, cannot be a minimal counterexample
                if state.affine.starting_residue > bound {
                    return SieveResult::Reject {
                        reason: RejectionReason::ExceedsMinimalCounterexampleBound {
                            step_index: j,
                            bound: bound.to_string(),
                        },
                    };
                }
            }
        }

        SieveResult::Keep
    }
}
