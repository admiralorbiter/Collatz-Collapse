import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import Mathlib.BigOperators.Group.Finset.Basic
import Mathlib.Data.Nat.Factorization.Basic
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Algebra.Ring.Parity
import PhaseI1CounterexampleCapture
import PhaseINOrbitItineraryBridge
import PhaseISUniversalCertificate
import OddPrefixWitness

namespace PhaseI1CounterexampleCapture

open PhaseI1CounterexampleCapture
open PhaseINOrbitItineraryBridge
open PhaseISUniversalCertificate

/-!
# Gate 2: Orbit Trajectory Witness & Semantic Endpoint Identification

This file constructs the generic orbit trajectory witness and semantic endpoint identification:
1. Gate 2A: Generic Odd-Orbit Prefix Witness (`oddOrbit_prefix_has_odd_prefix_witness`) — PASSED
2. Gate 2B: Semantic Prefix Endpoint Identification (`semantic_return_state_eq_oddOrbit_prefix`)
3. Gate 2C: Source Positivity & Oddness (`semantic_realizer_source_q1/pos/odd`)
4. Gate 2D: Final Witness Bridge & Category B Shifted Height Bound (`semantic_prefix_shifted_height_bound`)
-/

-- Valuation function for 3x + 1
def v2_3x_plus_1 (x : ℕ) : ℕ :=
  if x % 2 = 1 then (3 * x + 1).factorization 2 else 0

-- Single odd step function
def oddStep (x : ℕ) : ℕ :=
  if x % 2 = 1 then (3 * x + 1) / 2 ^ v2_3x_plus_1 x else x / 2

-- Accelerated odd orbit iteration
def oddOrbit (M : ℕ) : ℕ → ℕ
  | 0 => M
  | n + 1 => oddStep (oddOrbit M n)

-- Theorem 2A.1: Exact Step Equation (Category A)
theorem oddStep_exact
    (x : ℕ)
    (_hx_pos : 0 < x)
    (hx_odd : Odd x) :
    2 ^ v2_3x_plus_1 x * oddStep x = 3 * x + 1 := by
  have hx_mod : x % 2 = 1 := Nat.odd_iff.mp hx_odd
  simpa [oddStep, v2_3x_plus_1, hx_mod] using
    (Nat.ordProj_mul_ordCompl_eq_self (3 * x + 1) 2)

-- Theorem 2A.2: Step Output Positivity (Category A)
theorem oddStep_pos
    (x : ℕ)
    (_hx_pos : 0 < x)
    (hx_odd : Odd x) :
    0 < oddStep x := by
  have hx_mod : x % 2 = 1 := Nat.odd_iff.mp hx_odd
  have hnum_ne : 3 * x + 1 ≠ 0 := by omega
  simpa [oddStep, v2_3x_plus_1, hx_mod] using
    (Nat.ordCompl_pos (n := 3 * x + 1) 2 hnum_ne)

-- Theorem 2A.3: Step Output Oddness (Category A)
theorem oddStep_odd
    (x : ℕ)
    (_hx_pos : 0 < x)
    (hx_odd : Odd x) :
    Odd (oddStep x) := by
  have hx_mod : x % 2 = 1 := Nat.odd_iff.mp hx_odd
  have hnum_ne : 3 * x + 1 ≠ 0 := by omega
  have hnot :
      ¬ 2 ∣
        (3 * x + 1) /
          2 ^ (3 * x + 1).factorization 2 :=
    Nat.not_dvd_ordCompl
      (n := 3 * x + 1)
      (p := 2)
      Nat.prime_two
      hnum_ne
  apply Nat.odd_iff.mpr
  simpa [oddStep, v2_3x_plus_1, hx_mod] using
    (Nat.two_dvd_ne_zero.mp hnot)

-- Theorem 2A.4: Valuation Positivity (Category A Helper)
theorem v2_3x_plus_1_pos
    (x : ℕ)
    (hx_odd : Odd x) :
    0 < v2_3x_plus_1 x := by
  have hx_mod : x % 2 = 1 := Nat.odd_iff.mp hx_odd
  have hnum_ne : 3 * x + 1 ≠ 0 := by omega
  have htwo_dvd : 2 ∣ 3 * x + 1 := by
    apply Nat.dvd_of_mod_eq_zero
    omega
  have hfactor :
      1 ≤ (3 * x + 1).factorization 2 :=
    (Nat.prime_two.dvd_iff_one_le_factorization hnum_ne).mp
      htwo_dvd
  simpa [v2_3x_plus_1, hx_mod] using hfactor

-- Theorem 2A.5: Odd Orbit State Positivity (Category A)
theorem oddOrbit_state_pos (M : ℕ) (hM_pos : 0 < M) (n : ℕ) :
    0 < oddOrbit M n := by
  induction n with
  | zero => exact hM_pos
  | succ k ih =>
    dsimp [oddOrbit]
    by_cases h_odd : oddOrbit M k % 2 = 1
    · have h_st_odd : Odd (oddOrbit M k) := Nat.odd_iff.mpr h_odd
      exact oddStep_pos (oddOrbit M k) ih h_st_odd
    · dsimp [oddStep]
      rw [if_neg h_odd]
      omega

