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

namespace TrustedStatementSurface

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights
open PhaseIRAvoidanceCompletion
open PhaseISUniversalCertificate

/-!
# Trusted Statement Surface: Universal Counterexample Reduction

This file exposes the minimal, self-contained trusted statement surface
for the universal counterexample reduction of the Collatz Conjecture.
-/

-- Headline 1: Minimal Counterexample Definition
#print IsMinimalOddCounterexample

-- Headline 2: Infinite Semantic Itinerary Natural Point Classification
#print natural_semantic_realization_iff_eventual_zero_lift

-- Headline 3: Infinite Avoiding Itinerary Natural Point Classification
#print natural_avoiding_realization_iff_eventual_zero_lift

-- Headline 4: Universal Odd Prefix Certificate Structure
#print UniversalOddPrefixCertificate

-- Headline 5: Master Universal Synthesis Theorem
#print minimal_counterexample_has_universally_certified_tail

-- Axiom Audits
#print axioms minimal_counterexample_has_universally_certified_tail
#print axioms natural_semantic_realization_iff_eventual_zero_lift
#print axioms natural_avoiding_realization_iff_eventual_zero_lift
#print axioms minimal_counterexample_dual_2adic_coding_synthesis

end TrustedStatementSurface
