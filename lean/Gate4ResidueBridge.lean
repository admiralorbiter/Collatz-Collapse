import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import PhaseI1CounterexampleCapture
import PhaseINOrbitItineraryBridge
import OddPrefixWitness
import Gate2OrbitBridge
import Gate3TimeGrowth

namespace PhaseI1CounterexampleCapture

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge

/-!
# Gate 4: 3-Adic Endpoint Residue Congruence Bridge

This file constructs the 3-adic endpoint residue bridge from the independent semantic compiler:
1. Gate 4A: `semanticPrefixEndpointResidueNat` definition from independent `compiledBaseEndpoint`.
2. Gate 4B: `semantic_prefix_endpoint_residue_lt` — Natural representative strictly bound by $3^{T_m}$.
3. Gate 4C: `semantic_return_state_endpoint_mod` — Nat remainder equality $y_m \pmod{3^{T_m}} = \mu_m$.
4. Gate 4D: `semantic_return_state_endpoint_congruence` — ZMod congruence corollary.
-/

-- Definition 4A: Natural 3-Adic Endpoint Residue Representative from Independent Semantic Compiler
def semanticPrefixEndpointResidueNat (ω : InfiniteSemanticItinerary) (m : ℕ) : ℕ :=
  compiledBaseEndpoint ω m % (3 ^ semanticPrefixTimeItinerary ω m)

-- Theorem 4B: Natural Endpoint Residue Bounds (Category A Structural Arithmetic)
theorem semantic_prefix_endpoint_residue_lt
    (ω : InfiniteSemanticItinerary)
    (m : ℕ) :
    semanticPrefixEndpointResidueNat ω m < 3 ^ semanticPrefixTimeItinerary ω m := by
  dsimp [semanticPrefixEndpointResidueNat]
  have h_pow_pos : 0 < 3 ^ semanticPrefixTimeItinerary ω m := Nat.pos_of_ne_zero (by positivity)
  exact Nat.mod_lt _ h_pow_pos

-- Theorem 4C: Semantic Return State Equals Compiled Base Endpoint Modulo 3^T_m (Category B Bridge)
-- Derived directly from RealizesSemanticItinerary and compiled base endpoints!
theorem semantic_return_state_endpoint_mod
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    semanticReturnStateItinerary ω M m % (3 ^ semanticPrefixTimeItinerary ω m) =
      semanticPrefixEndpointResidueNat ω m := by
  dsimp [semanticReturnStateItinerary, semanticPrefixEndpointResidueNat]
  have h_eq : semanticReturnStateItinerary ω M m = oddOrbit M (semanticPrefixTimeItinerary ω m) :=
    semantic_return_state_eq_oddOrbit_prefix ω M m hreal
  dsimp [semanticReturnStateItinerary] at h_eq
  rw [h_eq]
  rfl

-- Theorem 4D: ZMod 3-Adic Endpoint Congruence Corollary (Category B Corollary)
theorem semantic_return_state_endpoint_congruence
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    ((semanticReturnStateItinerary ω M m : ℕ) : ZMod (3 ^ semanticPrefixTimeItinerary ω m)) =
      ((semanticPrefixEndpointResidueNat ω m : ℕ) : ZMod (3 ^ semanticPrefixTimeItinerary ω m)) := by
  have h_mod := semantic_return_state_endpoint_mod ω M m hreal
  exact ZMod.eq_iff_modEq_nat.mpr (by exact h_mod)

end PhaseI1CounterexampleCapture
