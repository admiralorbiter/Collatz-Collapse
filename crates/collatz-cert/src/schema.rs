use serde::{Deserialize, Serialize};

/// Descent Certificate Schema v1 (descent_v1)
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

/// Minimality Infeasible Certificate Schema (infeasible_minimality_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfeasibleMinimalityCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub starting_residue: String,
    pub modulus_exponent: u64,
    pub constant: String,
    pub intermediate_step_index: usize,
    pub bound_threshold: String,
}

/// Algebraic Empty Intersection Certificate Schema (infeasible_algebraic_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfeasibleAlgebraicCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub starting_residue: String,
    pub modulus_exponent: u64,
    pub modulus_secondary: u64,
    pub residue_secondary: u64,
    pub crt_empty_intersection_proof: String,
}

/// Subsumption Path Merging Certificate Schema (infeasible_subsumption_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfeasibleSubsumptionCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub target_valuation_word: Vec<u32>,
    pub subsumption_reason: String,
}
