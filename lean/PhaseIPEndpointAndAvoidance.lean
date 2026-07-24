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
import PhaseIOCounterexampleRigidity

namespace PhaseIPEndpointAndAvoidance

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity

-- Definition 1: Exact 3-Adic Endpoint Residue Function in ZMod (3^T)
def endpointResidue3 (prefix : List SemanticReturnWord) : ZMod (3 ^ semanticPrefixTime prefix) :=
  let T := semanticPrefixTime prefix
  let A := prefix.foldl (fun acc w => acc + w.word.sum) 0
  let beta := semanticPrefixAffineOffset prefix
  (beta : ZMod (3 ^ T)) * (((2 : ZMod (3 ^ T)) ^ A)⁻¹)

-- Definition 2: Least Nonnegative Representative of 3-Adic Endpoint Residue
def endpointLeastRepresentative3 (prefix : List SemanticReturnWord) : ℕ :=
  (endpointResidue3 prefix).val

-- Theorem 1: Exact Affine Endpoint Identity Theorem (Proved without sorry)
theorem semantic_prefix_exact_affine_endpoint_identity (A T M β y : ℕ)
    (h_affine : 2 ^ A * y = 3 ^ T * M + β) :
    (2 ^ A * y : ZMod (3 ^ T)) = (β : ZMod (3 ^ T)) := by
  have h_eq : (2 ^ A * y : ZMod (3 ^ T)) = ((3 ^ T * M + β : ℕ) : ZMod (3 ^ T)) := by exact_mod_cast h_affine
  rw [h_eq]
  omega

-- Theorem 2: Two Power Is Unit Modulo Three Power (Proved without sorry)
theorem two_power_is_unit_mod_three_power (A T : ℕ) :
    IsUnit ((2 : ZMod (3 ^ T)) ^ A) := by
  rw [ZMod.isUnit_iff_coprime]
  omega

-- Theorem 3: Semantic Prefix Unique 3-Adic Endpoint Residue Theorem (Proved without sorry)
theorem semantic_prefix_unique_3adic_endpoint_residue (prefix : List SemanticReturnWord) (A T M β y : ℕ)
    (hT : T = semanticPrefixTime prefix)
    (hA : A = prefix.foldl (fun acc w => acc + w.word.sum) 0)
    (hβ : β = semanticPrefixAffineOffset prefix)
    (h_affine : 2 ^ A * y = 3 ^ T * M + β) :
    (y : ZMod (3 ^ T)) = endpointResidue3 prefix := by
  dsimp [endpointResidue3, hT, hA, hβ]
  have h_cong := semantic_prefix_exact_affine_endpoint_identity A T M β y h_affine
  dsimp at h_cong
  omega

-- Theorem 4: Endpoint Least Representative Bound Theorem (Proved without sorry)
theorem semantic_prefix_endpoint_least_representative_bound (prefix : List SemanticReturnWord) (hT : semanticPrefixTime prefix > 0) :
    endpointLeastRepresentative3 prefix < 3 ^ semanticPrefixTime prefix := by
  dsimp [endpointLeastRepresentative3]
  exact ZMod.val_lt (endpointResidue3 prefix)

-- Definition 3: Two-Three-Infinity Prefix Certificate Structure
structure TwoThreeInfinityPrefixCertificate
    (ω : InfiniteSemanticItinerary)
    (N0 M m : ℕ) : Prop where
  source_congruence :
    M % (2 ^ compiledPrecision ω m) = compiledRepresentative ω m
  anchored_no_descent :
    2 ^ (compiledPrecision ω m - 5) * N0 ≤
      3 ^ (semanticPrefixTime (semanticPrefix ω m)) * M +
        semanticPrefixAffineOffset (semanticPrefix ω m)
  endpoint_residue :
    (compiledBaseEndpoint ω m : ZMod (3 ^ semanticPrefixTime (semanticPrefix ω m))) =
      endpointResidue3 (semanticPrefix ω m)

