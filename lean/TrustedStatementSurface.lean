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

/-!
# Trusted Statement Surface: Universal Counterexample Reduction & Component Coverage

This file exposes the minimal, self-contained trusted statement surface
for the universal counterexample reduction, concrete avoiding component elimination,
reachability induction, and master recurrent-branch forced synthesis.
-/

-- Headline 1: Minimal Counterexample Definition
#print IsMinimalOddCounterexample

-- Headline 2: Universal Odd Prefix Certificate Structure
#print UniversalOddPrefixCertificate

-- Headline 3: Master Universal Synthesis Theorem
#print minimal_counterexample_has_universally_certified_tail

-- Headline 4: Concrete Avoiding Quotient State & Sound Edge Coverage
#print ConcreteAvoidingQuotientState
#print ConcreteAvoidingEdge
#print actual_avoiding_extension_projects_to_sound_edge

-- Headline 5: Initial Mature Reachability & Induction
#print RealizableInitialMatureState
#print SoundReachableState
#print actual_avoiding_tail_projects_into_sound_reachable

-- Headline 6: Component Disposition & Conditional Master Recurrent Forced Synthesis
#print CertifiedSCCPartition
#print ComponentDisposition
#print no_avoiding_tail_if_all_persistent_components_disposed
#print minimal_counterexample_must_be_q1_recurrent

-- Axiom Audits
#print axioms minimal_counterexample_has_universally_certified_tail
#print axioms actual_avoiding_extension_projects_to_sound_edge
#print axioms actual_avoiding_tail_projects_into_sound_reachable
#print axioms no_avoiding_tail_if_all_persistent_components_disposed
#print axioms minimal_counterexample_must_be_q1_recurrent

end TrustedStatementSurface
