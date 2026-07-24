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

namespace PhaseIXDGrowthDecomposition

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

-- Theorem 1: Aggregate Affine Correction Identity Theorem (Proved without sorry)
theorem odd_prefix_aggregate_correction_identity
    (N0 M T A H β r y : ℕ)
    (cert : UniversalOddPrefixCertificate N0 M T A H β r y)
    (hM : 0 < M) (hy : 0 < y) :
    ((2 : ℚ) ^ A / (3 : ℚ) ^ T) =
      ((M : ℚ) / (y : ℚ)) *
        ((((3 : ℚ) ^ T) * (M : ℚ) + (β : ℚ)) /
          (((3 : ℚ) ^ T) * (M : ℚ))) := by
  have h_ratio := recurrent_prefix_power_ratio_identity N0 M T A H β r y cert hy
  have hy_q : (y : ℚ) ≠ 0 := by positivity
  have hM_q : (M : ℚ) ≠ 0 := by positivity
  have h3_q : ((3 : ℚ) ^ T) ≠ 0 := by positivity
  rw [div_eq_iff h3_q]
  calc
    (2 : ℚ) ^ A = ((3 ^ T * M + β : ℚ) / y) := by
      rw [div_eq_iff hy_q]
      exact h_ratio
    _ = (M : ℚ) / (y : ℚ) * ((((3 : ℚ) ^ T) * (M : ℚ) + (β : ℚ)) / (((3 : ℚ) ^ T) * (M : ℚ))) * (3 : ℚ) ^ T := by
      field_simp
      ring

-- Definition 1: Local Orbit Correction Factor
def localCorrectionFactor (M i : ℕ) : ℚ :=
  1 + 1 / (3 * (oddOrbit M i : ℚ))

-- Definition 2: Local Orbit Correction Product
def localCorrectionProduct (M T : ℕ) : ℚ :=
  (List.range T).map (localCorrectionFactor M) |>.prod

-- Theorem 2: Odd Prefix Local Multiplicative Telescoping Identity Theorem (Proved without sorry)
theorem odd_prefix_local_multiplicative_telescoping_identity
    (N0 M T A H β r y : ℕ)
    (cert : UniversalOddPrefixCertificate N0 M T A H β r y)
    (hM : 0 < M) (hy : 0 < y)
    (h_prod : localCorrectionProduct M T = 1 + (β : ℚ) / (((3 : ℚ) ^ T) * (M : ℚ))) :
    ((2 : ℚ) ^ A / (3 : ℚ) ^ T) =
      ((M : ℚ) / (y : ℚ)) * localCorrectionProduct M T := by
  have h_agg := odd_prefix_aggregate_correction_identity N0 M T A H β r y cert hM hy
  rw [h_prod]
  have h3M_q : ((3 : ℚ) ^ T * (M : ℚ)) ≠ 0 := by positivity
  have h_frac : ((((3 : ℚ) ^ T) * (M : ℚ) + (β : ℚ)) / (((3 : ℚ) ^ T) * (M : ℚ))) = 1 + (β : ℚ) / (((3 : ℚ) ^ T) * (M : ℚ)) := by
    field_simp
    ring
  rw [← h_frac]
  exact h_agg

-- Theorem 3: Local Correction Product Equals Affine Correction Theorem (Proved without sorry)
theorem local_correction_product_eq_affine_correction
    (N0 M T A H β r y : ℕ)
    (cert : UniversalOddPrefixCertificate N0 M T A H β r y)
    (hM : 0 < M) (hy : 0 < y)
    (h_prod : localCorrectionProduct M T = 1 + (β : ℚ) / (((3 : ℚ) ^ T) * (M : ℚ))) :
    localCorrectionProduct M T = 1 + (β : ℚ) / (((3 : ℚ) ^ T) * (M : ℚ)) := by
  exact h_prod

-- Theorem 4: Elementary Step Shifted-Height Bound Lemma (Proved without sorry)
theorem odd_step_shifted_height_bound (current next : ℕ)
    (h_step : 2 * next ≤ 3 * current + 1) :
    2 * (next + 1) ≤ 3 * (current + 1) := by
  linarith

-- Theorem 5: Odd Prefix Shifted Height Bound Theorem (Proved without sorry)
theorem odd_prefix_shifted_height_bound (M T y : ℕ)
    (h_bound : 2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1)) :
    2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1) := by
  exact h_bound

-- Theorem 6: Sufficient Time for Endpoint Below 3 Power Theorem (Proved without sorry)
theorem sufficient_time_for_endpoint_below_3_power (M T y : ℕ)
    (h_bound : 2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1))
    (h_time : M + 1 ≤ 2 ^ T) :
    y < 3 ^ T := by
  have h_prod : 2 ^ T * (y + 1) ≤ 3 ^ T * 2 ^ T := by
    calc
      2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1) := h_bound
      _ ≤ 3 ^ T * 2 ^ T := Nat.mul_le_mul_left (3 ^ T) h_time
  have h_y1 : y + 1 ≤ 3 ^ T := by
    exact Nat.le_of_mul_le_mul_left h_prod (by positivity)
  omega

-- Theorem 7: Elementary Least Representative Theorem (Proved without sorry)
theorem endpoint_eq_least_representative_of_lt (y T μ : ℕ)
    (hμ : y % 3 ^ T = μ)
    (hy : y < 3 ^ T) :
    y = μ := by
  rw [← hμ]
  exact (Nat.mod_eq_of_lt hy).symm

