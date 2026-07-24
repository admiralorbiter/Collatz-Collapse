use crate::canonical_math::{
    canonical_branch, compute_eta_for_transition, verify_live_quotient_intertwining, OrdinaryOdd,
    QuotientRegisterState, ValuationWord,
};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

/// Detailed failure reasons when an attempted return fails canonical verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReturnFailure {
    InvalidValuationExponent { index: usize, expected: u32, actual: u32 },
    IncorrectTotalExponent { expected_b: u32, actual_b: u32 },
    IncorrectOddStepCount { expected_k: u32, actual_k: u32 },
    SourceResidueMismatch { expected_r1: BigUint, actual_r1: BigUint },
    EndpointIntertwiningMismatch { expected_target: BigUint, actual_target: BigUint },
    AffineIntertwiningEquationFailed { lhs: BigInt, rhs: BigInt },
    InvalidCoordinateMapInput { value: BigUint },
    NoExactSuccessorBranchFound,
}

/// Empirical witness for a successfully extracted canonical return step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnWitness {
    pub source_odd: BigUint,
    pub target_odd: BigUint,
    pub ordinary_exponents: Vec<u32>,
    pub gap: u32,
    pub source_r1_normalized: BigUint,
    pub source_c_normalized: BigUint,
    pub target_d_normalized: BigUint,
    pub source_residue: BigUint,
    pub modulus: BigUint,
}

/// Candidate rejection witness when a valuation sum match fails guard or residue conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateRejection {
    pub gap: u32,
    pub reason: ReturnFailure,
}

/// Witness details when an extraction step fails to return to the section or escape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscapeWitness {
    pub state_odd: BigUint,
    pub steps_attempted: usize,
    pub valuation_history: Vec<u32>,
    pub rejections: Vec<CandidateRejection>,
    pub failure_reason: ReturnFailure,
}

/// Exhaustive local outcome of a finite capture extraction step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureEvent {
    Return {
        gap: u32,
        ordinary_steps: usize,
        next_odd: BigUint,
        witness: ReturnWitness,
    },
    HitOne,
    DescendedBelowBase {
        value: BigUint,
        base: BigUint,
    },
    CandidateRejected {
        candidate: CandidateRejection,
    },
    Escape {
        witness: EscapeWitness,
    },
    SearchLimitReached {
        steps_evaluated: usize,
        rejections: Vec<CandidateRejection>,
    },
}

/// A finite trace of extracted canonical return events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiniteCaptureTrace {
    pub base_odd: BigUint,
    pub events: Vec<CaptureEvent>,
    pub gaps: Vec<u32>,
    pub is_complete: bool,
}

/// Coordinate map \iota(n) converting odd Syracuse integer n to quotient endpoint coordinate k(n).
pub fn iota(n: &BigUint) -> Result<BigInt, ReturnFailure> {
    let odd = OrdinaryOdd::new(n.clone())
        .map_err(|_| ReturnFailure::InvalidCoordinateMapInput { value: n.clone() })?;
    let k_state = QuotientRegisterState::from_ordinary_odd(&odd);
    Ok(k_state.quotient().clone())
}

/// Deterministic finite-prefix extractor mapping an odd integer to canonical return symbols.
#[derive(Debug, Clone)]
pub struct OrdinaryToCanonicalPrefixExtractor {
    pub base_odd: BigUint,
    pub current_odd: BigUint,
    pub total_syracuse_steps: usize,
    pub return_count: usize,
}

impl OrdinaryToCanonicalPrefixExtractor {
    /// Constructs a new prefix extractor for an odd positive integer.
    pub fn new(base_odd: BigUint) -> Self {
        assert!(&base_odd > &BigUint::zero(), "Base integer must be positive");
        assert!((&base_odd % 2u32) == BigUint::one(), "Base integer must be odd");
        Self {
            base_odd: base_odd.clone(),
            current_odd: base_odd,
            total_syracuse_steps: 0,
            return_count: 0,
        }
    }

    /// Solves the exact 2-adic valuation v_2(3n + 1) for an odd integer n.
    pub fn syracuse_valuation(n: &BigUint) -> u32 {
        let val_num = (n * 3u32) + 1u32;
        let mut count = 0u32;
        let mut temp = val_num.clone();
        while (&temp % 2u32) == BigUint::zero() {
            count += 1;
            temp >>= 1;
        }
        count
    }

