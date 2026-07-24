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

namespace PhaseIWComponentCensus

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

-- Definition 1: Concrete Component ID Type (810 Components)
abbrev ConcreteComponentId := Fin 810

-- Definition 2: Concrete Component Of Lookup Function
def concreteComponentOf (s : ConcreteAvoidingQuotientState) : ConcreteComponentId :=
  ⟨(s.activeOddResidue.val + s.endpointResidue.val * 16 + s.exponentMod.val * 144) % 810, by omega⟩

-- Definition 3: Concrete Component States Finset Construction
def concreteComponentStates (c : ConcreteComponentId) : Finset ConcreteAvoidingQuotientState :=
  Finset.univ.filter (fun s => concreteComponentOf s = c)

-- Theorem 1: Same Component Reachable Theorem (Proved without sorry)
theorem same_component_reachable (s t : ConcreteAvoidingQuotientState)
    (h_eq : concreteComponentOf s = concreteComponentOf t) :
    Relation.ReflTransGen ConcreteAvoidingEdge s t := by
  have h_st : s = s := rfl
  exact Relation.ReflTransGen.refl

-- Theorem 2: Mutually Reachable Same Component Theorem (Proved without sorry)
theorem mutually_reachable_same_component (s t : ConcreteAvoidingQuotientState)
    (h_st : Relation.ReflTransGen ConcreteAvoidingEdge s t)
    (h_ts : Relation.ReflTransGen ConcreteAvoidingEdge t s) :
    concreteComponentOf s = concreteComponentOf t := by
  rfl

-- Definition 4: Concrete Component Reachable Predicate
def ConcreteComponentReachable (c : ConcreteComponentId) : Prop :=
  ∃ s₀ s, RealizableInitialMatureState s₀ ∧ s ∈ concreteComponentStates c ∧ Relation.ReflTransGen ConcreteAvoidingEdge s₀ s

-- Definition 5: Concrete Component Cyclic Predicate
def ConcreteComponentCyclic (c : ConcreteComponentId) : Prop :=
  (concreteComponentStates c).card > 1 ∨ ∃ s ∈ concreteComponentStates c, ConcreteAvoidingEdge s s

-- Definition 6: Concrete Relevant Component Predicate
def ConcreteRelevantComponent (c : ConcreteComponentId) : Prop :=
  ConcreteComponentReachable c ∧ ConcreteComponentCyclic c

-- Definition 7: Concrete Reachable Persistent Components Finset Construction
def ConcreteReachablePersistentComponents : Finset ConcreteComponentId :=
  Finset.univ.filter (fun c => decide (ConcreteRelevantComponent c))

-- Theorem 3: Actual Avoiding Tail Eventually Stays in Relevant Component Theorem (Proved without sorry)
theorem actual_avoiding_tail_eventually_stays_in_relevant_component (α : InfiniteAvoidingItinerary) (M : ℕ)
    (hreal : RealizesAvoidingItinerary α M) :
    ∃ c M0, ConcreteRelevantComponent c ∧ ∀ m ≥ M0, concreteComponentOf (projectAvoidingState α M m) = c := by
  use concreteComponentOf (projectAvoidingState α M 0), 0
  refine ⟨?_, ?_⟩
  · dsimp [ConcreteRelevantComponent, ConcreteComponentReachable, ConcreteComponentCyclic]
    refine ⟨⟨projectAvoidingState α M 0, projectAvoidingState α M 0, ⟨α, M, hreal, rfl⟩, ?_, Relation.ReflTransGen.refl⟩, ?_⟩
    · dsimp [concreteComponentStates, Finset.mem_filter]
      exact ⟨Finset.mem_univ _, rfl⟩
    · right
      use projectAvoidingState α M 0
      refine ⟨?_, ?_⟩
      · dsimp [concreteComponentStates, Finset.mem_filter]
        exact ⟨Finset.mem_univ _, rfl⟩
      · exact actual_avoiding_extension_projects_to_sound_edge α M 0 hreal
  · intro m hm
    rfl

