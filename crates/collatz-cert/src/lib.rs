#![allow(clippy::all)]

pub mod accelerated_ranking_verifier;
pub mod affine_interaction_verifier;
pub mod batch;
pub mod descent;
pub mod graph_contraction;
pub mod guarded_path_verifier;
pub mod lean_export;
pub mod macrocycle_theorem;
pub mod minus_one_countdown;
pub mod quotient_register_verifier;
pub mod scalar_lyapunov;
pub mod schema;
pub mod sct_engine;
pub mod symbolic_language_verifier;
pub mod tail;
pub mod ultrametric_verifier;
pub mod valuation_countdown;
pub mod verify;
pub mod verify_sct;

pub use accelerated_ranking_verifier::{verify_accelerated_invariant_report, verify_phase73d_report};

pub use affine_interaction_verifier::verify_phase73a_report;
pub use batch::{
    export_certificate_bundle, export_manifest, verify_certificate_bundle, BundleManifest,
};
pub use descent::generate_descent_certificate;
pub use graph_contraction::{
    verify_graph_contraction_certificate, GraphContractionCertificateJson, GraphContractionError,
    GraphEdge, ObstructionCycleJson, RationalRatioJson,
};
pub use guarded_path_verifier::verify_guarded_path_certificate;
pub use lean_export::{
    export_lean4_accelerated_invariant_theorem, export_lean4_accelerated_theorem,
    export_lean4_affine_interaction_theorem, export_lean4_quotient_register_theorem,
    export_lean4_sct_ranking_theorem, export_lean4_symbolic_language_theorem, export_lean4_theorem,
    export_lean4_ultrametric_theorem,
};
pub use macrocycle_theorem::verify_finite_fuel_macrocycle_certificate;
pub use minus_one_countdown::{
    verify_minus_one_countdown_certificate, MinusOneCountdownCertificateJson,
    MinusOneCountdownError,
};
pub use quotient_register_verifier::verify_phase73b_report;
pub use scalar_lyapunov::{
    verify_scalar_lyapunov_certificate, ScalarLyapunovCertificateJson, ScalarLyapunovError,
    ScalarTransition, ValuationConstraint,
};
pub use schema::{
    AffineInteractionJson, AffineMapJson, BuchiEmptinessCertificateJson, BuchiTransitionJson,
    CrossFormCylinderRecoveryJson, CycleCertificateJson, DescentCertificateJson,
    FeatureDefinitionJson, GuardedPathCertificateJson, GuardedReturnClassificationJson,
    InfeasibleAlgebraicCertificateJson, InfeasibleMinimalityCertificateJson,
    InfeasibleSubsumptionCertificateJson, MacrostepDataJson, Phase73aVerificationReportJson,
    Phase73b2VerificationReportJson, Phase73bVerificationReportJson,
    Phase73cVerificationReportJson, QuotientRegisterTransitionJson, SctEdgeCertificateJson,
    SequenceStepJson, SizeChangeCertificateJson, SizeChangeRelationJson, SizeChangeRelationKind,
    SizeChangeTransitionGraphJson, SourceGuardJson, SymbolicWordClassificationJson,
    TailDescentCertificateJson, UltrametricStateTransitionJson,
};
pub use symbolic_language_verifier::verify_phase73c_report;
pub use tail::{compute_a_crit, generate_tail_descent_certificate};
pub use ultrametric_verifier::verify_phase73b_2_report;
pub use valuation_countdown::{
    verify_valuation_countdown_certificate, ValuationCountdownCertificateJson,
    ValuationCountdownError,
};

pub use verify::{verify_descent_certificate, verify_tail_descent_certificate};
pub use verify_sct::{
    verify_sct_edge_certificate, verify_sct_scc_certificate, SctVerificationError,
};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum VerificationError {
    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: String, found: String },

    #[error("Valuation word domain invalid: {0}")]
    InvalidValuationWord(String),

    #[error("Total twos mismatch: declared {declared}, computed {computed}")]
    TotalTwosMismatch { declared: u64, computed: u64 },

    #[error("Constant c_k mismatch: declared {declared}, computed {computed}")]
    ConstantMismatch { declared: String, computed: String },

    #[error("Starting residue mismatch: declared {declared}, computed {computed}")]
    ResidueMismatch { declared: String, computed: String },

    #[error("Multiplicative contraction failed: 2^A_k ({pow2}) <= 3^k ({pow3})")]
    NoMultiplicativeContraction { pow2: String, pow3: String },

    #[error("Descent threshold B mismatch: declared {declared}, computed {computed}")]
    ThresholdMismatch { declared: String, computed: String },

    #[error("Exception verification failed for integer {integer}: trajectory did not descend")]
    ExceptionVerificationFailed { integer: String },

    #[error("JSON Deserialization / Parsing error: {0}")]
    JsonError(String),

    #[error("BigInt parse error: {0}")]
    ParseBigIntError(String),

    #[error("Digit count exceeded limit ({limit}): string length {length}")]
    MaxDigitsExceeded { length: usize, limit: usize },

    #[error("Subsumption verification failed: {0}")]
    SubsumptionVerificationFailed(String),
}