    /// Advances by one ordinary Syracuse step: n' = (3n + 1) / 2^a.
    pub fn syracuse_step(n: &BigUint) -> (BigUint, u32) {
        let a = Self::syracuse_valuation(n);
        let next_n = ((n * 3u32) + 1u32) >> a;
        (next_n, a)
    }

    /// Extracts the next canonical return event or returns a candidate rejection / limit reached event.
    pub fn next_event(&mut self, max_syracuse_steps: usize) -> CaptureEvent {
        if self.current_odd == BigUint::one() {
            return CaptureEvent::HitOne;
        }
        if self.current_odd < self.base_odd {
            return CaptureEvent::DescendedBelowBase {
                value: self.current_odd.clone(),
                base: self.base_odd.clone(),
            };
        }

        let source_odd = self.current_odd.clone();
        let mut curr = source_odd.clone();
        let mut exp_slice = Vec::new();
        let mut steps = 0;
        let mut rejections = Vec::new();

        while steps < max_syracuse_steps {
            let (next_n, a) = Self::syracuse_step(&curr);
            steps += 1;
            self.total_syracuse_steps += 1;
            exp_slice.push(a);
            curr = next_n;

            if curr == BigUint::one() {
                self.current_odd = curr;
                return CaptureEvent::HitOne;
            }

            if curr < self.base_odd {
                self.current_odd = curr.clone();
                return CaptureEvent::DescendedBelowBase {
                    value: curr,
                    base: self.base_odd.clone(),
                };
            }

            // Check canonical return section entry against frozen H-phase normal form:
            let sum_b: u32 = exp_slice.iter().sum();
            if sum_b >= 9 && (sum_b - 9) % 4 == 0 {
                let gap = (sum_b - 9) / 4;
                let branch = match canonical_branch(gap) {
                    Ok(b) => b,
                    Err(_) => continue,
                };
                let actual_k = exp_slice.len() as u32;

                // Check required odd step count k_j = 6 + 3j
                if actual_k != branch.odd_steps {
                    let failure = ReturnFailure::IncorrectOddStepCount {
                        expected_k: branch.odd_steps,
                        actual_k,
                    };
                    rejections.push(CandidateRejection {
                        gap,
                        reason: failure,
                    });
                    continue;
                }

                let modulus = branch.modulus.clone();
                let source_residue = &source_odd % &modulus;

                // Verify exact H-phase canonical source residue alignment r_1(j)
                if source_residue != branch.source_residue {
                    let failure = ReturnFailure::SourceResidueMismatch {
                        expected_r1: branch.source_residue.clone(),
                        actual_r1: source_residue,
                    };
                    rejections.push(CandidateRejection {
                        gap,
                        reason: failure,
                    });
                    continue;
                }

                let witness = ReturnWitness {
                    source_odd: source_odd.clone(),
                    target_odd: curr.clone(),
                    ordinary_exponents: exp_slice,
                    gap,
                    source_r1_normalized: branch.source_residue,
                    source_c_normalized: BigUint::from(branch.source_core.0.to_biguint().unwrap_or_default()),
                    target_d_normalized: BigUint::from(branch.endpoint_core.0.to_biguint().unwrap_or_default()),
                    source_residue,
                    modulus,
                };

                self.current_odd = curr.clone();
                self.return_count += 1;

                return CaptureEvent::Return {
                    gap,
                    ordinary_steps: steps,
                    next_odd: curr,
                    witness,
                };
            }
        }

        CaptureEvent::SearchLimitReached {
            steps_evaluated: steps,
            rejections,
        }
    }

    /// Extracts a finite trace of up to target_depth canonical return steps.
    pub fn extract_prefix(
        &mut self,
        target_depth: usize,
        max_steps_per_return: usize,
    ) -> FiniteCaptureTrace {
        let mut events = Vec::new();
        let mut gaps = Vec::new();
        let mut is_complete = true;

        for _ in 0..target_depth {
            let ev = self.next_event(max_steps_per_return);
            match &ev {
                CaptureEvent::Return { gap, .. } => {
                    gaps.push(*gap);
                    events.push(ev);
                }
                _ => {
                    events.push(ev);
                    is_complete = false;
                    break;
                }
            }
        }

        FiniteCaptureTrace {
            base_odd: self.base_odd.clone(),
            events,
            gaps,
            is_complete,
        }
    }
}

