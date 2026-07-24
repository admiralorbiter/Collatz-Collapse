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

namespace PhaseITQuotientsAndPotentials

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights
open PhaseIRAvoidanceCompletion
open PhaseISUniversalCertificate

-- Definition 1: Finite Drift Sign Enum Type
inductive DriftSign
  | negative
  | zero
  | positive
  deriving DecidableEq, Fintype

-- Definition 2: Finite Tail Branch Enum Type
inductive TailBranch
  | recurrent
  | avoiding
  deriving DecidableEq, Fintype

-- Definition 3: Quotient Parameters Structure with Positive Period Proofs
structure QuotientParameters where
  h : ℕ
  k : ℕ
  exponentPeriod : ℕ
  timePeriod : ℕ
  exponentPeriod_pos : 0 < exponentPeriod
  timePeriod_pos : 0 < timePeriod

-- Definition 4: Finite Projected Quotient State Structure
structure QuotientState (P : QuotientParameters) where
  sourceResidue : Fin (2 ^ P.h)
  endpointResidue : Fin (3 ^ P.k)
  exponentMod : Fin P.exponentPeriod
  timeMod : Fin P.timePeriod
  driftSign : DriftSign
  branch : TailBranch
  deriving DecidableEq, Fintype

-- Definition 5: Quotient Maturity Eligibility Predicate
def QuotientEligible (P : QuotientParameters) (H T : ℕ) : Prop :=
  P.h ≤ H ∧ P.k ≤ T

-- Definition 6: Universal Prefix Data Structure
structure UniversalPrefixData where
  time : ℕ
  precision : ℕ
  exponentSum : ℕ
  affineOffset : ℕ
  representative : ℕ
  endpoint : ℕ

-- Definition 7: Branch-Independent Certified Tail System Structure
structure CertifiedTailSystem where
  prefix : ℕ → UniversalPrefixData
  N0 : ℕ
  M : ℕ
  certificate : ∀ m, UniversalOddPrefixCertificate N0 M
    (prefix m).time (prefix m).exponentSum (prefix m).precision
    (prefix m).affineOffset (prefix m).representative (prefix m).endpoint
  time_strict : ∀ m, (prefix m).time < (prefix (m + 1)).time
  precision_tendsto : Filter.Tendsto (fun m => (prefix m).precision) Filter.atTop Filter.atTop

-- Definition 8: Sound Finite Tail Abstraction Structure
structure SoundFiniteTailAbstraction (system : CertifiedTailSystem) (P : QuotientParameters) where
  State : Type
  instFintype : Fintype State
  instDecidableEq : DecidableEq State
  project : ℕ → State
  Edge : State → State → Prop
  step_sound : ∀ m, Edge (project m) (project (m + 1))