-- Theorem 2A.6: Odd Orbit State Oddness (Category A)
theorem oddOrbit_state_odd
    (M : ℕ)
    (hM_odd : Odd M)
    (n : ℕ) :
    Odd (oddOrbit M n) := by
  have hM_pos : 0 < M := by
    obtain ⟨k, hk⟩ := hM_odd
    omega
  induction n with
  | zero =>
      simpa [oddOrbit] using hM_odd
  | succ k ih =>
      have hk_pos : 0 < oddOrbit M k :=
        oddOrbit_state_pos M hM_pos k
      simpa [oddOrbit] using
        oddStep_odd (oddOrbit M k) hk_pos ih

-- Theorem 2A.7: Exact Step Equation Along Orbit (Category A)
theorem oddOrbit_step_exact
    (M n : ℕ)
    (hM_pos : 0 < M)
    (hM_odd : Odd M) :
    2 ^ v2_3x_plus_1 (oddOrbit M n) *
        oddOrbit M (n + 1)
      =
    3 * oddOrbit M n + 1 := by
  have h_pos : 0 < oddOrbit M n :=
    oddOrbit_state_pos M hM_pos n
  have h_odd : Odd (oddOrbit M n) :=
    oddOrbit_state_odd M hM_odd n
  simpa [oddOrbit] using
    oddStep_exact (oddOrbit M n) h_pos h_odd

-- Theorem 2A.8: Generic Odd Orbit Has Odd Prefix Witness (Category A Headline Constructor)
theorem oddOrbit_prefix_has_odd_prefix_witness
    (M T : ℕ)
    (hM_pos : 0 < M)
    (hM_odd : Odd M) :
    OddPrefixWitness M T (oddOrbit M T) := by
  refine
    { state := fun i =>
        oddOrbit M i.val
      valuation := fun i =>
        v2_3x_plus_1 (oddOrbit M i.val)
      start_eq := by rfl
      end_eq := by rfl
      valuation_pos := ?_
      step_eq := ?_ }
  · intro i
    exact
      v2_3x_plus_1_pos
        (oddOrbit M i.val)
        (oddOrbit_state_odd M hM_odd i.val)
  · intro i
    simpa using
      oddOrbit_step_exact M i.val hM_pos hM_odd

-- Theorem 2B.1: Orbit Addition Identity (Category A)
theorem oddOrbit_add (M T U : ℕ) :
    oddOrbit (oddOrbit M T) U = oddOrbit M (T + U) := by
  induction U with
  | zero => rfl
  | succ k ih =>
    have h_eq : T + (k + 1) = (T + k) + 1 := by omega
    rw [h_eq]
    dsimp [oddOrbit]
    rw [ih]

-- Gate 2C: Source Facts Derived directly from RealizesSemanticItinerary (Category B Bridges)

theorem semantic_realizer_source_q1
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    M % 32 = 7 := by
  have h0 := hreal 0
  exact h0.1

theorem semantic_realizer_source_pos
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    0 < M := by
  have hq1 := semantic_realizer_source_q1 ω M hreal
  omega

theorem semantic_realizer_source_odd
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    Odd M := by
  have hq1 := semantic_realizer_source_q1 ω M hreal
  use 16 * (M / 32) + 3
  omega

-- Gate 2B: Semantic Prefix Time & Return State API

def semanticPrefixTimeItinerary (ω : InfiniteSemanticItinerary) (m : ℕ) : ℕ :=
  semanticPrefixTime (semanticPrefix ω m)

def semanticReturnStateItinerary (ω : InfiniteSemanticItinerary) (M m : ℕ) : ℕ :=
  oddOrbit M (semanticPrefixTimeItinerary ω m)

@[simp]
theorem semanticPrefixTime_zero (ω : InfiniteSemanticItinerary) :
    semanticPrefixTimeItinerary ω 0 = 0 := by
  rfl

@[simp]
theorem semanticReturnState_zero (ω : InfiniteSemanticItinerary) (M : ℕ) :
    semanticReturnStateItinerary ω M 0 = M := by
  rfl

-- Load-Bearing Category B Bridge Theorem: Semantic Return State Equals Odd Orbit at Prefix Time
theorem semantic_return_state_eq_oddOrbit_prefix
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (_hreal : RealizesSemanticItinerary ω M) :
    semanticReturnStateItinerary ω M m = oddOrbit M (semanticPrefixTimeItinerary ω m) := by
  rfl

-- Gate 2D: Final Category B Witness Bridge & Shifted Height Headline Theorem

theorem semantic_prefix_has_odd_prefix_witness
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    OddPrefixWitness
      M
      (semanticPrefixTimeItinerary ω m)
      (semanticReturnStateItinerary ω M m) := by
  have hpos := semantic_realizer_source_pos ω M hreal
  have hodd := semantic_realizer_source_odd ω M hreal
  dsimp [semanticReturnStateItinerary]
  exact oddOrbit_prefix_has_odd_prefix_witness M (semanticPrefixTimeItinerary ω m) hpos hodd

-- Category B Headline Theorem: Semantic Prefix Shifted Height Bound
-- Proved with ZERO witness arguments and ZERO aggregate bound hypotheses!
theorem semantic_prefix_shifted_height_bound
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    2 ^ (semanticPrefixTimeItinerary ω m) *
        (semanticReturnStateItinerary ω M m + 1)
      ≤
    3 ^ (semanticPrefixTimeItinerary ω m) *
        (M + 1) := by
  have w := semantic_prefix_has_odd_prefix_witness ω M m hreal
  exact odd_prefix_shifted_height_bound_of_witness M (semanticPrefixTimeItinerary ω m) (semanticReturnStateItinerary ω M m) w

end PhaseI1CounterexampleCapture
