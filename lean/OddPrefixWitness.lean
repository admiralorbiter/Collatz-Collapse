import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import Mathlib.Data.List.Nodup
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith
import Mathlib.BigOperators.Group.Finset.Basic

namespace PhaseI1CounterexampleCapture

/-!
# Gate 1: Derived Shifted-Height Contraction & Prefix Invariant

This file proves the core trajectory contraction and prefix shifted-height bounds:
1. `odd_step_shifted_height_step`: $2(s_{j+1} + 1) \le 3(s_j + 1)$ for any odd step $2^k s_{j+1} = 3 s_j + 1$ with $k \ge 1$.
2. `odd_prefix_shifted_height_at`: $\forall j \le T, 2^j (x_j + 1) \le 3^j (M + 1)$ by induction on $j$.
3. `odd_prefix_shifted_height_bound_of_witness`: $2^T (y + 1) \le 3^T (M + 1)$ derived cleanly from the witness.
-/

-- Definition: Trajectory Witness for an Odd-Step Collatz Return Prefix
-- T: number of accelerated odd steps
-- A: sum of 2-adic valuations along the T steps
structure OddPrefixWitness (M T y : ℕ) where
  state : Fin (T + 1) → ℕ
  valuation : Fin T → ℕ
  start_eq : state ⟨0, Nat.zero_lt_succ T⟩ = M
  end_eq : state ⟨T, Nat.lt_succ_self T⟩ = y
  valuation_pos : ∀ i : Fin T, 1 ≤ valuation i
  step_eq : ∀ i : Fin T, 2 ^ valuation i * state ⟨i.val + 1, Nat.succ_lt_succ i.isLt⟩ = 3 * state ⟨i.val, Nat.lt_trans i.isLt (Nat.lt_succ_self T)⟩ + 1

-- Lemma 1: Single-Step Shifted Height Inequality (Category A Core Theorem)
-- Proved from 2 <= 2^k and 2^k * s_next = 3 * s_curr + 1
theorem odd_step_shifted_height_step
    (s_curr s_next k : ℕ)
    (hk : 1 ≤ k)
    (hstep : 2 ^ k * s_next = 3 * s_curr + 1) :
    2 * (s_next + 1) ≤ 3 * (s_curr + 1) := by
  have h_pow2 : 2 ≤ 2 ^ k := Nat.one_lt_two_pow hk
  have h1 : 2 * s_next ≤ 2 ^ k * s_next := by nlinarith
  rw [hstep] at h1
  linarith

-- State lookup helper for indexed prefix invariant
def witnessStateAt (M T y : ℕ) (w : OddPrefixWitness M T y) (j : ℕ) (hj : j ≤ T) : ℕ :=
  w.state ⟨j, Nat.lt_succ_of_le hj⟩

-- Lemma 2: Indexed Prefix Shifted-Height Invariant (Category A Core Theorem)
-- Proved by induction on j <= T using odd_step_shifted_height_step
theorem odd_prefix_shifted_height_at
    (M T y : ℕ)
    (w : OddPrefixWitness M T y)
    (j : ℕ)
    (hj : j ≤ T) :
    2 ^ j * (witnessStateAt M T y w j hj + 1) ≤ 3 ^ j * (M + 1) := by
  induction j with
  | zero =>
    dsimp [witnessStateAt]
    rw [w.start_eq]
    ring_nf
    linarith
  | succ k ih =>
    have hk : k < T := by omega
    have hk_le : k ≤ T := by omega
    have ih_k := ih hk_le
    have h_step_k := w.step_eq ⟨k, hk⟩
    have h_pos_k := w.valuation_pos ⟨k, hk⟩
    have h_step_ineq := odd_step_shifted_height_step (witnessStateAt M T y w k hk_le) (witnessStateAt M T y w (k + 1) hj) (w.valuation ⟨k, hk⟩) h_pos_k h_step_k
    have h_mul : 2 ^ k * (2 * (witnessStateAt M T y w (k + 1) hj + 1)) ≤ 2 ^ k * (3 * (witnessStateAt M T y w k hk_le + 1)) := by
      nlinarith
    have h_assoc1 : 2 ^ (k + 1) * (witnessStateAt M T y w (k + 1) hj + 1) = 2 ^ k * (2 * (witnessStateAt M T y w (k + 1) hj + 1)) := by
      ring
    have h_assoc2 : 2 ^ k * (3 * (witnessStateAt M T y w k hk_le + 1)) = 3 * (2 ^ k * (witnessStateAt M T y w k hk_le + 1)) := by
      ring
    rw [h_assoc1, h_assoc2]
    have h_ih_mul : 3 * (2 ^ k * (witnessStateAt M T y w k hk_le + 1)) ≤ 3 * (3 ^ k * (M + 1)) := by
      nlinarith [ih_k]
    have h_pow_succ : 3 ^ (k + 1) * (M + 1) = 3 * (3 ^ k * (M + 1)) := by
      ring
    linarith

-- Theorem 1: Derived Prefix Shifted Height Bound from OddPrefixWitness (Category A Core Headline Theorem)
-- Derived directly at j = T without any aggregate bound or externally provided inequality hypothesis!
theorem odd_prefix_shifted_height_bound_of_witness
    (M T y : ℕ)
    (w : OddPrefixWitness M T y) :
    2 ^ T * (y + 1) ≤ 3 ^ T * (M + 1) := by
  have h_at := odd_prefix_shifted_height_at M T y w T (by omega)
  have h_end : witnessStateAt M T y w T (by omega) = y := by
    dsimp [witnessStateAt]
    exact w.end_eq
  rw [h_end] at h_at
  exact h_at

end PhaseI1CounterexampleCapture
