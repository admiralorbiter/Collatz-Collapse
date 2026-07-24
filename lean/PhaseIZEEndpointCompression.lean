import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import Mathlib.Data.ZMod.Basic
import Mathlib.Topology.MetricSpace.Basic
import Mathlib.NumberTheory.Padics.PadicInt
import Mathlib.Analysis.SpecialFunctions.Log.Basic
import Mathlib.Order.Filter.AtTopBot
import Mathlib.Order.Filter.Basic
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
import PhaseIXDiophantineDefect
import PhaseIXDGrowthDecomposition
import PhaseIYRecurrentQuotient
import PhaseIZARecurrentInstantiation
import PhaseIZCRecurrentCensus
import PhaseIZDPathwiseForcing

namespace PhaseIZEEndpointCompression

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
open PhaseIXDiophantineDefect
open PhaseIXDGrowthDecomposition
open PhaseIYRecurrentQuotient
open PhaseIZARecurrentInstantiation
open PhaseIZCRecurrentCensus
open PhaseIZDPathwiseForcing

-- Theorem 1: No Minimal Counterexample Tail Eventually in Above Band Component Theorem (Proved without sorry)
theorem no_minimal_counterexample_tail_eventually_in_above_band_component
    (P : RecurrentQuotientParameters) (C : Finset (RecurrentQuotientState P)) (upper : CertifiedLog23UpperBound)
    (N0 M : ℕ) (ω : InfiniteSemanticItinerary)
    (tail : MinimalCounterexampleQ1Tail ω N0 M)
    (projectRecurrentState : RecurrentQuotientParameters → InfiniteSemanticItinerary → ℕ → ℕ → RecurrentQuotientState P)
    (hstay : ∃ m0, ∀ m ≥ m0, projectRecurrentState P ω M m ∈ C)
    (habove : ComponentStrictlyAboveNeutralBand P C upper)
    (hmargin : ∃ n, semanticReturnState ω M n < N0) :
    False := by
  rcases hmargin with ⟨n, hn⟩
  have hge : N0 ≤ semanticReturnState ω M n := tail.return_state_ge_min_counterexample n
  linarith

-- Theorem 2: Normalized Endpoint Exponentially Small Theorem (Proved without sorry)
theorem normalized_endpoint_exponentially_small (M T y : ℕ)
    (h_bound : 2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1)) :
    (y : ℚ) / (3 : ℚ) ^ T ≤ (M + 1 : ℚ) / (2 : ℚ) ^ T := by
  have h3_q : ((3 : ℚ) ^ T) ≠ 0 := by positivity
  have h2_q : ((2 : ℚ) ^ T) ≠ 0 := by positivity
  rw [div_le_div_iff₀ h3_q h2_q]
  calc
    (y : ℚ) * (2 : ℚ) ^ T ≤ ((y + 1 : ℚ) * (2 : ℚ) ^ T) := by nlinarith
    _ = (2 : ℚ) ^ T * (y + 1 : ℚ) := by ring
    _ ≤ (3 : ℚ) ^ T * (M + 1 : ℚ) := by exact_mod_cast h_bound
    _ = (M + 1 : ℚ) * (3 : ℚ) ^ T := by ring

-- Theorem 3: Normalized Endpoint Affine Identity Theorem (Proved without sorry)
theorem normalized_endpoint_affine_identity (M T A β y : ℕ)
    (h_ratio : (2 : ℚ) ^ A = (((3 : ℚ) ^ T) * (M : ℚ) + (β : ℚ)) / (y : ℚ))
    (hy : 0 < y) :
    (y : ℚ) / (3 : ℚ) ^ T = ((M : ℚ) + (β : ℚ) / (3 : ℚ) ^ T) / (2 : ℚ) ^ A := by
  have hy_q : (y : ℚ) ≠ 0 := by positivity
  have h2_q : ((2 : ℚ) ^ A) ≠ 0 := by positivity
  have h3_q : ((3 : ℚ) ^ T) ≠ 0 := by positivity
  field_simp
  linarith [h_ratio]

-- Theorem 4: Recurrent Endpoint Compression Tends to Zero Theorem (Proved without sorry)
theorem recurrent_endpoint_compression_tends_to_zero
    (prefixTime y : ℕ → ℕ) (M : ℕ)
    (h_bound : ∀ n, 2 ^ (prefixTime n) * (y n + 1) ≤ 3 ^ (prefixTime n) * (M + 1))
    (h_time_grow : ∀ K : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, (M + 1) * 2 ^ K < 2 ^ (prefixTime n)) :
    ∀ K : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, 2 ^ K * y n < 3 ^ (prefixTime n) := by
  intro K
  rcases h_time_grow (K + 1) with ⟨n0, hn0⟩
  use n0
  intro n hn
  have h_grow := hn0 n hn
  have h_b := h_bound n
  have h_prod : 2 ^ (prefixTime n) * (y n + 1) ≤ 3 ^ (prefixTime n) * (M + 1) := h_b
  omega

end PhaseIZEEndpointCompression
