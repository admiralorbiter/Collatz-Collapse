import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import PhaseI1CounterexampleCapture
import OddPrefixWitness
import Gate2OrbitBridge
import Gate3TimeGrowth
import Gate4ResidueBridge

namespace PhaseI1CounterexampleCapture

open PhaseI1CounterexampleCapture

/-!
# Gate 5: Tail Compression & Eventual Least Representative Equality

This file derives eventual least representative equality for actual Collatz return tails:
1. Gate 5A: `sufficient_time_forces_endpoint_lt_three_power` — $y_m < 3^{T_m}$ for $m \ge m_0$.
2. Gate 5B: `recurrent_tail_eventually_endpoint_eq_least_representative` — $y_m = \mu_m$ for $m \ge m_0$.
3. Gate 5C: `semantic_return_endpoint_compression` — $2^K y_m < 3^{T_m}$ eventually for every $K$.
4. Gate 5C: `semantic_return_endpoint_has_leading_ternary_zeros` — $y_m < 3^{T_m - L}$ eventually for every $L$.
-/

-- Theorem 5A: Eventual Return State Bound Below 3^T_m (Category B Bridge)
theorem sufficient_time_forces_endpoint_lt_three_power
    (ω : InfiniteSemanticItinerary) (M m : ℕ)
    (hreal : RealizesSemanticItinerary ω M)
    (hm : M + 1 ≤ m) :
    semanticReturnStateItinerary ω M m < 3 ^ (semanticPrefixTimeItinerary ω m) := by
  have h_bound := semantic_prefix_shifted_height_bound ω M m hreal
  have h_time := initial_height_below_two_power_of_large_index ω M m hm
  have h_combine : 2 ^ (semanticPrefixTimeItinerary ω m) * (semanticReturnStateItinerary ω M m + 1) ≤
                    3 ^ (semanticPrefixTimeItinerary ω m) * 2 ^ (semanticPrefixTimeItinerary ω m) := by
    calc
      2 ^ (semanticPrefixTimeItinerary ω m) * (semanticReturnStateItinerary ω M m + 1)
        ≤ 3 ^ (semanticPrefixTimeItinerary ω m) * (M + 1) := h_bound
      _ ≤ 3 ^ (semanticPrefixTimeItinerary ω m) * 2 ^ (semanticPrefixTimeItinerary ω m) := by
        nlinarith [h_time]
  have h_div : (semanticReturnStateItinerary ω M m + 1) ≤ 3 ^ (semanticPrefixTimeItinerary ω m) := by
    nlinarith [Nat.two_pow_pos (semanticPrefixTimeItinerary ω m)]
  omega

-- Gate 5B Headline Theorem: Eventual Least Representative Equality for Actual Collatz Tails (Category B Headline)
theorem recurrent_tail_eventually_endpoint_eq_least_representative
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    ∃ m₀ : ℕ, ∀ m ≥ m₀,
      semanticReturnStateItinerary ω M m = semanticPrefixEndpointResidueNat ω m := by
  refine ⟨M + 1, ?_⟩
  intro m hm
  have h_lt := sufficient_time_forces_endpoint_lt_three_power ω M m hreal hm
  have h_mod := semantic_return_state_endpoint_mod ω M m hreal
  dsimp [semanticPrefixEndpointResidueNat] at *
  rw [Nat.mod_eq_of_lt h_lt] at h_mod
  exact h_mod

-- Theorem 3B Strong: Strong Exponential Time Domination (Category A)
theorem semantic_prefix_time_strong_exponential_domination
    (ω : InfiniteSemanticItinerary)
    (M K : ℕ) :
    ∃ m₀ : ℕ, ∀ m ≥ m₀,
      (M + 1) * 2 ^ K < 2 ^ semanticPrefixTimeItinerary ω m := by
  refine ⟨(M + 1) * 2 ^ K + 1, ?_⟩
  intro m hm
  have htime : m ≤ semanticPrefixTimeItinerary ω m :=
    semantic_prefix_time_ge_index ω m
  calc
    (M + 1) * 2 ^ K < (M + 1) * 2 ^ K + 1 := by omega
    _ ≤ m := hm
    _ ≤ 2 ^ m := nat_le_two_pow m
    _ ≤ 2 ^ semanticPrefixTimeItinerary ω m := two_pow_mono htime

