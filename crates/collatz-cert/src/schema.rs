use serde::{Deserialize, Serialize};

/// Descent Certificate Schema v1 (descent_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valuation_semantics: Option<String>,
}

/// Tail Descent Certificate Schema v1 (tail_descent_v1)
/// Certifies infinite child valuations a_k >= a_crit analytically (descent threshold B <= 1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TailDescentCertificateJson {
    pub schema_version: String,
    pub prefix_word: Vec<u32>,
    pub prefix_total_twos: u64,
    pub prefix_constant: String,
    pub minimum_child_valuation: u32,
    pub proof_bound: String,
}

/// Cycle Certificate Schema v1 (cycle_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct InfeasibleSubsumptionCertificateJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub odd_steps: usize,
    pub target_valuation_word: Vec<u32>,
    pub target_total_twos: u64,
    pub source_constant: String,
    pub target_constant: String,
    pub residue_offset: String,
    pub step_offset: usize,
    pub subsumption_reason: String,
}

/// Size-Change Relation Kind (decrease, non_increase, reset)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizeChangeRelationKind {
    Decrease,
    NonIncrease,
    Reset,
}

/// Bipartite Size-Change Feature Relation Edge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeChangeRelationJson {
    pub src_feature: String,
    pub relation: SizeChangeRelationKind,
    pub dst_feature: String,
}

/// Transition Graph between Abstract State Vertices
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeChangeTransitionGraphJson {
    pub source_node: String,
    pub target_node: String,
    pub valuation_word: Vec<u32>,
    pub relations: Vec<SizeChangeRelationJson>,
}

/// Size-Change Termination Certificate Schema (size_change_scc_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeChangeCertificateJson {
    pub schema_version: String,
    pub scc_id: String,
    pub feature_vector: Vec<String>,
    pub vertices: Vec<String>,
    pub transition_graphs: Vec<SizeChangeTransitionGraphJson>,
    pub canonical_edge_ordering: Vec<String>,
    pub verifier_recomputation_required: bool,
}

/// Subordinate Edge Soundness Certificate Schema (sct_edge_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceGuardJson {
    pub residue: String,
    pub modulus_exponent: u64,
    pub positivity_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AffineMapJson {
    pub odd_steps: usize,
    pub total_twos: u64,
    pub constant: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureDefinitionJson {
    pub feature_id: String,
    pub kind: String,
    pub alpha: String,
    pub beta: String,
    pub zero_case: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SctEdgeCertificateJson {
    pub schema_version: String,
    pub edge_id: String,
    pub source_state: String,
    pub target_state: String,
    pub valuation_word: Vec<u32>,
    pub source_guard: SourceGuardJson,
    pub affine_map: AffineMapJson,
    pub features: Vec<FeatureDefinitionJson>,
    pub proved_relations: Vec<SizeChangeRelationJson>,
    pub proof_kind: String,
}

/// Büchi Automaton Transition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuchiTransitionJson {
    pub src: String,
    pub symbol: String,
    pub dst: String,
}

/// Büchi Automaton Language Emptiness Certificate Schema (buchi_emptiness_scc_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuchiEmptinessCertificateJson {
    pub schema_version: String,
    pub scc_id: String,
    pub alphabet: Vec<String>,
    pub states: Vec<String>,
    pub initial_state: String,
    pub accepting_states: Vec<String>,
    pub transitions: Vec<BuchiTransitionJson>,
    pub reachable_states: Vec<String>,
    pub scc_decomposition: Vec<Vec<String>>,
    pub verifier_recomputation_required: bool,
}



