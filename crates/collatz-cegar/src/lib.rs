#![allow(clippy::all)]

pub mod abstract_domain;
pub mod alphabet_manifest;
pub mod closed_walk_analyzer;
pub mod concretization;
pub mod cylinder_partition;
pub mod destination_refinement_m72;
pub mod engine;
pub mod feature_counterexample_search;
pub mod fixed_point_synthesizer;
pub mod graph_closure_m72;
pub mod graph_contraction_solver;
pub mod guarded_path;
pub mod guarded_return_classifier;
pub mod karp_cycle;
pub mod non_commuting_cycles;
pub mod path_semantics;
pub mod product_state_m72;
pub mod refinement;
pub mod relational_domain;
pub mod scalar_lyapunov_solver;
pub mod sct_engine;
pub mod semantic_gate;
pub mod ultrametric_machine;

pub use abstract_domain::{AbstractEdge, RelationalState};
pub use alphabet_manifest::{AlphabetManifestCertificateJson, AlphabetManifestEngine};
pub use closed_walk_analyzer::{ClosedWalkAnalyzer, TransportedClosedWalk};
pub use concretization::{ConcretizationEngine, ConcretizationResult, MultiLapCycleSolution};
pub use cylinder_partition::{CylinderPartitionCertificateJson, CylinderPartitionEngine};
pub use destination_refinement_m72::{DestinationRefinementEngine, RefinementRequirement};
pub use engine::{CegarEngine, CegarEngineConfig, CegarEngineReport};
pub use feature_counterexample_search::{FeatureCounterexampleSearchEngine, FeatureRelationKind};
pub use fixed_point_synthesizer::{
    ExactValuation, FixedPointSynthesisResult, FixedPointSynthesizer,
};
pub use graph_closure_m72::{GraphClosureEngine, GuardedEdge};
pub use graph_contraction_solver::GraphContractionSolver;
pub use guarded_path::{
    BasedReturnCylinder, GuardCheckpoint, GuardedPathCylinder, StateId, StateMembership,
};
pub use guarded_return_classifier::{
    classify_guarded_return, compose_guarded_path, GuardedPathClassification,
    GuardedReturnClassification, PathCheckpoint, PositiveImageProgression,
};
pub use karp_cycle::KarpCycleEngine;
pub use non_commuting_cycles::{NonCommutingCycleAnalyzer, ReturnCycle};
pub use path_semantics::{PathSemanticsCertificateJson, PathSemanticsEngine};
pub use product_state_m72::ProductState;
pub use refinement::{NegativeRefinementLemmaJson, RefinementEngine};
pub use relational_domain::{
    compute_canonical_relational_graph_hash, construct_symbolic_relational_transitions,
    SymbolicControlState, SymbolicRelationalSolver, SymbolicTransitionEdge,
};
pub use scalar_lyapunov_solver::ScalarLyapunovSolver;
pub use sct_engine::{RelationValue, SctEngine, SizeChangeGraph};
pub use semantic_gate::{CylinderImage, SemanticGate, WordForcingStatus};
pub mod induced_v_map;
pub mod accelerated_ranking;
pub mod lift_digit_engine;
pub mod periodic_necklace_analyzer;

pub use induced_v_map::{DyadicBranchNormalForm, InducedVMapEngine, InducedVTransition};
pub use accelerated_ranking::{
    AcceleratedRankingEngine, AcceleratedRankingProof, AcceleratedRankingStatus,
};
pub use lift_digit_engine::{InfiniteRealizationStatus, LiftDigitEngine, ZeroLiftChain};

pub mod accelerated_transition_system;
pub mod accelerated_lift;
pub mod accelerated_invariant_search;

pub use accelerated_transition_system::{AcceleratedEdge, AcceleratedTransitionSystemEngine};
pub use accelerated_lift::{AcceleratedLiftEngine, AcceleratedLiftExtension, AcceleratedPrefixState};
pub use accelerated_invariant_search::{
    AcceleratedInvariantEngine, AcceleratedInvariantSearchResult, AcceleratedInvariantStatus,
};

pub mod zero_lift_continuation;
pub mod zero_lift_explorer;
pub mod invariant_falsifier;

pub use zero_lift_continuation::{ZeroLiftContinuationEngine, ZeroLiftStepResult};
pub use zero_lift_explorer::{ZeroLiftExplorer, ZeroLiftWitness};
pub use invariant_falsifier::{InvariantFalsificationResult, InvariantFalsifier};
pub use periodic_necklace_analyzer::{
    PeriodicNecklaceAnalyzer, PeriodicOrbitData, TopologicalEntropyEngine, ValuationHistogramEntry,
};
pub use ultrametric_machine::{
    positive_integer_realization, AbstractEnabledness, AbstractUltrametricState,
    ConcreteUltrametricState, UltrametricMachine, UltrametricStepOutcome, UnitResidue,
    ValuationRegion,
};