-- Definition 9: Strict Rational Ranking Predicate
def StrictRationalRanking {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (ε : ℚ) : Prop :=
  0 < ε ∧ ∀ s t, Q.Edge s t → V t - V s ≤ -ε

-- Theorem 1: Strict Rational Ranking Eliminates Infinite Quotient Path Theorem (Proved without sorry)
theorem strict_ranking_eliminates_infinite_quotient_path {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (ε : ℚ)
    (hV : StrictRationalRanking Q V ε) : False := by
  have h_dec : DecidableEq Q.State := Q.instDecidableEq
  have h_fin : Fintype Q.State := Q.instFintype
  obtain ⟨h_eps, h_edge⟩ := hV
  have h_step : ∀ m, V (Q.project (m + 1)) - V (Q.project m) ≤ -ε := by
    intro m
    exact h_edge (Q.project m) (Q.project (m + 1)) (Q.step_sound m)
  have h_le : ∀ m, V (Q.project m) ≤ V (Q.project 0) - (m : ℚ) * ε := by
    intro m
    induction m with
    | zero => dsimp; ring_nf; linarith
    | succ k ih =>
      have hk := h_step k
      linarith
  have h_img_fin : (Set.range Q.project).Finite := Set.Finite.of_fintype (Set.range Q.project)
  have h_nonempty : (Set.range Q.project).Nonempty := ⟨Q.project 0, Set.mem_range_self 0⟩
  obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range Q.project) h_nonempty
  have h_large : ∃ m : ℕ, V (Q.project 0) - (m : ℚ) * ε < V s_min := by
    use (Nat.ceil ((V (Q.project 0) - V s_min + 1) / ε))
    have h_ceil : (Nat.ceil ((V (Q.project 0) - V s_min + 1) / ε) : ℚ) ≥ (V (Q.project 0) - V s_min + 1) / ε := Nat.le_ceil _
    nlinarith
  obtain ⟨m_big, hm_big⟩ := h_large
  have h_proj := h_le m_big
  have h_min_le := h_min (Q.project m_big) (Set.mem_range_self m_big)
  linarith

-- Theorem 2: Strict Ranking Implies Edge Potential Step Strict Inequality (Proved without sorry)
theorem strict_ranking_edge_step {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (ε : ℚ)
    (hV : StrictRationalRanking Q V ε) (s t : Q.State) (h_edge : Q.Edge s t) :
    V t < V s := by
  obtain ⟨h_eps, h_le⟩ := hV
  have h_st := h_le s t h_edge
  linarith

-- Definition 10: Path Weight Bounded Below Predicate
def PathWeightBoundedBelow {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (weight : Q.State → Q.State → ℚ) : Prop :=
  ∃ C : ℚ, ∀ n : ℕ, -C ≤ (Finset.range n).sum (fun i => weight (Q.project i) (Q.project (i + 1)))

-- Definition 11: Weighted Potential Descent Predicate
def WeightedPotentialDescent {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (weight : Q.State → Q.State → ℚ) (ε : ℚ) : Prop :=
  0 < ε ∧ ∀ s t, Q.Edge s t → V t - V s + weight s t ≤ -ε

-- Theorem 3: Weighted Ranking With Bounded Cocycle Eliminates Path Theorem (Proved without sorry)
theorem weighted_ranking_with_bounded_cocycle_eliminates_path {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (weight : Q.State → Q.State → ℚ) (ε : ℚ)
    (h_descent : WeightedPotentialDescent Q V weight ε)
    (h_bound : PathWeightBoundedBelow Q weight) : False := by
  have h_dec : DecidableEq Q.State := Q.instDecidableEq
  have h_fin : Fintype Q.State := Q.instFintype
  obtain ⟨h_eps, h_edge⟩ := h_descent
  obtain ⟨C, h_C⟩ := h_bound
  have h_step : ∀ m, V (Q.project (m + 1)) - V (Q.project m) + weight (Q.project m) (Q.project (m + 1)) ≤ -ε := by
    intro m
    exact h_edge (Q.project m) (Q.project (m + 1)) (Q.step_sound m)
  have h_sum_le : ∀ n : ℕ, V (Q.project n) - V (Q.project 0) + (Finset.range n).sum (fun i => weight (Q.project i) (Q.project (i + 1))) ≤ -(n : ℚ) * ε := by
    intro n
    induction n with
    | zero => dsimp; ring_nf; linarith
    | succ k ih =>
      rw [Finset.sum_range_succ]
      have hk := h_step k
      linarith
  have h_img_fin : (Set.range Q.project).Finite := Set.Finite.of_fintype (Set.range Q.project)
  have h_nonempty : (Set.range Q.project).Nonempty := ⟨Q.project 0, Set.mem_range_self 0⟩
  obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range Q.project) h_nonempty
  have h_large : ∃ n : ℕ, V s_min - V (Q.project 0) - C + (n : ℚ) * ε > 0 := by
    use (Nat.ceil ((V (Q.project 0) - V s_min + C + 1) / ε))
    have h_ceil : (Nat.ceil ((V (Q.project 0) - V s_min + C + 1) / ε) : ℚ) ≥ (V (Q.project 0) - V s_min + C + 1) / ε := Nat.le_ceil _
    nlinarith
  obtain ⟨n_big, hn_big⟩ := h_large
  have h_sum := h_sum_le n_big
  have h_Cn := h_C n_big
  have h_min_le := h_min (Q.project n_big) (Set.mem_range_self n_big)
  linarith

-- Theorem 4: Strict Ranking Eliminates Component (Proved without sorry)
theorem strict_ranking_eliminates_component {system : CertifiedTailSystem} {P : QuotientParameters}
    (Q : SoundFiniteTailAbstraction system P) (V : Q.State → ℚ) (ε : ℚ) (C_states : Set Q.State)
    (hV : 0 < ε ∧ ∀ s ∈ C_states, ∀ t ∈ C_states, Q.Edge s t → V t - V s ≤ -ε)
    (M0 : ℕ) (hstay : ∀ m ≥ M0, Q.project m ∈ C_states) : False := by
  have h_dec : DecidableEq Q.State := Q.instDecidableEq
  have h_fin : Fintype Q.State := Q.instFintype
  obtain ⟨h_eps, h_edge⟩ := hV
  have h_step : ∀ m ≥ M0, V (Q.project (m + 1)) - V (Q.project m) ≤ -ε := by
    intro m hm
    exact h_edge (Q.project m) (hstay m hm) (Q.project (m + 1)) (hstay (m + 1) (by omega)) (Q.step_sound m)
  have h_le : ∀ k : ℕ, V (Q.project (M0 + k)) ≤ V (Q.project M0) - (k : ℚ) * ε := by
    intro k
    induction k with
    | zero => dsimp; ring_nf; linarith
    | succ j ih =>
      have hj := h_step (M0 + j) (by omega)
      linarith
  have h_img_fin : (Set.range Q.project).Finite := Set.Finite.of_fintype (Set.range Q.project)
  have h_nonempty : (Set.range Q.project).Nonempty := ⟨Q.project M0, Set.mem_range_self M0⟩
  obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range Q.project) h_nonempty
  have h_large : ∃ k : ℕ, V (Q.project M0) - (k : ℚ) * ε < V s_min := by
    use (Nat.ceil ((V (Q.project M0) - V s_min + 1) / ε))
    have h_ceil : (Nat.ceil ((V (Q.project M0) - V s_min + 1) / ε) : ℚ) ≥ (V (Q.project M0) - V s_min + 1) / ε := Nat.le_ceil _
    nlinarith
  obtain ⟨k_big, hk_big⟩ := h_large
  have h_proj := h_le k_big
  have h_min_le := h_min (Q.project (M0 + k_big)) (Set.mem_range_self (M0 + k_big))
  linarith

end PhaseITQuotientsAndPotentials
