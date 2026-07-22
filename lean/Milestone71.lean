-- Lean 4 Standalone Theorem File for Phase 7.1 Semantic Discovery
-- Formalizes Sound Refined Cylinder Inclusions (mod 256 -> mod 16) and Non-Zero Linear Form Positivity

import Mathlib.Data.Nat.Basic
import Mathlib.Data.Int.Basic
import Mathlib.Tactic.Omega

namespace Milestone71

-- Layer 1: Affine Functions & Guards
def F_w1 (n : ℕ) : ℕ := (27 * n + 19) / 16
def F_w2 (n : ℕ) : ℕ := (27 * n + 23) / 32

def guard_Q1_256 (r : ℕ) (n : ℕ) : Prop := n % 256 = r ∧ 0 < n
def guard_Q2_1024 (r : ℕ) (n : ℕ) : Prop := n % 1024 = r ∧ 0 < n

-- Layer 2: Sound Refined Cylinder Image Partition Theorem (n = 7 mod 256 -> F1(n) = 13 mod 16)
theorem E12_partition_7_256 (n : ℕ) (h : guard_Q1_256 7 n) : F_w1 n % 16 = 13 := by
  dsimp [F_w1, guard_Q1_256] at *
  obtain ⟨h_mod, h_pos⟩ := h
  obtain ⟨k, rfl⟩ : ∃ k, n = 256 * k + 7 := by
    use n / 256
    omega
  have h_num : 27 * (256 * k + 7) + 19 = 6912 * k + 208 := by ring
  rw [h_num]
  have h_div : (6912 * k + 208) / 16 = 432 * k + 13 := by omega
  rw [h_div]
  omega

-- Sound Refined Cylinder Image Partition Theorem (n = 39 mod 256 -> F1(n) = 3 mod 16)
theorem E12_partition_39_256 (n : ℕ) (h : guard_Q1_256 39 n) : F_w1 n % 16 = 3 := by
  dsimp [F_w1, guard_Q1_256] at *
  obtain ⟨h_mod, h_pos⟩ := h
  obtain ⟨k, rfl⟩ : ∃ k, n = 256 * k + 39 := by
    use n / 256
    omega
  have h_num : 27 * (256 * k + 39) + 19 = 6912 * k + 1072 := by ring
  rw [h_num]
  have h_div : (6912 * k + 1072) / 16 = 432 * k + 67 := by omega
  rw [h_div]
  omega

-- Layer 3: Oddness Preservation Theorem for F1 on Q1 (n = 7 mod 32)
theorem E12_image_is_odd (n : ℕ) (h_mod : n % 32 = 7) : F_w1 n % 2 = 1 := by
  dsimp [F_w1]
  obtain ⟨k, rfl⟩ : ∃ k, n = 32 * k + 7 := by
    use n / 32
    omega
  have h_num : 27 * (32 * k + 7) + 19 = 864 * k + 208 := by ring
  rw [h_num]
  have h_div : (864 * k + 208) / 16 = 54 * k + 13 := by omega
  rw [h_div]
  omega

-- Layer 4: Non-Zero Fixed-Point Linear Forms & Positivity Proofs
def L1 (n : ℕ) : ℕ := 11 * n + 19
def L2 (n : ℕ) : ℤ := 5 * (n : ℤ) - 23

theorem L1_positive (n : ℕ) (h_pos : 0 < n) : 0 < L1 n := by
  dsimp [L1]
  have h1 : 0 < 11 * n := Nat.mul_pos (by decide) h_pos
  exact Nat.add_pos_left h1 19

theorem L2_positive_on_Q2 (n : ℕ) (h_ge : 43 ≤ n) : 0 < L2 n := by
  dsimp [L2]
  omega

end Milestone71
