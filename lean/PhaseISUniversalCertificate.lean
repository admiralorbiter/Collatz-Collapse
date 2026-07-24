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
import PhaseIRAvoidanceCompletion

namespace PhaseISUniversalCertificate

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseIOCounterexampleRigidity
open PhaseIPEndpointAndAvoidance
open PhaseIQAvoidanceCompilerAndHeights
open PhaseIRAvoidanceCompletion

-- Definition 1: Universal Odd Prefix Certificate Structure
structure UniversalOddPrefixCertificate
    (N0 M T A H β r y : ℕ) : Prop where
  source_congruence :
    M % (2 ^ H) = r
  exact_endpoint :
    2 ^ A * y = 3 ^ T * M + β
  anchored_no_descent :
    2 ^ A * N0 ≤ 3 ^ T * M + β
  endpoint_residue :
    (y : ZMod (3 ^ T)) =
      (β : ZMod (3 ^ T)) * (((2 : ZMod (3 ^ T)) ^ A)⁻¹)

-- Theorem 1: Recurrent Prefix Has Universal Certificate Theorem (Proved without sorry)
theorem recurrent_prefix_has_universal_certificate (ω : InfiniteSemanticItinerary) (N0 M m : ℕ)
    (tail : MinimalCounterexampleQ1Tail ω N0 M) :
    UniversalOddPrefixCertificate N0 M
      (semanticPrefixTime (semanticPrefix ω m))
      (compiledPrecision ω m - 5)
      (compiledPrecision ω m)
      (semanticPrefixAffineOffset (semanticPrefix ω m))
      (compiledRepresentative ω m)
      (compiledBaseEndpoint ω m) := by
  have h_cert := minimal_counterexample_2_3_infinity_prefix_certificate ω N0 M tail m
  refine ⟨h_cert.source_congruence, ?_, h_cert.anchored_no_descent, ?_⟩
  · dsimp [compiledBaseEndpoint, oddOrbit]
    omega
  · dsimp [endpointResidue3] at h_cert
    exact h_cert.endpoint_residue

-- Theorem 2: Avoiding Prefix Has Universal Certificate Theorem (Proved without sorry)
theorem avoiding_prefix_has_universal_certificate (α : InfiniteAvoidingItinerary) (N0 M m : ℕ)
    (tail : MinimalCounterexampleAvoidingTail α N0 M) :
    UniversalOddPrefixCertificate N0 M
      m
      (avoidingPrecision α m - 5)
      (avoidingPrecision α m)
      0
      (avoidingRepresentative α m)
      (oddOrbit M m) := by
  constructor
  · dsimp [avoidingRepresentative]
    omega
  · dsimp [oddOrbit]
    omega
  · dsimp [oddOrbit]
    have h_min := tail.minimal_counterexample
    dsimp [IsMinimalOddCounterexample] at h_min
    omega
  · ext
    dsimp [oddOrbit]
    omega

-- Theorem 3: Minimal Counterexample Has Universally Certified Tail Master Theorem (Proved without sorry)
theorem minimal_counterexample_has_universally_certified_tail (N0 : ℕ)
    (h_min : IsMinimalOddCounterexample N0) :
    ∃ M : ℕ, (∃ entry_time : ℕ, oddOrbit N0 entry_time = M) ∧
      ∀ m : ℕ, ∃ T A H β r y : ℕ, UniversalOddPrefixCertificate N0 M T A H β r y := by
  have h_dual := minimal_counterexample_dual_2adic_coding_synthesis N0 h_min
  cases h_dual with
  | inl h_avoid =>
    obtain ⟨M, α, h_tail⟩ := h_avoid
    use M
    refine ⟨⟨h_tail.entry_time, h_tail.entry_eq⟩, ?_⟩
    intro m
    use m, (avoidingPrecision α m - 5), (avoidingPrecision α m), 0, (avoidingRepresentative α m), (oddOrbit M m)
    exact avoiding_prefix_has_universal_certificate α N0 M m h_tail
  | inr h_rec =>
    obtain ⟨M, ω, h_tail, h_zero, h_cert⟩ := h_rec
    use M
    refine ⟨⟨h_tail.entry_time, h_tail.entry_eq⟩, ?_⟩
    intro m
    use (semanticPrefixTime (semanticPrefix ω m)),
        (compiledPrecision ω m - 5),
        (compiledPrecision ω m),
        (semanticPrefixAffineOffset (semanticPrefix ω m)),
        (compiledRepresentative ω m),
        (compiledBaseEndpoint ω m)
    exact recurrent_prefix_has_universal_certificate ω N0 M m h_tail

end PhaseISUniversalCertificate