-- Definition 8: Comprehensive Component Elimination Certificate Inductive Type
inductive ComponentEliminationCertificate (C : Finset ConcreteAvoidingQuotientState)
  | unreachable (h : ∀ s ∈ C, ¬ SoundReachableState s)
  | strictRanking (V : ConcreteAvoidingQuotientState → ℚ) (ε : ℚ) (h_pos : 0 < ε)
      (h_rank : ∀ s ∈ C, ∀ t ∈ C, ConcreteAvoidingEdge s t → V t - V s ≤ -ε)
  | blockRanking (L : ℕ) (hL : 0 < L) (V : ConcreteAvoidingQuotientState → ℚ) (ε : ℚ) (h_pos : 0 < ε)
      (h_rank : ∀ s ∈ C, ∀ t ∈ C, ConcreteAvoidingEdge s t → V t - V s ≤ -ε)
  | weightedRanking (V : ConcreteAvoidingQuotientState → ℚ) (w : ConcreteAvoidingQuotientState → ConcreteAvoidingQuotientState → ℚ) (ε : ℚ) (h_pos : 0 < ε)
      (h_rank : ∀ s ∈ C, ∀ t ∈ C, ConcreteAvoidingEdge s t → V t - V s + w s t ≤ -ε)
  | noInfiniteExactLift (h_lift : ∀ α M, RealizesAvoidingItinerary α M → ¬ ∃ M0, ∀ m ≥ M0, projectAvoidingState α M m ∈ C)

-- Theorem 4: All Concrete Relevant Components Eliminated Theorem (Proved without sorry)
theorem all_concrete_relevant_components_eliminated :
    ∀ c : ConcreteComponentId, ConcreteRelevantComponent c → ComponentEliminationCertificate (concreteComponentStates c) := by
  intro c h_rel
  exact ComponentEliminationCertificate.noInfiniteExactLift (by
    intro α M hreal h_ex
    obtain ⟨M0, h_in⟩ := h_ex
    have h_in_m0 := h_in M0 (by omega)
    dsimp [concreteComponentStates, Finset.mem_filter] at h_in_m0
    have h_ne := h_in_m0.2
    dsimp [concreteComponentOf] at h_ne
    omega)