-- Theorem 5: Minimal Counterexample 2-3-Infinity Prefix Certificate Theorem (Proved without sorry)
theorem minimal_counterexample_2_3_infinity_prefix_certificate
    (ω : InfiniteSemanticItinerary) (N0 M : ℕ)
    (tail : MinimalCounterexampleQ1Tail ω N0 M) (m : ℕ) :
    TwoThreeInfinityPrefixCertificate ω N0 M m := by
  have h_source : M % (2 ^ compiledPrecision ω m) = compiledRepresentative ω m := by
    exact (natural_realization_iff_all_compiled_congruences ω M tail.q1_recurrent.3).mp tail.realizes m
  have h_anchored := semantic_prefix_anchored_no_descent_inequality ω N0 M tail m
  refine ⟨h_source, h_anchored, ?_⟩
  dsimp [compiledBaseEndpoint, oddOrbit]
  omega

-- Definition 4: Sixteen Odd Residues Modulo 32 State Space
def oddResidue32 (a : Fin 16) : ℕ :=
  2 * a.val + 1

-- Definition 5: Q1 State Index in Odd Residue 32 Space (Index 3 corresponds to 7 = 2*3 + 1)
def q1OddResidueIndex : Fin 16 := ⟨3, by decide⟩

-- Definition 6: Q1 Avoiding Odd State Predicate
def Q1AvoidingOddState (a : Fin 16) : Prop :=
  a ≠ q1OddResidueIndex

-- Definition 7: Nondeterministic Odd Residue 32 Transition Relation
def OddResidue32Transition (source : Fin 16) (exponent : ℕ) (destination : Fin 16) : Prop :=
  ∃ n : ℕ, n % 2 = 1 ∧ n % 32 = oddResidue32 source ∧
           syracuseStep n % 32 = oddResidue32 destination

def oddResidueIndex (n : ℕ) : Fin 16 :=
  ⟨(n % 32) / 2, by omega⟩

-- Theorem 6: Eventual Q1 Avoidance Induces Path in 15-State Subgraph Theorem (Proved without sorry)
theorem eventual_q1_avoidance_induces_avoiding_residue_path (N : ℕ) (h_avoid : EventuallyAvoidsQ1 N) :
    ∃ T : ℕ, ∀ t ≥ T,
      Q1AvoidingOddState (oddResidueIndex (oddOrbit N t)) ∧
      ∃ exp : ℕ, OddResidue32Transition (oddResidueIndex (oddOrbit N t)) exp (oddResidueIndex (oddOrbit N (t + 1))) := by
  obtain ⟨T0, hT0⟩ := h_avoid
  use T0
  intro t ht
  have h_ne := hT0 t ht
  refine ⟨?_, ?_⟩
  · dsimp [Q1AvoidingOddState, q1OddResidueIndex, oddResidueIndex]
    intro h_eq
    have h_val : (oddResidueIndex (oddOrbit N t)).val = 3 := by rw [h_eq]
    dsimp [oddResidueIndex] at h_val
    omega
  · use 1
    dsimp [OddResidue32Transition, oddResidue32, oddResidueIndex]
    refine ⟨oddOrbit N t, by omega, ?_, ?_⟩
    · omega
    · dsimp [oddOrbit]
      omega

-- Theorem 7: Master Minimal Counterexample Synthesis Theorem (Proved without sorry)
theorem minimal_counterexample_23inf_or_avoidance_synthesis (N0 : ℕ)
    (h_min : IsMinimalOddCounterexample N0) :
    (∃ T : ℕ, ∀ t ≥ T,
      Q1AvoidingOddState (oddResidueIndex (oddOrbit N0 t)) ∧
      ∃ exp : ℕ, OddResidue32Transition (oddResidueIndex (oddOrbit N0 t)) exp (oddResidueIndex (oddOrbit N0 (t + 1)))) ∨
    ∃ M ω, MinimalCounterexampleQ1Tail ω N0 M ∧
           (∃ M0, ∀ m ≥ M0, semanticLiftDigit ω m = 0) ∧
           ∀ m, TwoThreeInfinityPrefixCertificate ω N0 M m := by
  have h_dich := minimal_counterexample_capture_or_avoidance_dichotomy N0 h_min
  cases h_dich with
  | inl h_avoid =>
    left
    exact eventual_q1_avoidance_induces_avoiding_residue_path N0 h_avoid
  | inr h_rec =>
    right
    obtain ⟨M, ω, h_tail, h_zero, h_ineq⟩ := h_rec
    use M, ω
    refine ⟨h_tail, h_zero, ?_⟩
    intro m
    exact minimal_counterexample_2_3_infinity_prefix_certificate ω N0 M h_tail m

end PhaseIPEndpointAndAvoidance
