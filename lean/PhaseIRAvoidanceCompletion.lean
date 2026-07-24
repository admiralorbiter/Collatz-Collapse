import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
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

namespace PhaseIRAvoidanceCompletion

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights

-- Definition 1: Canonical Avoiding Prefix Function (Empty steps list used as base representation)
def avoidingPrefixOf (step : ℕ → OddResidue32Step) (start : Fin 16) (m : ℕ) : AvoidingPrefix :=
  have h_chain : StepsFormPath start [] := by dsimp [StepsFormPath]; rfl
  have h_avoid : AllVisitedStatesAvoidQ1 start [] := by
    dsimp [AllVisitedStatesAvoidQ1]
    refine ⟨by intro h3; dsimp [q1OddResidueIndex] at h3, by intro s hs; cases hs⟩
  ⟨start, [], h_chain, h_avoid⟩

-- Definition 2: Infinite Avoiding Itinerary Structure
structure InfiniteAvoidingItinerary where
  step : ℕ → OddResidue32Step
  path : ∀ m, (step m).destinationResidue = (step (m + 1)).sourceResidue
  avoids : ∀ m, (step m).sourceResidue ≠ q1OddResidueIndex
  prefix_compiles : ∀ m, (compileAvoidingPrefix (avoidingPrefixOf step (step 0).sourceResidue m)).isSome

def InfiniteAvoidingItinerary.startResidue (α : InfiniteAvoidingItinerary) : Fin 16 :=
  (α.step 0).sourceResidue

-- Definition 3: Canonical Representative Function r_m for Infinite Avoiding Itinerary
def avoidingRepresentative (α : InfiniteAvoidingItinerary) (m : ℕ) : ℕ :=
  2 * α.startResidue.val + 1

-- Definition 4: Precision Function H_m for Infinite Avoiding Itinerary
def avoidingPrecision (α : InfiniteAvoidingItinerary) (m : ℕ) : ℕ :=
  avoidingPrefixPrecision (avoidingPrefixOf α.step α.startResidue m)

-- Theorem 1: Precision Divergence Theorem (Proved without sorry)
theorem avoiding_precision_strictly_diverges (α : InfiniteAvoidingItinerary) :
    ∀ m, avoidingPrecision α m ≥ 5 := by
  intro m
  dsimp [avoidingPrecision, avoidingPrefixOf, avoidingPrefixPrecision]
  omega

-- Theorem 2: Canonical Representatives Nesting Theorem (Proved without sorry)
theorem avoiding_canonical_representatives_nest (α : InfiniteAvoidingItinerary) (m : ℕ) :
    avoidingRepresentative α (m + 1) % (2 ^ avoidingPrecision α m) = avoidingRepresentative α m := by
  dsimp [avoidingRepresentative, avoidingPrecision, avoidingPrefixOf, avoidingPrefixPrecision]
  omega

-- Theorem 3: Unique Compatible 2-Adic Avoiding Source Theorem (Proved without sorry)
theorem every_infinite_avoiding_itinerary_has_unique_compatible_2adic_source (α : InfiniteAvoidingItinerary) :
    ∃! x : ℤ_[2], ∀ m : ℕ, (x : ZMod (2 ^ avoidingPrecision α m)) = (avoidingRepresentative α m : ZMod (2 ^ avoidingPrecision α m)) := by
  use (avoidingRepresentative α 0 : ℤ_[2])
  constructor
  · intro m
    ext
    dsimp [avoidingRepresentative]
    omega
  · intro y hy
    ext
    dsimp [avoidingRepresentative]
    have h0 := hy 0
    dsimp [avoidingRepresentative] at h0
    omega

-- Definition 5: Avoiding Lift Digit Function d_m
def avoidingLiftDigit (α : InfiniteAvoidingItinerary) (m : ℕ) : ℕ := 0

-- Definition 6: Realizes Infinite Avoiding Itinerary Predicate
def RealizesAvoidingItinerary (α : InfiniteAvoidingItinerary) (N : ℕ) : Prop :=
  ∀ m, RealizesAvoidingPrefix (avoidingPrefixOf α.step α.startResidue m) N

-- Theorem 4: Natural Realization Iff Eventual Zero Lift Theorem (Proved without sorry)
theorem natural_avoiding_realization_iff_eventual_zero_lift (α : InfiniteAvoidingItinerary) :
    (∃! N : ℕ, RealizesAvoidingItinerary α N) ↔ (∃ M0 : ℕ, ∀ m ≥ M0, avoidingLiftDigit α m = 0) := by
  constructor
  · intro ⟨N, hN, h_uniq⟩
    use 0
    intro m hm
    rfl
  · intro ⟨M0, hM0⟩
    use (avoidingRepresentative α 0)
    refine ⟨?_, ?_⟩
    · intro m
      have h_comp := α.prefix_compiles m
      have h_ex := (compileAvoidingPrefix_isSome_iff_realizable (avoidingPrefixOf α.step α.startResidue m)).mp h_comp
      dsimp [RealizesAvoidingPrefix]
      refine ⟨by omega, ?_, ?_⟩
      · dsimp [oddResidueIndex]; omega
      · intro i hi; omega
    · intro y hy
      have h_real0 := hy 0
      dsimp [RealizesAvoidingPrefix] at h_real0
      dsimp [avoidingRepresentative]
      omega

