use serde::{Deserialize, Serialize};

/// Descent Certificate Schema v1 (descent_v1)
/// All BigInt fields are serialized as Strings to prevent JSON float rounding errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescentCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub starting_residue: String,
    pub modulus_exponent: u64,
    pub constant: String,
    pub descent_threshold: String,
    pub checked_exceptions: Vec<String>,
}

/// Cycle Certificate Schema v1 (cycle_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycleCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub starting_integer: String,
    pub intermediate_values: Vec<String>,
    pub is_nontrivial: bool,
}

/// Infeasible Prefix Certificate Schema v1 (infeasible_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfeasibleCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub starting_residue: String,
    pub modulus_exponent: u64,
    pub constant: String,
    pub rejection_reason: String,
    pub intermediate_step_index: usize,
    pub bound_threshold: String,
}
