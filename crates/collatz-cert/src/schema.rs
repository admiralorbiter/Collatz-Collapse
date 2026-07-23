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

/// Normalized macro sequence step for left-to-right sequence execution schemas (left_to_right_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SequenceStepJson {
    pub symbol: String,
    pub valuation_word: Vec<u32>,
}

/// Guarded Path Certificate Schema v1 (guarded_path_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedPathCertificateJson {
    pub schema_version: String,
    pub execution_semantics: String,
    pub steps: Vec<SequenceStepJson>,
    pub flattened_valuation_word: Vec<u32>,
    pub base_state_residue: String,
    pub base_state_modulus_exponent: u64,
    pub path_source_residue: String,
    pub path_source_modulus_exponent: u64,
    pub composite_multiplier: String,
    pub composite_constant: String,
    pub composite_denominator: String,
}

/// Macrostep Data Schema v1 (macrostep_data_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MacrostepDataJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub odd_steps: usize,
    pub total_valuation: u64,
    pub multiplier: String,
    pub denominator: String,
    pub constant: String,
    pub fixed_point_denominator: String,
}

/// Affine Interaction Schema v1 (affine_interaction_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AffineInteractionJson {
    pub schema_version: String,
    pub p_word: Vec<u32>,
    pub q_word: Vec<u32>,
    pub delta: String,
    pub delta_v2: String,
    pub is_common_center: bool,
    pub same_form_identity_holds: bool,
    pub cross_form_identity_holds: bool,
    pub commutator_identity_holds: bool,
}

/// Cross-Form Cylinder Recovery Schema v1 (cross_form_cylinder_recovery_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossFormCylinderRecoveryJson {
    pub schema_version: String,
    pub p_word: Vec<u32>,
    pub q_word: Vec<u32>,
    pub broad_cylinder_residue: String,
    pub broad_cylinder_modulus_exponent: u64,
    pub exact_cylinder_residue: String,
    pub exact_cylinder_modulus_exponent: u64,
    pub sequence_exact_cylinder_residue: String,
    pub sequence_exact_cylinder_modulus_exponent: u64,
    pub parity_term_preserved: bool,
}

/// Phase 7.3A Verification Report Schema v1 (phase73a_verification_report_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Phase73aVerificationReportJson {
    pub schema_version: String,
    pub macrosteps: Vec<MacrostepDataJson>,
    pub interactions: Vec<AffineInteractionJson>,
    pub cylinder_recoveries: Vec<CrossFormCylinderRecoveryJson>,
    pub all_identities_verified: bool,
}

/// Quotient Register Transition Schema v1 (quotient_register_transition_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuotientRegisterTransitionJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub eta: String,
    pub guard_residue: String,
    pub guard_modulus_exponent: u64,
    pub starting_k: String,
    pub outcome_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_k: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// Guarded Return Classification Schema v1 (guarded_return_classification_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedReturnClassificationJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub exact_word_residue: String,
    pub exact_word_modulus_exponent: u64,
    pub based_return_residue: String,
    pub based_return_modulus_exponent: u64,
    pub positive_image_start: String,
    pub positive_image_step: String,
    pub target_residue: String,
    pub target_modulus_exponent: u64,
    pub quotient_guard_residue: String,
    pub quotient_guard_modulus_exponent: u64,
}

/// Phase 7.3B Verification Report Schema v1 (phase73b_verification_report_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Phase73bVerificationReportJson {
    pub schema_version: String,
    pub transitions: Vec<QuotientRegisterTransitionJson>,
    pub classifications: Vec<GuardedReturnClassificationJson>,
    pub all_register_rules_verified: bool,
}

/// Ultrametric State Transition Schema v1 (ultrametric_state_transition_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UltrametricStateTransitionJson {
    pub schema_version: String,
    pub valuation_word: Vec<u32>,
    pub starting_k: String,
    pub start_x: u64,
    pub start_unit: String,
    pub outcome_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_x: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_unit: Option<String>,
}

/// Phase 7.3B-2 Verification Report Schema v1 (phase73b_2_verification_report_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Phase73b2VerificationReportJson {
    pub schema_version: String,
    pub transitions: Vec<UltrametricStateTransitionJson>,
    pub all_commuting_diagrams_verified: bool,
}