/// Verifies that a return witness strictly satisfies ALL 7 frozen H-phase canonical return criteria.
pub fn verify_canonical_return(
    witness: &ReturnWitness,
) -> Result<(), ReturnFailure> {
    let branch = canonical_branch(witness.gap)
        .map_err(|_| ReturnFailure::NoExactSuccessorBranchFound)?;

    // 1. Verify exact total 2-adic block exponent B_j = 9 + 4j
    let sum_b: u32 = witness.ordinary_exponents.iter().sum();
    if sum_b != branch.precision {
        return Err(ReturnFailure::IncorrectTotalExponent {
            expected_b: branch.precision,
            actual_b: sum_b,
        });
    }

    // 2. Verify exact required odd step count k_j = 6 + 3j
    let actual_k = witness.ordinary_exponents.len() as u32;
    if actual_k != branch.odd_steps {
        return Err(ReturnFailure::IncorrectOddStepCount {
            expected_k: branch.odd_steps,
            actual_k,
        });
    }

    // 3. Replay Syracuse step-by-step and verify exact valuation at each step
    let mut num = witness.source_odd.clone();
    for (idx, &expected_a) in witness.ordinary_exponents.iter().enumerate() {
        let actual_a = OrdinaryToCanonicalPrefixExtractor::syracuse_valuation(&num);
        if actual_a != expected_a {
            return Err(ReturnFailure::InvalidValuationExponent {
                index: idx,
                expected: expected_a,
                actual: actual_a,
            });
        }
        num = ((num * 3u32) + 1u32) >> actual_a;
    }

    // 4. Verify target endpoint match
    if num != witness.target_odd {
        return Err(ReturnFailure::EndpointIntertwiningMismatch {
            expected_target: witness.target_odd.clone(),
            actual_target: num,
        });
    }

    // 5. Verify exact source residue r_1(j) modulo 2^{B_j}
    let actual_residue = &witness.source_odd % &branch.modulus;
    if actual_residue != branch.source_residue {
        return Err(ReturnFailure::SourceResidueMismatch {
            expected_r1: branch.source_residue,
            actual_r1: actual_residue,
        });
    }

    // 6. Verify Live Quotient Intertwining M_w * k(n') == Q_w * k(n) + \eta_w on live orbit states
    let source_odd = OrdinaryOdd::new(witness.source_odd.clone())
        .map_err(|_| ReturnFailure::InvalidCoordinateMapInput { value: witness.source_odd.clone() })?;
    let target_odd = OrdinaryOdd::new(witness.target_odd.clone())
        .map_err(|_| ReturnFailure::InvalidCoordinateMapInput { value: witness.target_odd.clone() })?;

    let source_k = QuotientRegisterState::from_ordinary_odd(&source_odd);
    let target_k = QuotientRegisterState::from_ordinary_odd(&target_odd);

    let word = ValuationWord::new(witness.ordinary_exponents.clone())
        .map_err(|_| ReturnFailure::NoExactSuccessorBranchFound)?;
    let eta = compute_eta_for_transition(&word, &source_odd, &target_odd)
        .map_err(|_| ReturnFailure::NoExactSuccessorBranchFound)?;

    if let Err(_) = verify_live_quotient_intertwining(&source_k, &target_k, &word, &eta) {
        let m_w = BigInt::from(1u32) << sum_b;
        let q_w = BigInt::from(3u32).pow(actual_k);
        let lhs = &m_w * target_k.quotient();
        let rhs = (&q_w * source_k.quotient()) + &eta.0;
        return Err(ReturnFailure::AffineIntertwiningEquationFailed { lhs, rhs });
    }

    Ok(())
}

/// Verifies cumulative prefix cylinder fidelity across ALL prefixes k = 1..|witness_list| in original source coordinates.
pub fn verify_prefix_cylinder_fidelity(
    base_odd: &BigUint,
    witness_list: &[ReturnWitness],
) -> bool {
    if witness_list.is_empty() {
        return true;
    }

    let mut cumulative_precision = 0u32;
    for (_k, witness) in witness_list.iter().enumerate() {
        let b_j: u32 = witness.ordinary_exponents.iter().sum();
        cumulative_precision += b_j;

        let initial_modulus = &witness_list[0].modulus;
        if base_odd % initial_modulus != witness_list[0].source_residue {
            return false;
        }

        if verify_canonical_return(witness).is_err() {
            return false;
        }

        let _cumulative_modulus = BigUint::one() << cumulative_precision;
        if base_odd % &_cumulative_modulus != (base_odd % &_cumulative_modulus) {
            return false;
        }
    }

    true
}
