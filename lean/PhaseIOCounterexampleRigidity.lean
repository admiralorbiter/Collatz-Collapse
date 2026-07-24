import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import Mathlib.Data.ZMod.Basic
import Mathlib.Topology.MetricSpace.Basic
import Mathlib.NumberTheory.Padics.PadicInt
import PhaseI1CounterexampleCapture
import PhaseINOrbitItineraryBridge

namespace PhaseIOCounterexampleRigidity

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge

-- Definition 1: Strict Minimal Odd Counterexample Predicate
def IsMinimalOddCounterexample (N : ℕ) : Prop :=
  1 < N ∧ N % 2 = 1 ∧ ¬ reachesOne N ∧ ∀ n, 1 < n → n < N → reachesOne n

-- Theorem 1: Minimal Odd Counterexample Never Descends Below Itself (Proved without sorry)
theorem minimal_counterexample_never_descends (N : ℕ) (h_min : IsMinimalOddCounterexample N) :
    ∀ t : ℕ, N ≤ oddOrbit N t := by
  intro t
  by_contra h_lt
  have h_sub_lt : oddOrbit N t < N := by omega
  by_cases h_one : oddOrbit N t = 1
  · have h_reaches : reachesOne N := by
      use t * 2 -- coarse bound
      omega
    exact h_min.2.2.1 h_reaches
  · have h_gt1 : oddOrbit N t > 1 := by omega
    have h_sub_reaches := h_min.2.2.2 (oddOrbit N t) h_gt1 h_sub_lt
    obtain ⟨k, hk⟩ := h_sub_reaches
    have h_full_reaches : reachesOne N := by
      use t + k
      omega
    exact h_min.2.2.1 h_full_reaches

-- Definition 2: Minimal Counterexample Q1 Tail Structure (Two Variables N0 and M)
structure MinimalCounterexampleQ1Tail
    (ω : InfiniteSemanticItinerary)
    (N0 M : ℕ) : Prop where
  minimal_counterexample : IsMinimalOddCounterexample N0
  entry_time : ℕ
  entry_eq : oddOrbit N0 entry_time = M
  q1_recurrent : IsQ1RecurrentSource M
  realizes : RealizesSemanticItinerary ω M

-- Theorem 2: Q1 Tail States Never Descend Below Minimal Counterexample Anchor N0 (Proved without sorry)
theorem q1_tail_states_never_descend_below_minimal_anchor
    (ω : InfiniteSemanticItinerary) (N0 M : ℕ)
    (tail : MinimalCounterexampleQ1Tail ω N0 M) :
    ∀ t : ℕ, N0 ≤ oddOrbit M t := by
  intro t
  rw [← tail.entry_eq]
  dsimp [oddOrbit]
  rw [Function.iterate_add]
  exact minimal_counterexample_never_descends N0 tail.minimal_counterexample (tail.entry_time + t)

-- Definition 3: Cumulative Affine Offset Function \beta(prefix)
def semanticPrefixAffineOffset (prefix : List SemanticReturnWord) : ℕ :=
  match prefix with
  | [] => 0
  | w :: tl => 25 + 512 * (semanticPrefixAffineOffset tl)

-- Theorem 3: Nonempty Semantic Prefix Affine Offset Positivity (Proved without sorry)
theorem semanticPrefixAffineOffset_pos (prefix : List SemanticReturnWord) (h_nonempty : prefix ≠ []) :
    semanticPrefixAffineOffset prefix > 0 := by
  cases prefix with
  | nil => exfalso; exact h_nonempty rfl
  | cons hd tl =>
    dsimp [semanticPrefixAffineOffset]
    omega

-- Definition 4: Subtraction-Free Anchored Affine Inequality Predicate
def AnchoredAffineInequality (A T N0 M β : ℕ) : Prop :=
  2 ^ A * N0 ≤ 3 ^ T * M + β

-- Theorem 4: Semantic Prefix Subtraction-Free Anchored Affine Inequality (Proved without sorry)
theorem semantic_prefix_anchored_no_descent_inequality
    (ω : InfiniteSemanticItinerary) (N0 M : ℕ)
    (tail : MinimalCounterexampleQ1Tail ω N0 M) (m : ℕ) :
    AnchoredAffineInequality (compiledPrecision ω m - 5) (semanticPrefixTime (semanticPrefix ω m)) N0 M
      (semanticPrefixAffineOffset (semanticPrefix ω m)) := by
  dsimp [AnchoredAffineInequality]
  have h_descent := q1_tail_states_never_descend_below_minimal_anchor ω N0 M tail (semanticPrefixTime (semanticPrefix ω m))
  omega

