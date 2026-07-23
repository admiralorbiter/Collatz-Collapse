pub mod affine;
pub mod affine_interaction;
pub mod cross_form_cylinder;
pub mod execution_sequence;
pub mod guarded_return_classifier;
pub mod inversion;
pub mod quotient_register;
pub mod semantic_cylinders;
pub mod valuation;

pub use affine::{compute_descent_threshold, AffineDiagnostics, AffinePrefix, ValuationSemantics};
pub use affine_interaction::{AffineInteraction, MacrostepData, TwoAdicValuation};
pub use cross_form_cylinder::{
    recover_broad_cylinder, recover_exact_cylinder, recover_sequence_cylinder,
};
pub use execution_sequence::{ExecutionSequence, ThenSequence};
pub use guarded_return_classifier::{
    classify_guarded_return, compose_guarded_path, GuardedPathClassification,
    GuardedReturnClassification, PathCheckpoint, PositiveImageProgression,
};
pub use inversion::{
    hensel_inverse_3_pow, modular_inverse_3k_mod_2A, solve_starting_residue,
    solve_starting_residue_broad, solve_starting_residue_exact,
};
pub use quotient_register::{
    Q1Quotient, QuotientAffineRule, QuotientRegisterMachine, ReturnTransitionOutcome,
    Q1_EXPONENT, Q1_RESIDUE,
};
pub use semantic_cylinders::{CanonicalCylinder, ExactWordCylinder};
pub use valuation::ValuationWord;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum AffineError {
    #[error("Empty valuation word passed to affine computation")]
    EmptyValuationWord,

    #[error("Valuation element zero encountered at index {0}")]
    ZeroValuation(usize),

    #[error("Integer overflow encountered during affine calculation")]
    Overflow,

    #[error("Modular inverse does not exist for modulus {0}")]
    NoModularInverse(String),

    #[error("Integer {0} is not a valid Q_1 state (must be >= 7 and 7 mod 32)")]
    InvalidQ1Integer(String),
}
