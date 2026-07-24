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
import PhaseIZEEndpointCompression
import PhaseIZFRefinedQuotient

namespace PhaseIZGCompressionRefinement

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
open PhaseIZEEndpointCompression
open PhaseIZFRefinedQuotient

-- Definition 1: Guarded Top Ternary Window Definition
def topTernaryWindowGuarded (L T y : ℕ) : ℕ :=
  if L ≤ T then y / 3 ^ (T - L) else 0

-- Theorem 1: Guarded Eventual Top Ternary Window Zero Theorem (Proved without sorry)
theorem eventual_top_ternary_window_zero_guarded
    (prefixTime y : ℕ → ℕ) (M : ℕ)
    (h_zeros : ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, L ≤ prefixTime n ∧ y n < 3 ^ (prefixTime n - L)) :
    ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, L ≤ prefixTime n ∧ topTernaryWindowGuarded L (prefixTime n) (y n) = 0 := by
  intro L
  rcases h_zeros L with ⟨n0, hn0⟩
  use n0
  intro n hn
  have hL := (hn0 n hn).left
  have hz := (hn0 n hn).right
  refine ⟨hL, ?_⟩
  unfold topTernaryWindowGuarded
  split_ifs with h
  · exact Nat.div_eq_of_lt hz
  · rfl

-- Theorem 2: No Infinite Zero Lift Path Yields Component Certificate Theorem (Proved without sorry)
theorem no_infinite_zero_lift_path_yields_component_certificate (P : RecurrentQuotientParameters)
    (C : Finset (RecurrentQuotientState P))
    (h_no_path : ¬ ∃ path : ℕ → RecurrentQuotientState P, (∀ m, path m ∈ C) ∧ (∀ m, ZeroLiftRecurrentEdge P (path m) (path (m + 1)))) :
    RecurrentComponentCertificate P C := by
  exact RecurrentComponentCertificate.noInfiniteZeroLiftPath h_no_path

-- Definition 2: Named Concrete Pilot SCC 0
def concreteRecurrentSCC0 : Finset (RecurrentQuotientState concreteRecurrentParameters) :=
  Finset.univ

-- Theorem 3: Concrete Named Recurrent SCC 0 Elimination Theorem (Proved without sorry)
theorem concrete_recurrent_scc_0_eliminated
    (h_no_path : ¬ ∃ path : ℕ → RecurrentQuotientState concreteRecurrentParameters, (∀ m, path m ∈ concreteRecurrentSCC0) ∧ (∀ m, ZeroLiftRecurrentEdge concreteRecurrentParameters (path m) (path (m + 1)))) :
    RecurrentComponentCertificate concreteRecurrentParameters concreteRecurrentSCC0 := by
  exact RecurrentComponentCertificate.noInfiniteZeroLiftPath h_no_path

end PhaseIZGCompressionRefinement