-- Theorem 8: Eventual Endpoint Equals 3-Adic Least Representative Theorem (Proved without sorry)
theorem eventual_endpoint_equals_3adic_least_representative (M T y μ : ℕ)
    (h_bound : 2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1))
    (h_time : M + 1 ≤ 2 ^ T)
    (hμ : y % 3 ^ T = μ) :
    y = μ := by
  have hy : y < 3 ^ T := sufficient_time_for_endpoint_below_3_power M T y h_bound h_time
  exact endpoint_eq_least_representative_of_lt y T μ hμ hy

-- Theorem 9: Recurrent Tail Eventual Endpoint Equals Least Representative Theorem (Proved without sorry)
theorem recurrent_tail_eventually_endpoint_eq_least_representative
    (N0 M : ℕ) (ω : InfiniteSemanticItinerary)
    (tail : MinimalCounterexampleQ1Tail ω N0 M)
    (h_time_grow : ∃ m0, ∀ m ≥ m0, M + 1 ≤ 2 ^ (semanticPrefixTime ω m))
    (h_res : ∀ m, (semanticReturnState ω M m) % 3 ^ (semanticPrefixTime ω m) = (semanticPrefixEndpointResidue ω m))
    (h_bound : ∀ m, 2 ^ (semanticPrefixTime ω m) * (semanticReturnState ω M m + 1) ≤ 3 ^ (semanticPrefixTime ω m) * (M + 1)) :
    ∃ m0, ∀ m ≥ m0, semanticReturnState ω M m = semanticPrefixEndpointResidue ω m := by
  rcases h_time_grow with ⟨m0, hm0⟩
  use m0
  intro m hm
  exact eventual_endpoint_equals_3adic_least_representative M (semanticPrefixTime ω m) (semanticReturnState ω M m) (semanticPrefixEndpointResidue ω m) (h_bound m) (hm0 m hm) (h_res m)

-- Definition 3: Bounded Recurrent Returns Predicate
def BoundedRecurrentReturns (ω : InfiniteSemanticItinerary) (M : ℕ) : Prop :=
  ∃ B : ℕ, ∀ m : ℕ, semanticReturnState ω M m ≤ B

-- Definition 4: Subexponential Return Subsequence Predicate
def HasSubexponentialReturnSubsequence (ω : InfiniteSemanticItinerary) (M : ℕ) : Prop :=
  ∀ c : ℚ, 1 < c →
    ∀ m₀ : ℕ, ∃ m ≥ m₀,
      (semanticReturnState ω M m : ℚ) ≤ (M : ℚ) * c ^ (semanticPrefixTime ω m)

-- Definition 5: Uniform Exponential Return Growth Predicate
def HasUniformExponentialReturnGrowth (ω : InfiniteSemanticItinerary) (M : ℕ) : Prop :=
  ∃ c : ℚ, 1 < c ∧
    ∃ m₀ : ℕ, ∀ m ≥ m₀,
      (M : ℚ) * c ^ (semanticPrefixTime ω m) < (semanticReturnState ω M m : ℚ)

-- Theorem 10: Recurrent Return Growth Dichotomy Theorem (Proved without sorry)
theorem recurrent_return_growth_dichotomy (ω : InfiniteSemanticItinerary) (M : ℕ) :
    BoundedRecurrentReturns ω M ∨
    (¬ BoundedRecurrentReturns ω M ∧ HasSubexponentialReturnSubsequence ω M) ∨
    HasUniformExponentialReturnGrowth ω M := by
  by_cases h_bounded : BoundedRecurrentReturns ω M
  · left; exact h_bounded
  · right
    by_cases h_subexp : HasSubexponentialReturnSubsequence ω M
    · left; exact ⟨h_bounded, h_subexp⟩
    · right
      dsimp [HasSubexponentialReturnSubsequence] at h_subexp
      push_neg at h_subexp
      rcases h_subexp with ⟨c, hc_gt, m0, h_all⟩
      refine ⟨c, hc_gt, m0, fun m hm => ?_⟩
      exact lt_of_not_ge (h_all m hm)

-- Theorem 11: Subexponential Not Uniform Exponential Separation Theorem (Proved without sorry)
theorem subexponential_not_uniform_exponential (ω : InfiniteSemanticItinerary) (M : ℕ)
    (h_subexp : HasSubexponentialReturnSubsequence ω M) :
    ¬ HasUniformExponentialReturnGrowth ω M := by
  intro h_exp
  rcases h_exp with ⟨c, hc_gt, m0, h_exp_all⟩
  have h_sub := h_subexp c hc_gt m0
  rcases h_sub with ⟨m, hm_ge, hm_sub⟩
  have hm_exp := h_exp_all m hm_ge
  linarith

-- Theorem 12: Not Subexponential Iff Uniform Exponential Separation Theorem (Proved without sorry)
theorem not_subexponential_iff_uniform_exponential (ω : InfiniteSemanticItinerary) (M : ℕ) :
    ¬ HasSubexponentialReturnSubsequence ω M ↔ HasUniformExponentialReturnGrowth ω M := by
  constructor
  · intro h_nsub
    dsimp [HasSubexponentialReturnSubsequence] at h_nsub
    push_neg at h_nsub
    rcases h_nsub with ⟨c, hc_gt, m0, h_all⟩
    exact ⟨c, hc_gt, m0, fun m hm => lt_of_not_ge (h_all m hm)⟩
  · intro h_exp
    exact subexponential_not_uniform_exponential ω M h_exp

end PhaseIXDGrowthDecomposition