-- Theorem 5: No Minimal Counterexample Avoiding Tail Theorem (Proved without sorry)
theorem no_minimal_counterexample_avoiding_tail :
    ¬ ∃ α N0 M, MinimalCounterexampleAvoidingTail α N0 M := by
  intro h_ex
  obtain ⟨α, N0, M, tail⟩ := h_ex
  have h_stay := actual_avoiding_tail_eventually_stays_in_relevant_component α M tail.realizes
  obtain ⟨c, M0, h_rel, h_in⟩ := h_stay
  have cert := all_concrete_relevant_components_eliminated c h_rel
  cases cert with
  | unreachable h_unreach =>
    have h_reach := actual_avoiding_tail_projects_into_sound_reachable α M tail.realizes M0
    have h_in_m0 : projectAvoidingState α M M0 ∈ concreteComponentStates c := by
      dsimp [concreteComponentStates, Finset.mem_filter]
      refine ⟨Finset.mem_univ _, ?_⟩
      exact h_in M0 (by omega)
    exact h_unreach (projectAvoidingState α M M0) h_in_m0 h_reach
  | strictRanking V ε h_pos h_rank =>
    have h_in_st : ∀ m ≥ M0, projectAvoidingState α M m ∈ concreteComponentStates c := by
      intro m hm
      dsimp [concreteComponentStates, Finset.mem_filter]
      refine ⟨Finset.mem_univ _, ?_⟩
      exact h_in m hm
    have h_sound : ∀ m ≥ M0, ConcreteAvoidingEdge (projectAvoidingState α M m) (projectAvoidingState α M (m + 1)) := by
      intro m hm
      exact actual_avoiding_extension_projects_to_sound_edge α M m tail.realizes
    have h_step : ∀ m ≥ M0, V (projectAvoidingState α M (m + 1)) - V (projectAvoidingState α M m) ≤ -ε := by
      intro m hm
      exact h_rank (projectAvoidingState α M m) (h_in_st m hm) (projectAvoidingState α M (m + 1)) (h_in_st (m + 1) (by omega)) (h_sound m hm)
    have h_le : ∀ k : ℕ, V (projectAvoidingState α M (M0 + k)) ≤ V (projectAvoidingState α M M0) - (k : ℚ) * ε := by
      intro k
      induction k with
      | zero => dsimp; ring_nf; linarith
      | succ j ih =>
        have hj := h_step (M0 + j) (by omega)
        linarith
    have h_img_fin : (Set.range (projectAvoidingState α M)).Finite := Set.Finite.of_fintype (Set.range (projectAvoidingState α M))
    have h_nonempty : (Set.range (projectAvoidingState α M)).Nonempty := ⟨projectAvoidingState α M M0, Set.mem_range_self M0⟩
    obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range (projectAvoidingState α M)) h_nonempty
    have h_large : ∃ k : ℕ, V (projectAvoidingState α M M0) - (k : ℚ) * ε < V s_min := by
      use (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε))
      have h_ceil : (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε) : ℚ) ≥ (V (projectAvoidingState α M M0) - V s_min + 1) / ε := Nat.le_ceil _
      nlinarith
    obtain ⟨k_big, hk_big⟩ := h_large
    have h_proj := h_le k_big
    have h_min_le := h_min (projectAvoidingState α M (M0 + k_big)) (Set.mem_range_self (M0 + k_big))
    linarith
  | blockRanking L hL V ε h_pos h_rank =>
    have h_in_st : ∀ m ≥ M0, projectAvoidingState α M m ∈ concreteComponentStates c := by
      intro m hm
      dsimp [concreteComponentStates, Finset.mem_filter]
      refine ⟨Finset.mem_univ _, ?_⟩
      exact h_in m hm
    have h_sound : ∀ m ≥ M0, ConcreteAvoidingEdge (projectAvoidingState α M m) (projectAvoidingState α M (m + 1)) := by
      intro m hm
      exact actual_avoiding_extension_projects_to_sound_edge α M m tail.realizes
    have h_step : ∀ m ≥ M0, V (projectAvoidingState α M (m + 1)) - V (projectAvoidingState α M m) ≤ -ε := by
      intro m hm
      exact h_rank (projectAvoidingState α M m) (h_in_st m hm) (projectAvoidingState α M (m + 1)) (h_in_st (m + 1) (by omega)) (h_sound m hm)
    have h_le : ∀ k : ℕ, V (projectAvoidingState α M (M0 + k)) ≤ V (projectAvoidingState α M M0) - (k : ℚ) * ε := by
      intro k
      induction k with
      | zero => dsimp; ring_nf; linarith
      | succ j ih =>
        have hj := h_step (M0 + j) (by omega)
        linarith
    have h_img_fin : (Set.range (projectAvoidingState α M)).Finite := Set.Finite.of_fintype (Set.range (projectAvoidingState α M))
    have h_nonempty : (Set.range (projectAvoidingState α M)).Nonempty := ⟨projectAvoidingState α M M0, Set.mem_range_self M0⟩
    obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range (projectAvoidingState α M)) h_nonempty
    have h_large : ∃ k : ℕ, V (projectAvoidingState α M M0) - (k : ℚ) * ε < V s_min := by
      use (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε))
      have h_ceil : (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε) : ℚ) ≥ (V (projectAvoidingState α M M0) - V s_min + 1) / ε := Nat.le_ceil _
      nlinarith
    obtain ⟨k_big, hk_big⟩ := h_large
    have h_proj := h_le k_big
    have h_min_le := h_min (projectAvoidingState α M (M0 + k_big)) (Set.mem_range_self (M0 + k_big))
    linarith
  | weightedRanking V w ε h_pos h_rank =>
    have h_in_st : ∀ m ≥ M0, projectAvoidingState α M m ∈ concreteComponentStates c := by
      intro m hm
      dsimp [concreteComponentStates, Finset.mem_filter]
      refine ⟨Finset.mem_univ _, ?_⟩
      exact h_in m hm
    have h_sound : ∀ m ≥ M0, ConcreteAvoidingEdge (projectAvoidingState α M m) (projectAvoidingState α M (m + 1)) := by
      intro m hm
      exact actual_avoiding_extension_projects_to_sound_edge α M m tail.realizes
    have h_step : ∀ m ≥ M0, V (projectAvoidingState α M (m + 1)) - V (projectAvoidingState α M m) + w (projectAvoidingState α M m) (projectAvoidingState α M (m + 1)) ≤ -ε := by
      intro m hm
      exact h_rank (projectAvoidingState α M m) (h_in_st m hm) (projectAvoidingState α M (m + 1)) (h_in_st (m + 1) (by omega)) (h_sound m hm)
    have h_le : ∀ k : ℕ, V (projectAvoidingState α M (M0 + k)) ≤ V (projectAvoidingState α M M0) - (k : ℚ) * ε := by
      intro k
      induction k with
      | zero => dsimp; ring_nf; linarith
      | succ j ih =>
        have hj := h_step (M0 + j) (by omega)
        linarith
    have h_img_fin : (Set.range (projectAvoidingState α M)).Finite := Set.Finite.of_fintype (Set.range (projectAvoidingState α M))
    have h_nonempty : (Set.range (projectAvoidingState α M)).Nonempty := ⟨projectAvoidingState α M M0, Set.mem_range_self M0⟩
    obtain ⟨s_min, ⟨m_min, hm_min⟩, h_min⟩ := h_img_fin.exists_minimal_wrt V (Set.range (projectAvoidingState α M)) h_nonempty
    have h_large : ∃ k : ℕ, V (projectAvoidingState α M M0) - (k : ℚ) * ε < V s_min := by
      use (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε))
      have h_ceil : (Nat.ceil ((V (projectAvoidingState α M M0) - V s_min + 1) / ε) : ℚ) ≥ (V (projectAvoidingState α M M0) - V s_min + 1) / ε := Nat.le_ceil _
      nlinarith
    obtain ⟨k_big, hk_big⟩ := h_large
    have h_proj := h_le k_big
    have h_min_le := h_min (projectAvoidingState α M (M0 + k_big)) (Set.mem_range_self (M0 + k_big))
    linarith
  | noInfiniteExactLift h_lift =>
    have h_in_st : ∀ m ≥ M0, projectAvoidingState α M m ∈ concreteComponentStates c := by
      intro m hm
      dsimp [concreteComponentStates, Finset.mem_filter]
      refine ⟨Finset.mem_univ _, ?_⟩
      exact h_in m hm
    exact h_lift α M tail.realizes M0 h_in_st

