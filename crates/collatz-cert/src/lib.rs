pub mod descent;
pub mod schema;
pub mod verify;

pub use descent::generate_descent_certificate;
pub use schema::{CycleCertificateJson, DescentCertificateJson, InfeasibleCertificateJson};
pub use verify::verify_descent_certificate;

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
}
