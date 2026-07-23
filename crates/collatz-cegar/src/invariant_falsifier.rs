use crate::zero_lift_explorer::ZeroLiftWitness;
use serde::{Deserialize, Serialize};

/// Result of testing candidate invariant families against record zero-lift witnesses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantFalsificationResult {
    pub candidate_name: String,
    pub holds_on_records: bool,
    pub smallest_counterexample_run: Option<usize>,
    pub counterexample_reason: String,
}

/// Systematic invariant family tester & falsifier.
pub struct InvariantFalsifier;

impl InvariantFalsifier {
    /// Falsifies naive candidate invariants against a set of record witnesses.
    pub fn falsify_all(witnesses: &[ZeroLiftWitness]) -> Vec<InvariantFalsificationResult> {
        let mut results = Vec::new();

        // 1. Family A Candidate: "Gap sequence j_m is strictly monotonically increasing"
        let mut mono_holds = true;
        let mut mono_reason = "Holds on all evaluated record witnesses".to_string();
        let mut mono_run = None;

        for w in witnesses {
            for window in w.gap_sequence.windows(2) {
                if window[1] <= window[0] {
                    mono_holds = false;
                    mono_run = Some(w.zero_lift_suffix_length);
                    mono_reason = format!(
                        "Falsified on zero-lift suffix length {}: gap transition {} -> {} is non-increasing",
                        w.zero_lift_suffix_length, window[0], window[1]
                    );
                    break;
                }
            }
            if !mono_holds {
                break;
            }
        }

        results.push(InvariantFalsificationResult {
            candidate_name: "Family A: Strictly Monotonic Gap Sequence".to_string(),
            holds_on_records: mono_holds,
            smallest_counterexample_run: mono_run,
            counterexample_reason: mono_reason,
        });

        // 2. Family B Candidate: "Endpoint y_m is strictly bounded by 2^{B_m}"
        let mut bound_holds = true;
        let mut bound_reason = "Holds on all evaluated record witnesses".to_string();
        let mut bound_run = None;

        for w in witnesses {
            for (idx, y) in w.endpoint_trajectory.iter().enumerate() {
                let j = w.gap_sequence.get(idx).copied().unwrap_or(0);
                let b_m = 9 + 4 * j;
                let pow_b = num_bigint::BigUint::from(1u32) << b_m;
                if y >= &pow_b {
                    bound_holds = false;
                    bound_run = Some(w.zero_lift_suffix_length);
                    bound_reason = format!(
                        "Falsified on step {} of zero-lift suffix length {}: endpoint z = {} exceeds 2^{}",
                        idx, w.zero_lift_suffix_length, y, b_m
                    );
                    break;
                }
            }
            if !bound_holds {
                break;
            }
        }

        results.push(InvariantFalsificationResult {
            candidate_name: "Family B: Endpoint Bounded By 2^B_m".to_string(),
            holds_on_records: bound_holds,
            smallest_counterexample_run: bound_run,
            counterexample_reason: bound_reason,
        });

        // 3. Family D Candidate: "Endpoint z mod 27 is invariant under zero-lift transitions"
        let mut mod27_holds = true;
        let mut mod27_reason = "Holds on all evaluated record witnesses".to_string();
        let mut mod27_run = None;

        for w in witnesses {
            if let (Some(&first), Some(&last)) = (w.features_3adic.first(), w.features_3adic.last()) {
                if first != last {
                    mod27_holds = false;
                    mod27_run = Some(w.zero_lift_suffix_length);
                    mod27_reason = format!(
                        "Falsified on zero-lift suffix length {}: z mod 27 shifted from {} to {}",
                        w.zero_lift_suffix_length, first, last
                    );
                    break;
                }
            }
        }

        results.push(InvariantFalsificationResult {
            candidate_name: "Family D: Invariant z mod 27 Signature".to_string(),
            holds_on_records: mod27_holds,
            smallest_counterexample_run: mod27_run,
            counterexample_reason: mod27_reason,
        });

        results
    }
}
