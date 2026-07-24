import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import PhaseI1CounterexampleCapture
import Gate2OrbitBridge

namespace PhaseI1CounterexampleCapture

open PhaseI1CounterexampleCapture

/-!
# Gate 3: Time Growth & Exponential Domination

This file proves the linear growth of semantic prefix time and exponential domination:
1. Gate 3A: `semantic_word_time_pos` — $1 \le w.\text{word}.\text{length}$ for all return words.
2. Gate 3B: `semantic_prefix_time_ge_index` — $m \le T_m$ proved by induction on $m$.
3. Gate 3C: `nat_le_two_pow` — $n \le 2^n$ proved by induction on $n$.
4. Gate 3D: `two_pow_mono` — $a \le b \implies 2^a \le 2^b$.
5. Gate 3 Headline: `semantic_prefix_time_exponential_domination` — $\exists m_0, \forall m \ge m_0, M + 1 \le 2^{T_m}$ with explicit $m_0 = M + 1$.
-/

-- Theorem 3A: Every Semantic Return Word Contributes Positive Time (Category A)
theorem semantic_word_time_pos (w : SemanticReturnWord) :
    1 ≤ w.word.length := by
  have h_ne := w.valid.1
  cases h_len : w.word.length with
  | zero =>
    have h_nil : w.word = [] := List.length_eq_zero_iff.mp h_len
    contradiction
  | succ k =>
    omega

-- Theorem 3B: Prefix Time Dominates Index (Category A)
theorem semantic_prefix_time_ge_index
    (ω : InfiniteSemanticItinerary) (m : ℕ) :
    m ≤ semanticPrefixTimeItinerary ω m := by
  induction m with
  | zero =>
    dsimp [semanticPrefixTimeItinerary, semanticPrefixTime, semanticPrefix]
    rfl
  | succ k ih =>
    dsimp [semanticPrefixTimeItinerary, semanticPrefixTime, semanticPrefix]
    have h_take : (List.range (k + 1)).map ω = ((List.range k).map ω) ++ [ω k] := by
      rw [List.range_succ, List.map_append]
      rfl
    rw [h_take]
    dsimp [semanticPrefixTime]
    rw [List.foldl_append]
    dsimp [List.foldl]
    have h_pos := semantic_word_time_pos (ω k)
    have h_fold : ((List.range k).map ω).foldl (fun acc w => acc + w.word.length) 0 = semanticPrefixTimeItinerary ω k := by
      rfl
    omega

-- Theorem 3C: Elementary Natural Exponential Domination Helper (Category A)
theorem nat_le_two_pow (n : ℕ) :
    n ≤ 2 ^ n := by
  induction n with
  | zero => decide
  | succ k ih =>
    rw [pow_succ]
    omega

-- Theorem 3D: Power Monotonicity Helper (Category A)
theorem two_pow_mono {a b : ℕ} (hab : a ≤ b) :
    2 ^ a ≤ 2 ^ b := by
  obtain ⟨d, rfl⟩ := Nat.exists_eq_add_of_le hab
  calc
    2 ^ a = 2 ^ a * 1 := by ring
    _ ≤ 2 ^ a * 2 ^ d := Nat.mul_le_mul_left (2 ^ a) (Nat.one_le_two_pow)
    _ = 2 ^ (a + d) := by ring

-- Pointwise Height Domination Lemma (Category A Structural Arithmetic)
theorem initial_height_below_two_power_of_large_index
    (ω : InfiniteSemanticItinerary)
    (M m : ℕ)
    (hm : M + 1 ≤ m) :
    M + 1 ≤ 2 ^ semanticPrefixTimeItinerary ω m := by
  have htime : m ≤ semanticPrefixTimeItinerary ω m :=
    semantic_prefix_time_ge_index ω m
  calc
    M + 1 ≤ m := hm
    _ ≤ 2 ^ m := nat_le_two_pow m
    _ ≤ 2 ^ semanticPrefixTimeItinerary ω m :=
      two_pow_mono htime

-- Gate 3 Headline Theorem: Exponential Domination of Initial Height (Category A Structural Arithmetic)
-- Proved with EXPLICIT threshold m0 = M + 1 and 100% without sorry!
theorem semantic_prefix_time_exponential_domination
    (ω : InfiniteSemanticItinerary)
    (M : ℕ) :
    ∃ m₀ : ℕ, ∀ m ≥ m₀,
      M + 1 ≤ 2 ^ semanticPrefixTimeItinerary ω m := by
  refine ⟨M + 1, ?_⟩
  intro m hm
  exact initial_height_below_two_power_of_large_index ω M m hm

end PhaseI1CounterexampleCapture
