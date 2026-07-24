pub mod affine;
pub mod affine_interaction;
pub mod cross_form_cylinder;
pub mod execution_sequence;
pub mod guarded_return_classifier;
pub mod inversion;
pub mod lift_kernel;
pub mod quotient_register;
pub mod semantic_cylinders;
pub mod aperiodic_complexity;
pub mod core_selector;
pub mod precision_ledger;
pub mod projective_residues;
pub mod semantic_bridge;
pub mod sturmian_graph;
pub mod symbolic_language;
pub mod counterexample_capture;
pub mod canonical_math;

pub use canonical_math::{canonical_branch, CanonicalBranch};
pub use counterexample_capture::{
    iota, verify_canonical_return, verify_prefix_cylinder_fidelity, CandidateRejection,
    CaptureEvent, EscapeWitness, FiniteCaptureTrace, OrdinaryToCanonicalPrefixExtractor,
    ReturnFailure, ReturnWitness,
};

pub use aperiodic_complexity::{
    DeterministicBenchmarkGenerators, FactorComplexityAnalyzer, MultiscaleCoverageMetrics,
    RecurrenceType, SturmianCubeAnalyzer, SubstitutivePotentialFunction,
};
pub use sturmian_graph::{
    SturmianGapEmbedding, SturmianPhaseNode, SturmianTemplateExtractor, SturmianTransitionGraph,
};
pub use core_selector::{
    CanonicalCoreSelector, CoreTransitionReport, PrimitiveCoreSelection, SelectorOutput,
};
pub use precision_ledger::{FineWilfBound, PrecisionLedger, SwitchLedgerEntry};
pub use projective_residues::{
    detect_constant_suffix, verify_compatible_pair, verify_stabilization_certificate, LiftBlock,
    PrecisionSchedule, ProjectiveResidue, StabilizationCertificate,
};
pub use semantic_bridge::SemanticCoreDistanceBridge;
pub mod u_block_accelerator;
pub mod ultrametric_machine;
pub mod valuation;

pub use lift_kernel::{extend_prefix_state, LiftTransition, PrefixLiftState};
pub use u_block_accelerator::{UBlockAccelerator, UBlockResult};
pub use affine::{compute_descent_threshold, AffineDiagnostics, AffinePrefix, ValuationSemantics};
pub use affine_interaction::{
    AffineInteraction, CoreInteractionKernel, CoreSwitchResult, CoreSwitchType, MacrostepData,
    OddRational2Adic, PeriodicReturnCore, ResonanceOutcome, TwoAdicValuation,
};
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
    Q1Quotient, QuotientAffineRule, QuotientRegisterMachine, ReturnTransitionOutcome, Q1_EXPONENT,
    Q1_RESIDUE,
};
pub use semantic_cylinders::{CanonicalCylinder, ExactWordCylinder};
pub use symbolic_language::{SymbolicLanguageEnumerator, SymbolicWordData};
pub use ultrametric_machine::{
    positive_integer_realization, AbstractEnabledness, AbstractUltrametricState,
    ConcreteUltrametricState, UltrametricMachine, UltrametricStepOutcome, UnitResidue,
    ValuationRegion,
};
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
