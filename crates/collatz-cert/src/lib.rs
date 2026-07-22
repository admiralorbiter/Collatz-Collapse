pub mod batch;
pub mod descent;
pub mod graph_contraction;
pub mod lean_export;
pub mod scalar_lyapunov;
pub mod schema;
pub mod tail;
pub mod valuation_countdown;
pub mod verify;

pub use batch::{export_certificate_bundle, export_manifest, verify_certificate_bundle, BundleManifest};
pub use descent::generate_descent_certificate;
pub use graph_contraction::{
    verify_graph_contraction_certificate, GraphContractionCertificateJson, GraphContractionError,
    GraphEdge, ObstructionCycleJson, RationalRatioJson,
};
pub use lean_export::export_lean4_theorem;
pub use scalar_lyapunov::{
    verify_scalar_lyapunov_certificate, ScalarLyapunovCertificateJson, ScalarLyapunovError,
    ScalarTransition, ValuationConstraint,
};
pub use schema::{
    CycleCertificateJson, DescentCertificateJson, InfeasibleAlgebraicCertificateJson,
    InfeasibleMinimalityCertificateJson, InfeasibleSubsumptionCertificateJson, TailDescentCertificateJson,
};
pub use tail::{compute_a_crit, generate_tail_descent_certificate};
pub use valuation_countdown::{
    verify_valuation_countdown_certificate, ValuationCountdownCertificateJson, ValuationCountdownError,
};
pub use verify::{verify_descent_certificate, verify_tail_descent_certificate};





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

