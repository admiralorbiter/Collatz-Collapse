use crate::accelerated_branch_params::AcceleratedBranchParams;
use crate::spine_quotient_oracle::OddRational2Adic;
use num_bigint::{BigInt, BigUint};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Memory-Lean One-Zero Section Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneZeroSectionRecord {
    pub word: Vec<u64>,
    pub first_gap: u64,
    pub quotient_n: BigInt,
    pub centered_carry_x: BigInt,
    pub shell_carry_z: BigInt,
    pub successor: BigInt,
}

/// Typed Safety Certificate for Rejection Hierarchy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyCertificate {
    OddSuccessor { observed_val: BigInt },
    EndpointValuationMismatch { observed_v2: u64 },
    SpineValuationMismatch { observed_t: u64 },
    ShellByteMismatch { candidate_gap: u64, observed_byte: u8, expected_byte: u8 },
    TemplateRule { rule_id: [u8; 32], template_id: u32, witness_data: String },
}

/// Candidate Rule Result for Verification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateRuleResult {
    ProvedSafe { certificate: SafetyCertificate },
    Unclassified,
    CounterexampleToRule { explanation: String },
}

/// Thread-Safe Formula-Based Gap Parameter Cache supporting arbitrary j >= 0
pub struct GapParameterCache {
    cache: RwLock<HashMap<u64, Arc<AcceleratedBranchParams>>>,
}

impl Default for GapParameterCache {
    fn default() -> Self {
        Self::new()
    }
}

impl GapParameterCache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_or_compute(&self, gap_j: u64) -> Arc<AcceleratedBranchParams> {
        {
            let read_guard = self.cache.read().unwrap();
            if let Some(params) = read_guard.get(&gap_j) {
                return Arc::clone(params);
            }
        }

        let params = Arc::new(AcceleratedBranchParams::for_gap(gap_j));
        let mut write_guard = self.cache.write().unwrap();
        write_guard.entry(gap_j).or_insert(Arc::clone(&params));
        params
    }

    /// Compute exact odd 2-adic rational quotient root a_{j,\infty} = Q_j^{-1}(C_\infty - D_j)
    pub fn quotient_root_a_j_infinity(gap_j: u64) -> OddRational2Adic {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        let q_j = &p_j.multiplier;
        let d_j = &p_j.z_endpoint;

        // a_{j,\infty} = -(320 + 2673 * D_j) / (2673 * Q_j)
        let num = -BigInt::from(320u64) - BigInt::from(2673u64) * BigInt::from(d_j.clone());
        let denom = BigUint::from(2673u64) * q_j;

        OddRational2Adic {
            numerator: num,
            denominator: denom,
        }
    }

    /// Compute exact odd 2-adic rational centered-carry root x_{j,\infty} = 2673 * a_{j,\infty}
    pub fn centered_root_x_j_infinity(gap_j: u64) -> OddRational2Adic {
        let a_root = Self::quotient_root_a_j_infinity(gap_j);
        let num = BigInt::from(2673u64) * a_root.numerator;

        // Simplify by dividing common factor 2673
        OddRational2Adic {
            numerator: num / BigInt::from(2673u64),
            denominator: a_root.denominator / BigUint::from(2673u64),
        }
    }
}
