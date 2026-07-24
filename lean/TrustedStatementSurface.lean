import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.ZMod.Basic
import PhaseI1CounterexampleCapture
import PhaseINOrbitItineraryBridge
import PhaseIOCounterexampleRigidity
import PhaseIPEndpointAndAvoidance
import PhaseIQAvoidanceCompilerAndHeights
import PhaseIRAvoidanceCompletion
import PhaseISUniversalCertificate
import PhaseITQuotientsAndPotentials
import PhaseIUConcreteElimination
import PhaseIVComponentCoverage
import PhaseIWComponentCensus

namespace TrustedStatementSurface

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights
open PhaseIRAvoidanceCompletion
open PhaseISUniversalCertificate
open PhaseITQuotientsAndPotentials
open PhaseIUConcreteElimination
open PhaseIVComponentCoverage
open PhaseIWComponentCensus

/-!
# Trusted Statement Surface: Universal Counterexample Reduction & Complete Component Census

This file exposes the minimal, self-contained trusted statement surface
for the universal counterexample reduction, concrete component partition,
component relevance classification, and unconditional master recurrent-branch forced synthesis.
-/

-- Headline 1: Minimal Counterexample Definition
#print IsMinimalOddCounterexample

-- Headline 2: Universal Odd Prefix Certificate Structure
#print UniversalOddPrefixCertificate

-- Headline 3: Master Universal Synthesis Theorem
#print minimal_counterexample_has_universally_certified_tail

-- Headline 4: Concrete Component ID & Lookup Partition
#print ConcreteComponentId
#print concreteComponentOf
#print concreteComponentStates
#print same_component_reachable
#print mutually_reachable_same_component

-- Headline 5: Relevant Component Classification & Tail Residence
#print ConcreteComponentReachable
#print ConcreteComponentCyclic
#print ConcreteRelevantComponent
#print ConcreteReachablePersistentComponents
#print actual_avoiding_tail_eventually_stays_in_relevant_component

-- Headline 6: Unconditional Elimination & Master Recurrent Forced Synthesis
#print ComponentEliminationCertificate
#print all_concrete_relevant_components_eliminated
#print no_minimal_counterexample_avoiding_tail
#print minimal_counterexample_must_be_q1_recurrent_unconditional

-- Headline 7: Clean Number-Theoretic Headline Theorem
#print HasInfinitelyManyQ1Returns
#print minimal_odd_counterexample_returns_to_q1_infinitely_often

-- Axiom Audits
#print axioms minimal_counterexample_has_universally_certified_tail
#print axioms actual_avoiding_tail_eventually_stays_in_relevant_component
#print axioms all_concrete_relevant_components_eliminated
#print axioms no_minimal_counterexample_avoiding_tail
#print axioms minimal_counterexample_must_be_q1_recurrent_unconditional
#print axioms minimal_odd_counterexample_returns_to_q1_infinitely_often

end TrustedStatementSurface
