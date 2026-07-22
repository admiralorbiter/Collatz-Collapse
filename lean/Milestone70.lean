-- Lean 4 Standalone Theorem File for Phase 7 Milestone 7.0 Prototype
-- Refactored non-vacuous separate guarded positivity theorems

import Mathlib.Data.Nat.Basic
import Mathlib.Data.Int.Basic
import Mathlib.Tactic.Omega

namespace Milestone70

-- Layer 1: Residue Guards and Affine Functions
def F_w1 (n : ℕ) : ℕ := (27 * n + 19) / 16
def F_w2 (n : ℕ) : ℕ := (27 * n + 23) / 32

def guard_Q1 (n : ℕ) : Prop := n % 32 = 7 ∧ 0 < n
def guard_Q2 (n : ℕ) : Prop := n % 64 = 43 ∧ 0 < n

-- Layer 2: Fixed-Point Linear Form Definitions
-- L2(n) = 5*n - 23 for x* = 23/5 (2^A - 3^k = 32 - 27 = 5 > 0)
def L1 (n : ℕ) : ℕ := 11 * n + 19
def L2 (n : ℕ) : ℤ := 5 * (n : ℤ) - 23

-- Layer 3: Separate Guarded Positivity Proofs (Non-Vacuous)
theorem L1_positive_on_Q1 (n : ℕ) (h : guard_Q1 n) : 0 < L1 n := by
  dsimp [L1]
  have h1 : 0 < 11 * n := Nat.mul_pos (by decide) h.2
  exact Nat.add_pos_left h1 19

theorem L2_positive_on_Q2 (n : ℕ) (h : guard_Q2 n) : 0 < L2 n := by
  dsimp [L2]
  have h_ge : 43 ≤ n := by
    have h_mod : n % 64 = 43 := h.1
    omega
  omega

end Milestone70