-- Gate 5C Headline 1: Full Actual-Tail Asymptotic Compression (Category B Headline)
-- Proved 100% without sorry for any arbitrary K!
theorem semantic_return_endpoint_compression
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    ∀ K : ℕ, ∃ m₀ : ℕ, ∀ m ≥ m₀,
      2 ^ K * (semanticReturnStateItinerary ω M m) < 3 ^ (semanticPrefixTimeItinerary ω m) := by
  intro K
  obtain ⟨m0, hm0⟩ := semantic_prefix_time_strong_exponential_domination ω M K
  refine ⟨m0, ?_⟩
  intro m hm
  have h_bound := semantic_prefix_shifted_height_bound ω M m hreal
  have h_dom := hm0 m hm
  have h1 : 2 ^ K * 2 ^ (semanticPrefixTimeItinerary ω m) * (semanticReturnStateItinerary ω M m + 1) ≤
            2 ^ K * 3 ^ (semanticPrefixTimeItinerary ω m) * (M + 1) := by
    nlinarith [h_bound]
  have h2 : 2 ^ K * 3 ^ (semanticPrefixTimeItinerary ω m) * (M + 1) =
            3 ^ (semanticPrefixTimeItinerary ω m) * ((M + 1) * 2 ^ K) := by
    ring
  rw [h2] at h1
  have h3 : 3 ^ (semanticPrefixTimeItinerary ω m) * ((M + 1) * 2 ^ K) <
            3 ^ (semanticPrefixTimeItinerary ω m) * 2 ^ (semanticPrefixTimeItinerary ω m) := by
    nlinarith [h_dom, Nat.pos_of_ne_zero (by positivity : 3 ^ (semanticPrefixTimeItinerary ω m) ≠ 0)]
  have h4 : 2 ^ (semanticPrefixTimeItinerary ω m) * (2 ^ K * semanticReturnStateItinerary ω M m) <
            2 ^ (semanticPrefixTimeItinerary ω m) * 3 ^ (semanticPrefixTimeItinerary ω m) := by
    calc
      2 ^ (semanticPrefixTimeItinerary ω m) * (2 ^ K * semanticReturnStateItinerary ω M m)
        ≤ 2 ^ K * 2 ^ (semanticPrefixTimeItinerary ω m) * (semanticReturnStateItinerary ω M m + 1) := by
          nlinarith
      _ < 3 ^ (semanticPrefixTimeItinerary ω m) * 2 ^ (semanticPrefixTimeItinerary ω m) := by
          linarith
      _ = 2 ^ (semanticPrefixTimeItinerary ω m) * 3 ^ (semanticPrefixTimeItinerary ω m) := by
          ring
  exact Nat.lt_of_mul_lt_mul_left h4

-- Gate 5C Headline 2: Leading Ternary Zeros in Base 3 Representation (Category B Headline)
-- Proved 100% without sorry for any arbitrary L leading zeros!
theorem semantic_return_endpoint_has_leading_ternary_zeros
    (ω : InfiniteSemanticItinerary) (M : ℕ)
    (hreal : RealizesSemanticItinerary ω M) :
    ∀ L : ℕ, ∃ m₀ : ℕ, ∀ m ≥ m₀,
      L ≤ semanticPrefixTimeItinerary ω m ∧
      semanticReturnStateItinerary ω M m < 3 ^ (semanticPrefixTimeItinerary ω m - L) := by
  intro L
  -- Pick K such that 3^L <= 2^K
  have h3L : ∃ K : ℕ, 3 ^ L ≤ 2 ^ K := by
    use 2 * L
    induction L with
    | zero => decide
    | succ k ih =>
      calc
        3 ^ (k + 1) = 3 * 3 ^ k := by ring
        _ ≤ 3 * 2 ^ (2 * k) := by nlinarith
        _ ≤ 4 * 2 ^ (2 * k) := by nlinarith
        _ = 2 ^ (2 * (k + 1)) := by ring
  obtain ⟨K, hK⟩ := h3L
  obtain ⟨m0_time, hm0_time⟩ := semantic_prefix_time_strong_exponential_domination ω 0 L
  obtain ⟨m0_comp, hm0_comp⟩ := semantic_return_endpoint_compression ω M hreal K
  let m0 := max m0_time m0_comp
  refine ⟨m0, ?_⟩
  intro m hm
  have hm_t : m ≥ m0_time := by omega
  have hm_c : m ≥ m0_comp := by omega
  have h_time_dom := hm0_time m hm_t
  have h_comp := hm0_comp m hm_c
  dsimp at h_time_dom
  have hL_le : L ≤ semanticPrefixTimeItinerary ω m := by
    have h2L : 2 ^ L ≤ 2 ^ (semanticPrefixTimeItinerary ω m) := by omega
    exact (Nat.pow_le_pow_iff_right (by decide)).mp h2L
  refine ⟨hL_le, ?_⟩
  have h_sub : 3 ^ semanticPrefixTimeItinerary ω m = 3 ^ L * 3 ^ (semanticPrefixTimeItinerary ω m - L) := by
    rw [← pow_add, Nat.add_sub_of_le hL_le]
  rw [h_sub] at h_comp
  have h_mul : 3 ^ L * semanticReturnStateItinerary ω M m < 3 ^ L * 3 ^ (semanticPrefixTimeItinerary ω m - L) := by
    calc
      3 ^ L * semanticReturnStateItinerary ω M m
        ≤ 2 ^ K * semanticReturnStateItinerary ω M m := by nlinarith [hK]
      _ < 3 ^ L * 3 ^ (semanticPrefixTimeItinerary ω m - L) := h_comp
  exact Nat.lt_of_mul_lt_mul_left h_mul

end PhaseI1CounterexampleCapture