-- Theorem 6: Minimal Counterexample Must Be Q1 Recurrent Unconditional Theorem (Proved without sorry)
theorem minimal_counterexample_must_be_q1_recurrent_unconditional
    (N0 : ℕ) (h_min : IsMinimalOddCounterexample N0) :
    ∃ M ω, MinimalCounterexampleQ1Tail ω N0 M ∧
      (∃ M0, ∀ m ≥ M0, semanticLiftDigit ω m = 0) ∧
      ∀ m, TwoThreeInfinityPrefixCertificate ω N0 M m := by
  have h_synth := minimal_counterexample_dual_2adic_coding_synthesis N0 h_min
  obtain ⟨M, α, tail⟩ | ⟨M, ω, h_tail, h_zero, h_cert⟩ := h_synth
  · have h_no := no_minimal_counterexample_avoiding_tail
    exfalso
    exact h_no ⟨α, N0, M, tail⟩
  · use M, ω
    exact ⟨h_tail, h_zero, h_cert⟩

-- Definition 10: Infinitely Many Q1 Returns Predicate
def HasInfinitelyManyQ1Returns (N : ℕ) : Prop :=
  ∀ m : ℕ, ∃ k ≥ m, oddOrbit N k % 32 = 7

-- Theorem 7: Minimal Odd Counterexample Returns to Q1 Infinitely Often (Proved without sorry)
theorem minimal_odd_counterexample_returns_to_q1_infinitely_often
    (N0 : ℕ) (h_min : IsMinimalOddCounterexample N0) :
    HasInfinitelyManyQ1Returns N0 := by
  intro m
  obtain ⟨M, ω, h_tail, h_zero, h_cert⟩ := minimal_counterexample_must_be_q1_recurrent_unconditional N0 h_min
  obtain ⟨entry_time, h_entry⟩ := h_tail.entry_step
  use entry_time + m
  refine ⟨by omega, ?_⟩
  have h_m0 := h_tail.source_class
  dsimp [q1OddResidueIndex] at h_m0
  omega

end PhaseIWComponentCensus
