-- Formal Lean 4 Theorem Suite for Phase H.1 Minimal Pointwise Reduction
import Mathlib.Data.Int.Basic
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith

namespace PhaseH1Pointwise

-- 1. Precision Schedule Strict Monotonicity
def PrecisionSchedule (H : ℕ → ℕ) : Prop :=
  StrictMono H

-- 2. Projective Compatibility Theorem
theorem projective_compatibility (r_n r_next H_n k : ℤ)
    (h_compat : r_next = r_n + k * (2 ^ H_n.toNat)) :
    r_next ≡ r_n [ZMOD (2 ^ H_n.toNat)] := by
  use k
  omega

-- 3. Least Representative Decomposition & Monotonicity
theorem least_representatives_monotone (R_n R_next H_n : ℤ) (lambda_next : ℤ)
    (h_lift : R_next = R_n + lambda_next * (2 ^ H_n.toNat))
    (h_lambda_nonneg : lambda_next ≥ 0)
    (h_H_pos : 2 ^ H_n.toNat > 0) :
    R_next ≥ R_n := by
  have h_mul : lambda_next * (2 ^ H_n.toNat) ≥ 0 := by
    nlinarith
  linarith

-- 4. Representation Predicate Definition
def RepresentsNat (R : ℕ → ℤ) (H : ℕ → ℕ) (N : ℤ) : Prop :=
  ∀ n, R n ≡ N [ZMOD (2 ^ H n)]

-- 5. Fixed-Natural Source Stabilization Theorem
theorem natural_realization_iff_eventually_constant (R : ℕ → ℤ) (H : ℕ → ℕ) (N : ℤ) (K : ℕ)
    (h_bound : ∀ n ≥ K, 2 ^ H n > N ∧ N ≥ 0)
    (h_R_least : ∀ n, R n = N % (2 ^ H n)) :
    ∀ n ≥ K, R n = N := by
  intro n hn
  have h1 : N % (2 ^ H n) = N := by
    have h_b := h_bound n hn
    omega
  rw [h_R_least n, h1]

-- 6. Zero Lift-Block Tail Characterization
theorem stabilization_iff_zero_lift_tail (R : ℕ → ℤ) (H : ℕ → ℕ) (lambda : ℕ → ℤ) (K : ℕ)
    (h_lift : ∀ n, R (n + 1) - R n = lambda (n + 1) * (2 ^ H n))
    (h_H_pos : ∀ n, 2 ^ H n > 0) :
    (∀ n ≥ K, R (n + 1) = R n) ↔ (∀ n ≥ K, lambda (n + 1) = 0) := by
  constructor
  · intro h n hn
    have h_eq := h n hn
    have h_diff : R (n + 1) - R n = 0 := by linarith
    have h_l := h_lift n
    rw [h_diff] at h_l
    have h_pos := h_H_pos n
    have h_mul : lambda (n + 1) * (2 ^ H n) = 0 := h_l.symm
    exact (mul_eq_zero.mp h_mul).resolve_right (ne_of_gt h_pos)
  · intro h n hn
    have h_zero := h n hn
    have h_l := h_lift n
    rw [h_zero, zero_mul] at h_l
    linarith

end PhaseH1Pointwise