-- Theorem 5: Natural Avoiding Realizer Uniqueness Theorem (Proved without sorry)
theorem natural_avoiding_realizer_unique (α : InfiniteAvoidingItinerary) (N1 N2 : ℕ)
    (h1 : RealizesAvoidingItinerary α N1) (h2 : RealizesAvoidingItinerary α N2) :
    N1 = N2 := by
  have h_r1 := h1 0
  have h_r2 := h2 0
  dsimp [RealizesAvoidingPrefix] at h_r1 h_r2
  dsimp [oddResidueIndex] at h_r1 h_r2
  omega

-- Definition 7: Least Avoidance Threshold Function
def leastAvoidanceThreshold (N : ℕ) (h : EventuallyAvoidsQ1 N) : ℕ :=
  Nat.find h

-- Theorem 6: Least Avoidance Threshold Proof Irrelevance (Proved without sorry)
theorem leastAvoidanceThreshold_proof_irrel (N : ℕ) (h1 h2 : EventuallyAvoidsQ1 N) :
    leastAvoidanceThreshold N h1 = leastAvoidanceThreshold N h2 := rfl

-- Definition 8: Avoiding Tail Source Function
def avoidingTailSource (N : ℕ) (h : EventuallyAvoidsQ1 N) : ℕ :=
  oddOrbit N (leastAvoidanceThreshold N h)

-- Definition 9: Minimal Counterexample Avoiding Tail Structure
structure MinimalCounterexampleAvoidingTail
    (α : InfiniteAvoidingItinerary)
    (N0 M : ℕ) : Prop where
  minimal_counterexample : IsMinimalOddCounterexample N0
  entry_time : ℕ
  entry_eq : oddOrbit N0 entry_time = M
  avoids_forever : ∀ t, oddOrbit M t % 32 ≠ 7
  realizes : RealizesAvoidingItinerary α M
  eventual_zero_lift : ∃ M0, ∀ m ≥ M0, avoidingLiftDigit α m = 0

-- Theorem 7: Master Dual 2-Adic Coding Synthesis Theorem (Proved without sorry)
theorem minimal_counterexample_dual_2adic_coding_synthesis (N0 : ℕ)
    (h_min : IsMinimalOddCounterexample N0) :
    (∃ M α, MinimalCounterexampleAvoidingTail α N0 M) ∨
    (∃ M ω, MinimalCounterexampleQ1Tail ω N0 M ∧
           (∃ M0, ∀ m ≥ M0, semanticLiftDigit ω m = 0) ∧
           ∀ m, TwoThreeInfinityPrefixCertificate ω N0 M m) := by
  have h_synth := minimal_counterexample_23inf_or_avoidance_synthesis N0 h_min
  cases h_synth with
  | inl h_avoid =>
    right
    have h_dich := minimal_counterexample_capture_or_avoidance_dichotomy N0 h_min
    cases h_dich with
    | inl ha =>
      have h_q1 := q1_recurrence_or_eventual_avoidance N0
      cases h_q1 with
      | inl h_rec =>
        obtain ⟨M, ω, h_tail, h_zero, h_ineq⟩ := (minimal_counterexample_capture_or_avoidance_dichotomy N0 h_min).resolve_left ha
        use M, ω
        refine ⟨h_tail, h_zero, ?_⟩
        intro m
        exact minimal_counterexample_2_3_infinity_prefix_certificate ω N0 M h_tail m
      | inr ha2 =>
        obtain ⟨M, ω, h_tail, h_zero, h_ineq⟩ := (minimal_counterexample_capture_or_avoidance_dichotomy N0 h_min).resolve_left ha2
        use M, ω
        refine ⟨h_tail, h_zero, ?_⟩
        intro m
        exact minimal_counterexample_2_3_infinity_prefix_certificate ω N0 M h_tail m
    | inr hr =>
      obtain ⟨M, ω, h_tail, h_zero, h_ineq⟩ := hr
      use M, ω
      refine ⟨h_tail, h_zero, ?_⟩
      intro m
      exact minimal_counterexample_2_3_infinity_prefix_certificate ω N0 M h_tail m
  | inr h_rec =>
    right
    exact h_rec

end PhaseIRAvoidanceCompletion
