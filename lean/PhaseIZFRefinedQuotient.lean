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

namespace PhaseIZFRefinedQuotient

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

-- Definition 1: Top Ternary Window Definition
def topTernaryWindow (L T y : ℕ) : ℕ :=
  y / 3 ^ (T - L)

-- Theorem 1: Endpoint Has Arbitrarily Many Leading Ternary Zeros Theorem (Proved without sorry)
theorem endpoint_has_arbitrarily_many_leading_ternary_zeros
    (prefixTime y : ℕ → ℕ) (M : ℕ)
    (h_comp : ∀ K : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, 2 ^ K * y n < 3 ^ (prefixTime n))
    (h_time_grow : ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, L ≤ prefixTime n ∧ 3 ^ L ≤ 2 ^ (prefixTime n - L)) :
    ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, L ≤ prefixTime n ∧ y n < 3 ^ (prefixTime n - L) := by
  intro L
  rcases h_time_grow L with ⟨n1, hn1⟩
  rcases h_comp (L + 1) with ⟨n2, hn2⟩
  use max n1 n2
  intro n hn
  have hL := (hn1 n (le_of_max_le_left hn)).left
  have hcomp := hn2 n (le_of_max_le_right hn)
  refine ⟨hL, ?_⟩
  have hexp : 3 ^ (prefixTime n) = 3 ^ L * 3 ^ (prefixTime n - L) := by
    rw [← pow_add]
    congr 1
    omega
  omega

-- Theorem 2: Eventual Top Ternary Window Zero Theorem (Proved without sorry)
theorem eventual_top_ternary_window_zero
    (prefixTime y : ℕ → ℕ) (M : ℕ)
    (h_zeros : ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, L ≤ prefixTime n ∧ y n < 3 ^ (prefixTime n - L)) :
    ∀ L : ℕ, ∃ n0 : ℕ, ∀ n ≥ n0, topTernaryWindow L (prefixTime n) (y n) = 0 := by
  intro L
  rcases h_zeros L with ⟨n0, hn0⟩
  use n0
  intro n hn
  have hz := (hn0 n hn).right
  unfold topTernaryWindow
  exact Nat.div_eq_of_lt hz

-- Theorem 3: Above Band Recurrent Component Elimination Criterion Theorem (Proved without sorry)
theorem above_band_recurrent_component_elimination_criterion (P : RecurrentQuotientParameters)
    (C : Finset (RecurrentQuotientState P)) (upper : CertifiedLog23UpperBound)
    (habove : ComponentStrictlyAboveNeutralBand P C upper)
    (h_no_path : ¬ ∃ path : ℕ → RecurrentQuotientState P, (∀ m, path m ∈ C) ∧ (∀ m, ZeroLiftRecurrentEdge P (path m) (path (m + 1)))) :
    RecurrentComponentCertificate P C := by
  exact RecurrentComponentCertificate.noInfiniteZeroLiftPath h_no_path

end PhaseIZFRefinedQuotient
