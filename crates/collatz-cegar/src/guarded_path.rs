use collatz_affine::{AffinePrefix, CanonicalCylinder, ExecutionSequence, ValuationWord};
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateId {
    Q1, // 7 mod 32
    Q2, // 43 mod 64
    Q3, // 11 mod 32
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateMembership {
    pub state_id: StateId,
    pub set: CanonicalCylinder,
}

impl StateMembership {
    pub fn q1() -> Self {
        Self {
            state_id: StateId::Q1,
            set: CanonicalCylinder::new(BigUint::from(7u32), 5),
        }
    }

    pub fn contains(&self, n: &BigUint) -> bool {
        self.set.contains(n)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BasedReturnCylinder {
    pub base_state: StateId,
    pub word: ValuationWord,
    pub source: CanonicalCylinder,
    pub target: StateMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuardCheckpoint {
    pub step_index: usize,
    pub expected_state: StateMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuardedPathCylinder {
    pub sequence: ExecutionSequence<ValuationWord>,
    pub source: CanonicalCylinder,
    pub checkpoints: Vec<GuardCheckpoint>,
    pub final_state: StateMembership,
}

impl GuardedPathCylinder {
    /// Constructs a path-first complete guarded path cylinder for a macro sequence on a base state.
    /// Composes the path, intersects intermediate guards, verifies images, and classifies closed walk.
    /// Universal determinism requires modulus exponent M >= total_twos + base_modulus_exponent.
    pub fn compute(
        sequence: ExecutionSequence<ValuationWord>,
        base_state: StateMembership,
    ) -> Result<Self, String> {
        if sequence.is_empty() {
            return Err("Execution sequence is empty".to_string());
        }

        let combined_word = sequence.flatten_valuation_word();
        let composite_prefix =
            AffinePrefix::from_valuation_word(combined_word).map_err(|e| e.to_string())?;

        let total_twos = composite_prefix.total_twos;
        // Universal target determinism requires M >= A + base_modulus_exponent
        let min_exp = total_twos + base_state.set.modulus_exponent;

        // Search starting from min_exp to guarantee universal cylinder determinism
        for exp in min_exp..=(min_exp + 6) {
            if exp > 24 {
                break;
            }
            let num_candidates = 1u64 << exp;
            let step_prefixes: Vec<AffinePrefix> = sequence
                .steps()
                .iter()
                .map(|w| AffinePrefix::from_valuation_word(w.clone()).unwrap())
                .collect();

            let mut valid_residues = Vec::new();
            for r_val in 0..num_candidates {
                let r = BigUint::from(r_val);
                if !base_state.contains(&r) {
                    continue;
                }

                let mut curr = r.clone();
                let mut path_valid = true;

                for step_pref in &step_prefixes {
                    match step_pref.apply(&curr) {
                        Ok(next_val) => {
                            curr = next_val;
                            if !base_state.contains(&curr) {
                                path_valid = false;
                                break;
                            }
                        }
                        Err(_) => {
                            path_valid = false;
                            break;
                        }
                    }
                }

                if path_valid {
                    valid_residues.push(r);
                }
            }

            if valid_residues.len() == 1 {
                let res = valid_residues[0].clone();
                let checkpoints = (0..sequence.len())
                    .map(|idx| GuardCheckpoint {
                        step_index: idx,
                        expected_state: base_state.clone(),
                    })
                    .collect();

                return Ok(Self {
                    sequence,
                    source: CanonicalCylinder::new(res, exp),
                    checkpoints,
                    final_state: base_state,
                });
            }
        }

        Err("Failed to find unique guarded path cylinder".to_string())
    }
}