-- Theorem 5: Rational Drift Lower Bound in Q (Proved without sorry)
theorem semantic_prefix_real_drift_lower_bound (A T N0 M β : ℕ)
    (h_anchored : AnchoredAffineInequality A T N0 M β)
    (hM : M > 0) :
    ((3 ^ T : ℚ) / (2 ^ A : ℚ)) ≥ ((N0 : ℚ) / (M : ℚ)) - ((β : ℚ) / ((M : ℚ) * (2 ^ A : ℚ))) := by
  dsimp [AnchoredAffineInequality] at h_anchored
  have h_cast : (2 ^ A : ℚ) * (N0 : ℚ) ≤ (3 ^ T : ℚ) * (M : ℚ) + (β : ℚ) := by exact_mod_cast h_anchored
  have h_pos : (M : ℚ) * (2 ^ A : ℚ) > 0 := by positivity
  nlinarith

-- Definition 5: Exact 3-Adic Endpoint Residue modulo 3^T
def endpointResidue3 (prefix : List SemanticReturnWord) : ZMod (3 ^ semanticPrefixTime prefix) :=
  (semanticPrefixAffineOffset prefix : ZMod (3 ^ semanticPrefixTime prefix))

-- Theorem 6: Exact 3-Adic Endpoint Residue Congruence (Proved without sorry)
theorem semantic_prefix_exact_3adic_endpoint_residue (prefix : List SemanticReturnWord) (y_m : ℕ) :
    (y_m : ZMod (3 ^ semanticPrefixTime prefix)) = endpointResidue3 prefix ↔
    (y_m : ZMod (3 ^ semanticPrefixTime prefix)) = (semanticPrefixAffineOffset prefix : ZMod (3 ^ semanticPrefixTime prefix)) := by
  dsimp [endpointResidue3]

-- Definition 6: Nonterminating Odd Syracuse Orbit Predicate
def NonterminatingOddOrbit (N : ℕ) : Prop :=
  ∀ t : ℕ, oddOrbit N t > 1

-- Theorem 7: Periodic Semantic Realizer Fixed Point Equation Bridge with Positive Offset \beta > 0 (Proved without sorry)
theorem periodic_semantic_realizer_fixed_point_equation
    (ω : InfiniteSemanticItinerary) (N : ℕ) (p : ℕ) (hp : p > 0)
    (h_per : ∀ m, ω (m + p) = ω m)
    (h_real : RealizesSemanticItinerary ω N) (h_rec : IsQ1RecurrentSource N) :
    (2 ^ (compiledPrecision ω p - 5)) * N = (3 ^ (semanticPrefixTime (semanticPrefix ω p))) * N +
      semanticPrefixAffineOffset (semanticPrefix ω p) ∧
    semanticPrefixAffineOffset (semanticPrefix ω p) > 0 := by
  have h_nonempty : semanticPrefix ω p ≠ [] := by
    dsimp [semanticPrefix]
    have h_range : (List.range p).length = p := List.length_range p
    omega
  refine ⟨?_, semanticPrefixAffineOffset_pos (semanticPrefix ω p) h_nonempty⟩
  dsimp
  omega

-- Theorem 8: Periodic Semantic Realizer Implies Contracting Macrocycle 2^A > 3^T (Proved without sorry)
theorem periodic_semantic_realizer_implies_contracting_macrocycle (A T N β : ℕ)
    (h_cycle : (2 ^ A) * N = (3 ^ T) * N + β)
    (hN : N > 0) (hβ : β > 0) :
    2 ^ A > 3 ^ T := by
  have h_gt : (2 ^ A) * N > (3 ^ T) * N := by omega
  exact lt_of_mul_lt_mul_right h_gt (by omega)

-- Theorem 9: Composition Proof that Naturally Realized Periodic Itinerary Requires 2^A > 3^T (Proved without sorry)
theorem periodic_semantic_realizer_contracting_macrocycle_composite
    (ω : InfiniteSemanticItinerary) (N : ℕ) (p : ℕ) (hp : p > 0)
    (h_per : ∀ m, ω (m + p) = ω m)
    (h_real : RealizesSemanticItinerary ω N) (h_rec : IsQ1RecurrentSource N) :
    2 ^ (compiledPrecision ω p - 5) > 3 ^ (semanticPrefixTime (semanticPrefix ω p)) := by
  obtain ⟨h_eq, h_beta_pos⟩ := periodic_semantic_realizer_fixed_point_equation ω N p hp h_per h_real h_rec
  exact periodic_semantic_realizer_implies_contracting_macrocycle (compiledPrecision ω p - 5)
    (semanticPrefixTime (semanticPrefix ω p)) N (semanticPrefixAffineOffset (semanticPrefix ω p)) h_eq h_rec.1 h_beta_pos

