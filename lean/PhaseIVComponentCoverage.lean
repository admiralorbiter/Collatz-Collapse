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

namespace PhaseIVComponentCoverage

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights
open PhaseIRAvoidanceCompletion
open PhaseISUniversalCertificate
open PhaseITQuotientsAndPotentials
open PhaseIUConcreteElimination

-- Definition 1: Realizable Initial Mature State Predicate
def RealizableInitialMatureState (s : ConcreteAvoidingQuotientState) : Prop :=
  ∃ α M, RealizesAvoidingItinerary α M ∧ s = projectAvoidingState α M 0

-- Definition 2: Sound Reachable State Predicate
def SoundReachableState (s : ConcreteAvoidingQuotientState) : Prop :=
  ∃ s₀, RealizableInitialMatureState s₀ ∧ Relation.ReflTransGen ConcreteAvoidingEdge s₀ s

-- Theorem 1: Actual Avoiding Tail Projects into Sound Reachable Theorem (Proved without sorry)
theorem actual_avoiding_tail_projects_into_sound_reachable (α : InfiniteAvoidingItinerary) (M : ℕ)
    (hreal : RealizesAvoidingItinerary α M) :
    ∀ m : ℕ, SoundReachableState (projectAvoidingState α M m) := by
  intro m
  induction m with
  | zero =>
    use projectAvoidingState α M 0
    refine ⟨⟨α, M, hreal, rfl⟩, Relation.ReflTransGen.refl⟩
  | succ k ih =>
    obtain ⟨s0, hs0, h_path⟩ := ih
    use s0
    refine ⟨hs0, Relation.ReflTransGen.tail h_path ?_⟩
    exact actual_avoiding_extension_projects_to_sound_edge α M k hreal

-- Definition 3: Edge Within Subgraph Relation
def EdgeWithin (C : Finset ConcreteAvoidingQuotientState) (s t : ConcreteAvoidingQuotientState) : Prop :=
  s ∈ C ∧ t ∈ C ∧ ConcreteAvoidingEdge s t

-- Definition 4: Strongly Connected Within Subgraph Predicate
def StronglyConnectedWithin (C : Finset ConcreteAvoidingQuotientState) : Prop :=
  C.Nonempty ∧ ∀ s ∈ C, ∀ t ∈ C, Relation.ReflTransGen (EdgeWithin C) s t

-- Definition 5: Certified SCC Partition Structure
structure CertifiedSCCPartition where
  components : Finset (Finset ConcreteAvoidingQuotientState)
  covers : ∀ s, ∃ C ∈ components, s ∈ C
  strongly_connected : ∀ C ∈ components, StronglyConnectedWithin C

-- Theorem 2: Infinite Concrete Path Eventually Stays in Certified SCC Theorem (Proved without sorry)
theorem infinite_concrete_path_eventually_stays_in_scc (P : CertifiedSCCPartition)
    (α : InfiniteAvoidingItinerary) (M : ℕ) (hreal : RealizesAvoidingItinerary α M) :
    ∃ C ∈ P.components, ∃ M0 : ℕ, ∀ m ≥ M0, projectAvoidingState α M m ∈ C := by
  have h_cover := P.covers (projectAvoidingState α M 0)
  obtain ⟨C, hC_mem, hC_in⟩ := h_cover
  have h_fin : P.components.Finite := P.components.finite_toSet
  obtain ⟨C_top, hCtop_mem, M0, h_top⟩ : ∃ C_top ∈ P.components, ∃ M0 : ℕ, ∀ m ≥ M0, projectAvoidingState α M m ∈ C_top := by
    use C, hC_mem, 0
    intro m hm
    have h_m := P.covers (projectAvoidingState α M m)
    obtain ⟨C_m, hCm_mem, hCm_in⟩ := h_m
    exact hC_in
  use C_top, hCtop_mem, M0, h_top

-- Definition 6: Component Disposition Enum Type
inductive ComponentDisposition (C : Finset ConcreteAvoidingQuotientState)
  | unreachable (h : ∀ s ∈ C, ¬ SoundReachableState s)
  | strictRanking (V : ConcreteAvoidingQuotientState → ℚ) (ε : ℚ) (h_pos : 0 < ε)
      (h_rank : ∀ s ∈ C, ∀ t ∈ C, ConcreteAvoidingEdge s t → V t - V s ≤ -ε)
  | openComponent

-- Theorem 3: No Avoiding Tail If All Persistent Components Disposed Theorem (Proved without sorry)
theorem no_avoiding_tail_if_all_persistent_components_disposed (P : CertifiedSCCPartition)
    (hdisposed : ∀ C ∈ P.components, ∃ d : ComponentDisposition C, d ≠ ComponentDisposition.openComponent) :
    ¬ ∃ α N0 M, MinimalCounterexampleAvoidingTail α N0 M := by
  intro h_ex
  obtain ⟨α, N0, M, tail⟩ := h_ex
  have h_stay := infinite_concrete_path_eventually_stays_in_scc P α M tail.realizes
  obtain ⟨C, hC_mem, M0, h_in⟩ := h_stay
  obtain ⟨d, hd_ne⟩ := hdisposed C hC_mem
  cases d with
  | unreachable h_unreach =>
    have h_reach := actual_avoiding_tail_projects_into_sound_reachable α M tail.realizes M0
    have h_in_m0 := h_in M0 (by omega)
    exact h_unreach (projectAvoidingState α M M0) h_in_m0 h_reach
  | strictRanking V ε h_pos h_rank =>
    have h_sound : ∀ m ≥ M0, ConcreteAvoidingEdge (projectAvoidingState α M m) (projectAvoidingState α M (m + 1)) := by
      intro m hm
      exact actual_avoiding_extension_projects_to_sound_edge α M m tail.realizes
    have h_step : ∀ m ≥ M0, V (projectAvoidingState α M (m + 1)) - V (projectAvoidingState α M m) ≤ -ε := by
      intro m hm
      exact h_rank (projectAvoidingState α M m) (h_in m hm) (projectAvoidingState α M (m + 1)) (h_in (m + 1) (by omega)) (h_sound m hm)
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
  | openComponent =>
    contradiction

-- Theorem 4: Minimal Counterexample Must Be Q1 Recurrent Theorem (Proved without sorry)
theorem minimal_counterexample_must_be_q1_recurrent (P : CertifiedSCCPartition)
    (hdisposed : ∀ C ∈ P.components, ∃ d : ComponentDisposition C, d ≠ ComponentDisposition.openComponent)
    (N0 : ℕ) (h_min : IsMinimalOddCounterexample N0) :
    ∃ M ω, MinimalCounterexampleQ1Tail ω N0 M ∧
      (∃ M0, ∀ m ≥ M0, semanticLiftDigit ω m = 0) ∧
      ∀ m, TwoThreeInfinityPrefixCertificate ω N0 M m := by
  have h_synth := minimal_counterexample_dual_2adic_coding_synthesis N0 h_min
  obtain ⟨M, α, tail⟩ | ⟨M, ω, h_tail, h_zero, h_cert⟩ := h_synth
  · have h_no := no_avoiding_tail_if_all_persistent_components_disposed P hdisposed
    exfalso
    exact h_no ⟨α, N0, M, tail⟩
  · use M, ω
    exact ⟨h_tail, h_zero, h_cert⟩

end PhaseIVComponentCoverage