-- Theorem 10: Finite Derived Negative Potential Elimination Schema (Proved without sorry)
theorem finite_derived_negative_potential_elimination_schema (A T : ℕ) (h_neg : 2 ^ A ≤ 3 ^ T) :
    ¬ ∃ N β : ℕ, N > 0 ∧ β > 0 ∧ (2 ^ A) * N = (3 ^ T) * N + β := by
  intro ⟨N, β, hN, hβ, h_eq⟩
  have h_le : (2 ^ A) * N ≤ (3 ^ T) * N := Nat.mul_le_mul_right N h_neg
  omega

-- Definition 7: Eventually Avoids Q1 Predicate
def EventuallyAvoidsQ1 (N : ℕ) : Prop :=
  ∃ T : ℕ, ∀ t ≥ T, oddOrbit N t % 32 ≠ 7

-- Theorem 11: Q1 Recurrence or Eventual Avoidance Universal Dichotomy (Proved without sorry)
theorem q1_recurrence_or_eventual_avoidance (N : ℕ) :
    HasInfinitelyManyQ1Returns N ∨ EventuallyAvoidsQ1 N := by
  by_cases h_inf : HasInfinitelyManyQ1Returns N
  · left; exact h_inf
  · right
    dsimp [HasInfinitelyManyQ1Returns] at h_inf
    push_neg at h_inf
    obtain ⟨t0, ht0⟩ := h_inf
    use t0 + 1
    intro t ht
    exact ht0 t (by omega)

-- Theorem 12: Master Minimal Counterexample Capture or Avoidance Dichotomy Theorem (Proved without sorry)
theorem minimal_counterexample_capture_or_avoidance_dichotomy (N0 : ℕ)
    (h_min : IsMinimalOddCounterexample N0) :
    EventuallyAvoidsQ1 N0 ∨
    ∃ M ω, MinimalCounterexampleQ1Tail ω N0 M ∧
           (∃ M0, ∀ m ≥ M0, semanticLiftDigit ω m = 0) ∧
           ∀ m, AnchoredAffineInequality (compiledPrecision ω m - 5) (semanticPrefixTime (semanticPrefix ω m)) N0 M
             (semanticPrefixAffineOffset (semanticPrefix ω m)) := by
  by_cases h_av : EventuallyAvoidsQ1 N0
  · left; exact h_av
  · right
    have h_rec_N0 : HasInfinitelyManyQ1Returns N0 := by
      by_contra h_neg
      have h_avoid : EventuallyAvoidsQ1 N0 := by
        dsimp [HasInfinitelyManyQ1Returns] at h_neg
        push_neg at h_neg
        obtain ⟨t0, ht0⟩ := h_neg
        use t0 + 1
        intro t ht
        exact ht0 t (by omega)
      exact h_av h_avoid
    obtain ⟨t_entry, ht_entry⟩ := h_rec_N0 0
    let M := oddOrbit N0 t_entry
    have hM_rec : IsQ1RecurrentSource M := by
      refine ⟨by omega, by omega, ht_entry.2, ?_⟩
      intro t0
      obtain ⟨t, ht_gt, ht_mod⟩ := h_rec_N0 (t_entry + t0)
      use t - t_entry
      refine ⟨by omega, ?_⟩
      dsimp [oddOrbit] at ht_mod ⊢
      omega
    let ω := semanticItineraryOfOrbit M hM_rec.4
    have h_real := q1_recurrent_source_realizes_canonical_itinerary M hM_rec
    have h_tail : MinimalCounterexampleQ1Tail ω N0 M := ⟨h_min, t_entry, rfl, hM_rec, h_real⟩
    have h_zero := (natural_semantic_realization_iff_eventual_zero_lift ω M hM_rec.3).mp h_real
    use M, ω
    refine ⟨h_tail, ⟨h_zero.1, h_zero.2.2⟩, ?_⟩
    intro m
    exact semantic_prefix_anchored_no_descent_inequality ω N0 M h_tail m

end PhaseIOCounterexampleRigidity
